use core::fmt;

use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, ValueEnum, Deserialize)]
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
