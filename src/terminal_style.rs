//! Provides stuff related to terminal style.
//!
//! They are referenced from [ANSI escape code].
//!
//! [ANSI escape code]: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters

use getset::{CopyGetters, Setters};

use crate::Level;

/// The terminal text color style.
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    /// Gets foreground color terminal escape code.
    pub fn to_fg_code(&self) -> &'static str {
        match self {
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
        }
    }

    /// Gets background color terminal escape code.
    pub fn to_bg_code(&self) -> &'static str {
        match self {
            Color::Black => "\x1b[40m",
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Yellow => "\x1b[43m",
            Color::Blue => "\x1b[44m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
        }
    }
}

/// The terminal text style structure.
///
/// You can construct it easily with [`StyleBuilder`].
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, CopyGetters, Setters)]
#[getset(get_copy = "pub", set = "pub")]
pub struct Style {
    ///
    color: Option<Color>,
    ///
    bg_color: Option<Color>,
    ///
    bold: bool,
    ///
    faint: bool,
    ///
    italic: bool,
    ///
    underline: bool,
    ///
    slow_blink: bool,
    ///
    rapid_blink: bool,
    ///
    invert: bool,
    ///
    conceal: bool,
    ///
    strikethrough: bool,
    ///
    reset: bool,
}

impl Style {
    /// Constructs a `Style` with no styles.
    pub fn new() -> Style {
        Style::default()
    }

    /// Constructs a [`StyleBuilder`].
    pub fn builder() -> StyleBuilder {
        StyleBuilder::new()
    }

    /// Gets the escape code for rendering style text.
    pub fn render_code(&self) -> StyleCode {
        if self.reset {
            return StyleCode {
                start: Style::reset_code(),
                end: Style::reset_code(),
            };
        }

        let mut res = String::new();

        macro_rules! push_escape_code {
            () => {};
            ($field_name:ident: Option => $code:expr, $($tail:tt)*) => {
                if let Some($field_name) = self.$field_name {
                    res.push_str($code);
                }
                push_escape_code! { $($tail)* }
            };
            ($field_name:ident: bool => $code:expr, $($tail:tt)*) => {
                if self.$field_name {
                    res.push_str($code);
                }
                push_escape_code! { $($tail)* }
            };
        }

        push_escape_code! {
            color: Option => color.to_fg_code(),
            bg_color: Option => bg_color.to_bg_code(),
            bold: bool => "\x1b[1m",
            faint: bool => "\x1b[2m",
            italic: bool => "\x1b[3m",
            underline: bool => "\x1b[4m",
            slow_blink: bool => "\x1b[5m",
            rapid_blink: bool => "\x1b[6m",
            invert: bool => "\x1b[7m",
            conceal: bool => "\x1b[8m",
            strikethrough: bool => "\x1b[9m",
        }

        StyleCode {
            start: res,
            end: Style::reset_code(),
        }
    }

    fn reset_code() -> String {
        "\x1b[m".to_string()
    }
}

/// The builder of [`Style`].
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct StyleBuilder {
    style: Style,
}

pub(crate) mod macros {
    macro_rules! impl_style_builder_setters {
        ($builder_type:ident =>) => {};
        ($builder_type:ident => $visibility:vis $field_name:ident: Option<$field_type:ty>, $($tail:tt)*) => {
            #[allow(missing_docs)]
            #[must_use]
            $visibility fn $field_name(&mut self, $field_name: $field_type) -> &mut $builder_type {
                self.style.$field_name = Some($field_name);
                self
            }
            macros::impl_style_builder_setters! { $builder_type => $($tail)* }
        };
        ($builder_type:ident => $visibility:vis $field_name:ident: bool, $($tail:tt)*) => {
            #[allow(missing_docs)]
            #[must_use]
            $visibility fn $field_name(&mut self) -> &mut $builder_type {
                self.style.$field_name = true;
                self
            }
            macros::impl_style_builder_setters! { $builder_type => $($tail)* }
        };
    }
    pub(crate) use impl_style_builder_setters;
}

impl StyleBuilder {
    /// Constructs a `StyleBuilder`.
    pub fn new() -> StyleBuilder {
        StyleBuilder::default()
    }

    macros::impl_style_builder_setters! {
        StyleBuilder =>
        pub reset: bool,
        pub color: Option<Color>,
        pub bg_color: Option<Color>,
        pub bold: bool,
        pub faint: bool,
        pub italic: bool,
        pub underline: bool,
        pub slow_blink: bool,
        pub rapid_blink: bool,
        pub invert: bool,
        pub conceal: bool,
        pub strikethrough: bool,
    }

    /// Builds a [`Style`].
    pub fn build(&mut self) -> Style {
        self.style.clone()
    }
}

/// Represents styles of all log levels.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LevelStyles([Style; Level::count()]);

impl LevelStyles {
    /// Gets the style for the given level.
    pub fn style(&self, level: Level) -> &Style {
        &self.0[level as usize]
    }

    /// Sets the style for the given level.
    pub fn set_style(&mut self, level: Level, style: Style) {
        self.0[level as usize] = style;
    }
}

impl From<LevelStyles> for LevelStyleCodes {
    fn from(level_styles: LevelStyles) -> LevelStyleCodes {
        LevelStyleCodes(level_styles.0.map(|style| style.into()))
    }
}

impl Default for LevelStyles {
    fn default() -> LevelStyles {
        LevelStyles([
            StyleBuilder::new().bg_color(Color::Red).bold().build(), // Critical
            StyleBuilder::new().color(Color::Red).bold().build(),    // Error
            StyleBuilder::new().color(Color::Yellow).bold().build(), // Warn
            StyleBuilder::new().color(Color::Green).build(),         // Info
            StyleBuilder::new().color(Color::Cyan).build(),          // Debug
            StyleBuilder::new().color(Color::White).build(),         // Trace
        ])
    }
}

/// Represents the start escape code and the end escape code of a style.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyleCode {
    /// The start escape code for rendering style text.
    pub start: String,
    /// The end escape code for rendering style text.
    pub end: String,
}

/// Represents style codes of all log levels.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LevelStyleCodes([StyleCode; Level::count()]);

impl LevelStyleCodes {
    /// Gets the code for the given level.
    pub fn code(&self, level: Level) -> &StyleCode {
        &self.0[level as usize]
    }

    /// Sets the code for the given level.
    pub fn set_code<C>(&mut self, level: Level, code: C)
    where
        C: Into<StyleCode>,
    {
        self.0[level as usize] = code.into();
    }
}

impl From<Style> for StyleCode {
    fn from(style: Style) -> StyleCode {
        style.render_code()
    }
}

impl Default for LevelStyleCodes {
    fn default() -> LevelStyleCodes {
        LevelStyles::default().into()
    }
}

/// Represents style enable mode.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StyleMode {
    /// Always output style escape codes.
    Always,
    /// Output style escape codes only when the target is detected as a
    /// terminal.
    Auto,
    /// Always do not output style escape codes.
    Never,
}
