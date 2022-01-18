use std::{
    collections::{hash_map::Entry, HashMap},
    env::{self, VarError},
    sync::RwLock,
};

use lazy_static::lazy_static;
use thiserror::Error;

use crate::LevelFilter;

lazy_static! {
    static ref ENV_LEVEL: RwLock<Option<EnvLevel>> = RwLock::new(None);
}

pub(crate) type EnvLevel = HashMap<EnvLevelLogger, LevelFilter>;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum EnvLevelLogger {
    Default,
    Named(String),
    Unnamed,
    AllExceptDefault,
}

/// The error type of environment level initialization.
#[derive(Error, Debug)]
pub enum EnvLevelError {
    /// Fetch environment variable error.
    #[error("fetch environment variable error: {0}")]
    FetchEnvVar(VarError),

    /// Parse environment variable error, usually caused by incorrect format.
    #[error("parse environment variable error: {0}")]
    ParseEnvVar(
        /// Parse error description
        String,
    ),
}

impl EnvLevelLogger {
    fn from_key(logger_name: &str) -> Self {
        if logger_name.is_empty() {
            EnvLevelLogger::Unnamed
        } else if logger_name == "*" {
            EnvLevelLogger::AllExceptDefault
        } else {
            EnvLevelLogger::Named(logger_name.into())
        }
    }

    fn from_logger(logger_name: Option<&str>) -> Self {
        match logger_name {
            None => Self::Unnamed,
            Some(name) => Self::Named(name.into()),
        }
    }
}

pub(crate) fn from_env(env_name: &str) -> Result<bool, EnvLevelError> {
    let var = match env::var(env_name) {
        Err(VarError::NotPresent) => return Ok(false),
        Err(err) => return Err(EnvLevelError::FetchEnvVar(err)),
        Ok(var) => var,
    };
    from_str(&var)?;
    Ok(true)
}

pub(crate) fn from_str(var: &str) -> Result<(), EnvLevelError> {
    let env_level = from_str_inner(var)?;
    *ENV_LEVEL.write().unwrap() = Some(env_level);
    Ok(())
}

pub(crate) fn from_str_inner(var: &str) -> Result<EnvLevel, EnvLevelError> {
    (|| {
        let mut env_level = EnvLevel::new();

        for kv_str in var.split(',').map(str::trim) {
            if kv_str.is_empty() {
                continue;
            }

            let mut kv = kv_str.split('=');
            let (left, right) = (kv.next().map(str::trim), kv.next().map(str::trim));

            let (logger, level) = match (left, right, kv.next()) {
                (Some(default_logger_level), None, None) => {
                    if let Some(level) = LevelFilter::from_str_for_env(default_logger_level) {
                        (EnvLevelLogger::Default, level)
                    } else {
                        return Err(format!(
                            "cannot parse level for default logger: '{}'",
                            kv_str
                        ));
                    }
                }
                (Some(logger_name), Some(level), None) => {
                    if let Some(level) = LevelFilter::from_str_for_env(level) {
                        (EnvLevelLogger::from_key(logger_name), level)
                    } else {
                        return Err(format!(
                            "cannot parse level for logger '{}': '{}'",
                            logger_name, kv_str
                        ));
                    }
                }
                _ => {
                    return Err(format!("invalid kv: '{}'", kv_str));
                }
            };

            match env_level.entry(logger) {
                Entry::Occupied(_) => {
                    return Err(format!("specified level multiple times: '{}'", kv_str));
                }
                Entry::Vacant(entry) => entry.insert(level),
            };
        }

        Ok(env_level)
    })()
    .map_err(EnvLevelError::ParseEnvVar)
}

