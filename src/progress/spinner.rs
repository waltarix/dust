use std::fmt;

use clap::{builder::PossibleValue, ValueEnum};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Spinner {
    Bar,
    Dots,
    Arc,
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Bar => "bar",
            Self::Dots => "dots",
            Self::Arc => "arc",
        })
    }
}

impl ValueEnum for Spinner {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Bar, Self::Dots, Self::Arc]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(self.to_string()))
    }
}

impl Spinner {
    pub fn chars(&self) -> &'static [char] {
        match self {
            Self::Bar => &[
                ' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃', '▂', '▁',
            ],
            Self::Dots => &['⠁', '⠉', '⠙', '⠸', '⢰', '⣠', '⣄', '⡆', '⠇', '⠃'],
            Self::Arc => &['', '', '', '', '', ''],
        }
    }
}
