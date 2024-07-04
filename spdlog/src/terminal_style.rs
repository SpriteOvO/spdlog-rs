//! Provides stuff related to terminal style.
//!
//! They are referenced from [ANSI escape code].
//!
//! [ANSI escape code]: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters

use std::io;

use crate::Level;

/// Text color for terminal rendering.
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
    // Gets foreground color terminal escape code.
    #[must_use]
    pub(crate) fn fg_code(&self) -> &'static str {
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

    // Gets background color terminal escape code.
    #[must_use]
    pub(crate) fn bg_code(&self) -> &'static str {
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

/// Expresses how to render a piece of text in the terminal.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Style {
    color: Option<Color>,
    bg_color: Option<Color>,
    bold: bool,
    faint: bool,
    italic: bool,
    underline: bool,
    slow_blink: bool,
    rapid_blink: bool,
    invert: bool,
    conceal: bool,
    strikethrough: bool,
    reset: bool,
}

impl Style {
    /// Constructs a `Style` with no styles.
    #[allow(clippy::new_without_default)]
    #[deprecated(
        since = "0.3.0",
        note = "it may be removed in the future, use `Style::builder()` instead"
    )]
    #[must_use]
    pub fn new() -> Style {
        Style::builder().build()
    }

    /// Gets a [`StyleBuilder`].
    #[must_use]
    pub fn builder() -> StyleBuilder {
        StyleBuilder {
            style: Style {
                color: None,
                bg_color: None,
                bold: false,
                faint: false,
                italic: false,
                underline: false,
                slow_blink: false,
                rapid_blink: false,
                invert: false,
                conceal: false,
                strikethrough: false,
                reset: false,
            },
        }
    }

    pub(crate) fn write_start(&self, dest: &mut impl io::Write) -> io::Result<()> {
        if self.reset {
            dest.write_all(Self::reset_code().as_bytes())?;
            return Ok(());
        }
        if let Some(color) = self.color {
            dest.write_all(color.fg_code().as_bytes())?;
        }
        if let Some(color) = self.bg_color {
            dest.write_all(color.bg_code().as_bytes())?;
        }
        if self.bold {
            dest.write_all("\x1b[1m".as_bytes())?;
        }
        if self.faint {
            dest.write_all("\x1b[2m".as_bytes())?;
        }
        if self.italic {
            dest.write_all("\x1b[3m".as_bytes())?;
        }
        if self.underline {
            dest.write_all("\x1b[4m".as_bytes())?;
        }
        if self.slow_blink {
            dest.write_all("\x1b[5m".as_bytes())?;
        }
        if self.rapid_blink {
            dest.write_all("\x1b[6m".as_bytes())?;
        }
        if self.invert {
            dest.write_all("\x1b[7m".as_bytes())?;
        }
        if self.conceal {
            dest.write_all("\x1b[8m".as_bytes())?;
        }
        if self.strikethrough {
            dest.write_all("\x1b[9m".as_bytes())?;
        }
        Ok(())
    }

    pub(crate) fn write_end(&self, dest: &mut impl io::Write) -> io::Result<()> {
        dest.write_all(Self::reset_code().as_bytes())
    }

    #[must_use]
    fn reset_code() -> &'static str {
        "\x1b[m"
    }
}

#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyleBuilder {
    style: Style,
}

pub(crate) mod macros {
    macro_rules! impl_style_builder_setters {
        ($builder_type:ident =>) => {};
        ($builder_type:ident => $visibility:vis $field_name:ident: Option<$field_type:ty>, $($tail:tt)*) => {
            #[allow(missing_docs)]
            $visibility fn $field_name(&mut self, $field_name: $field_type) -> &mut $builder_type {
                self.style.$field_name = Some($field_name);
                self
            }
            macros::impl_style_builder_setters! { $builder_type => $($tail)* }
        };
        ($builder_type:ident => $visibility:vis $field_name:ident: bool, $($tail:tt)*) => {
            #[allow(missing_docs)]
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
    #[allow(clippy::new_without_default)]
    #[deprecated(
        since = "0.3.0",
        note = "it may be removed in the future, use `Style::builder()` instead"
    )]
    #[must_use]
    pub fn new() -> StyleBuilder {
        Style::builder()
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
    #[must_use]
    pub fn build(&mut self) -> Style {
        self.style.clone()
    }
}

/// Represents terminal style enabling mode.
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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct LevelStyles([Style; Level::count()]);

impl LevelStyles {
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn style(&self, level: Level) -> &Style {
        &self.0[level as usize]
    }

    #[allow(dead_code)]
    pub(crate) fn set_style(&mut self, level: Level, style: Style) {
        self.0[level as usize] = style;
    }
}

impl Default for LevelStyles {
    fn default() -> LevelStyles {
        LevelStyles([
            Style::builder().bg_color(Color::Red).bold().build(), // Critical
            Style::builder().color(Color::Red).bold().build(),    // Error
            Style::builder().color(Color::Yellow).bold().build(), // Warn
            Style::builder().color(Color::Green).build(),         // Info
            Style::builder().color(Color::Cyan).build(),          // Debug
            Style::builder().color(Color::White).build(),         // Trace
        ])
    }
}