pub(crate) fn logger_level(kind: LoggerKind) -> Option<LevelFilter> {
    logger_level_inner(ENV_LEVEL.read().unwrap().as_ref()?, kind)
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum LoggerKind<'a> {
    Default,
    Other(Option<&'a str>),
}

pub(crate) fn logger_level_inner(env_level: &EnvLevel, kind: LoggerKind) -> Option<LevelFilter> {
    let level = match kind {
        LoggerKind::Default => env_level.get(&EnvLevelLogger::Default)?,
        LoggerKind::Other(logger_name) => env_level
            .get(&EnvLevelLogger::from_logger(logger_name))
            .or_else(|| env_level.get(&EnvLevelLogger::AllExceptDefault))?,
    };
    Some(*level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Level;

    #[test]
    fn validation() {
        macro_rules! assert_levels {
            ($env_level:expr, DEFAULT => $default:expr, UNNAMED => $unnamed:expr, NAMED($name:literal) => $named:expr $(,)?) => {
                assert_eq!(
                    logger_level_inner(&$env_level, LoggerKind::Default),
                    $default
                );
                assert_eq!(
                    logger_level_inner(&$env_level, LoggerKind::Other(None)),
                    $unnamed
                );
                assert_eq!(
                    logger_level_inner(&$env_level, LoggerKind::Other(Some($name))),
                    $named
                );
            };
        }

        {
            let mut env_level = HashMap::new();
            env_level.insert(
                EnvLevelLogger::Default,
                LevelFilter::MoreSevereEqual(Level::Debug),
            );
            assert_eq!(from_str_inner("dEBUg").unwrap(), env_level);

            assert_levels!(
                env_level,
                DEFAULT => Some(LevelFilter::MoreSevereEqual(Level::Debug)),
                UNNAMED => None,
                NAMED("name") => None,
            );
        }

        {
            let mut env_level = HashMap::new();
            env_level.insert(EnvLevelLogger::Default, LevelFilter::All);
            env_level.insert(
                EnvLevelLogger::Unnamed,
                LevelFilter::MoreSevereEqual(Level::Info),
            );
            assert_eq!(from_str_inner("aLl,=inFo").unwrap(), env_level);

            assert_levels!(
                env_level,
                DEFAULT => Some(LevelFilter::All),
                UNNAMED => Some(LevelFilter::MoreSevereEqual(Level::Info)),
                NAMED("name") => None,
            );
        }

        {
            let mut env_level = HashMap::new();
            env_level.insert(EnvLevelLogger::Default, LevelFilter::Off);
            env_level.insert(
                EnvLevelLogger::Unnamed,
                LevelFilter::MoreSevereEqual(Level::Info),
            );
            env_level.insert(
                EnvLevelLogger::AllExceptDefault,
                LevelFilter::MoreSevereEqual(Level::Error),
            );
            assert_eq!(from_str_inner("oFf,=iNfo,*=erRor").unwrap(), env_level);

            assert_levels!(
                env_level,
                DEFAULT => Some(LevelFilter::Off),
                UNNAMED => Some(LevelFilter::MoreSevereEqual(Level::Info)),
                NAMED("name") => Some(LevelFilter::MoreSevereEqual(Level::Error)),
            );
        }

        {
            let mut env_level = HashMap::new();
            env_level.insert(
                EnvLevelLogger::Unnamed,
                LevelFilter::MoreSevereEqual(Level::Warn),
            );
            env_level.insert(
                EnvLevelLogger::Named("name".into()),
                LevelFilter::MoreSevereEqual(Level::Trace),
            );
            assert_eq!(from_str_inner("=wArn,name=trAce").unwrap(), env_level);

            assert_levels!(
                env_level,
                DEFAULT => None,
                UNNAMED => Some(LevelFilter::MoreSevereEqual(Level::Warn)),
                NAMED("name") => Some(LevelFilter::MoreSevereEqual(Level::Trace)),
            );
        }

        {
            let mut env_level = HashMap::new();
            env_level.insert(
                EnvLevelLogger::AllExceptDefault,
                LevelFilter::MoreSevereEqual(Level::Warn),
            );
            env_level.insert(
                EnvLevelLogger::Named("name".into()),
                LevelFilter::MoreSevereEqual(Level::Trace),
            );
            assert_eq!(from_str_inner("*=wArn,name=trAce").unwrap(), env_level);

            assert_levels!(
                env_level,
                DEFAULT => None,
                UNNAMED => Some(LevelFilter::MoreSevereEqual(Level::Warn)),
                NAMED("name") => Some(LevelFilter::MoreSevereEqual(Level::Trace)),
            );
        }

        {
            let mut env_level = HashMap::new();
            env_level.insert(EnvLevelLogger::Default, LevelFilter::All);
            env_level.insert(EnvLevelLogger::AllExceptDefault, LevelFilter::All);
            assert_eq!(from_str_inner("all,*=all").unwrap(), env_level);

            assert_levels!(
                env_level,
                DEFAULT => Some(LevelFilter::All),
                UNNAMED => Some(LevelFilter::All),
                NAMED("name") => Some(LevelFilter::All),
            );
        }

        {
            let mut env_level = HashMap::new();
            env_level.insert(EnvLevelLogger::Default, LevelFilter::Off);
            env_level.insert(EnvLevelLogger::AllExceptDefault, LevelFilter::All);
            assert_eq!(from_str_inner("off,*=all").unwrap(), env_level);

            assert_levels!(
                env_level,
                DEFAULT => Some(LevelFilter::Off),
                UNNAMED => Some(LevelFilter::All),
                NAMED("name") => Some(LevelFilter::All),
            );
        }
    }
}
