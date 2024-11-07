//! Configuration for Prompter
//!
//! The configuration consists of a number of data sources, and then
//! the prompt configuration.  Data sources are computed and then
//! rendered into the prompt variables.  The prompt configuration is
//! computed once during `init` and otherwise has no bearing on what
//! computation is done to display the prompt.

// Ideally remove this once we have config files
#![allow(dead_code)]

use std::fmt;
use std::fmt::Write;

pub struct Config {
    pub left_prompt: Vec<PromptElement>,
    pub right_prompt: Vec<PromptElement>,
    pub sources: Vec<DataSource>,
}

pub enum PromptElement {
    Username,
    Lit(char),
    Literal(String),
    Hostname,
    Path,
    UserOrRoot,
    LastExit,
    Bold(Vec<PromptElement>),
    Underline(Vec<PromptElement>),
    Foreground(Colour, Vec<PromptElement>),
    Background(Colour, Vec<PromptElement>),
    Ternary(TernaryTest, Vec<PromptElement>, Vec<PromptElement>),
    PsVar(u8),
}

pub enum TernaryTest {
    PsVarSet(u8),
    ExitCode(u8),
}

pub enum Colour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub enum DataSource {
    Git(usize),
    Sudo(usize),
    Rust(usize),
    Key(usize),
    Flake(usize),
}

impl PromptElement {
    fn escape(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
        for c in s.chars() {
            match c {
                '%' | ')' => write!(f, "%{c}")?,
                _ => write!(f, "{c}")?,
            }
        }
        Ok(())
    }
    fn wrapped(
        f: &mut fmt::Formatter<'_>,
        first: impl AsRef<str>,
        second: impl AsRef<str>,
        rest: &[PromptElement],
    ) -> fmt::Result {
        let first = first.as_ref();
        let second = second.as_ref();
        write!(f, "{first}")?;
        for e in rest {
            write!(f, "{e}")?;
        }
        write!(f, "{second}")
    }
}

impl fmt::Display for PromptElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PromptElement::*;
        match self {
            Username => write!(f, "%n"),
            Lit(c) => write!(f, "{c}"),
            Literal(s) => Self::escape(f, s),
            Hostname => write!(f, "%m"),
            Path => write!(f, "%~"),
            UserOrRoot => write!(f, "%#"),
            LastExit => write!(f, "%?"),
            Bold(rest) => Self::wrapped(f, "%B", "%b", rest),
            Underline(rest) => Self::wrapped(f, "%U", "%u", rest),
            Foreground(c, rest) => Self::wrapped(f, format!("%F{{{c}}}"), "%f", rest),
            Background(c, rest) => Self::wrapped(f, format!("%K{{{c}}}"), "%k", rest),
            Ternary(test, truthy, falsey) => {
                write!(f, "%({test}.")?;
                for e in truthy {
                    e.fmt(f)?;
                }
                write!(f, ".")?;
                for e in falsey {
                    e.fmt(f)?;
                }
                write!(f, ")")
            }
            PsVar(n) => write!(f, "%{n}v"),
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Colour::*;
        match self {
            Black => write!(f, "black"),
            Red => write!(f, "red"),
            Green => write!(f, "green"),
            Yellow => write!(f, "yellow"),
            Blue => write!(f, "blue"),
            Magenta => write!(f, "magenta"),
            Cyan => write!(f, "cyan"),
            White => write!(f, "white"),
        }
    }
}

impl fmt::Display for TernaryTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TernaryTest::*;
        match self {
            PsVarSet(n) => write!(f, "{n}V"),
            ExitCode(n) => write!(f, "{n}?"),
        }
    }
}

// Default prompt uses the following vars:
// 1 - the active VCS
// 2 - The path to the base of the VCS dir
// 3 - The branch in VCS
// 4 - The path within the VCS dir
// 5 - Set if we can sudo
// 6 - Labels (eg. Rust)
// 7 - Can SSH (key present)
// 8 - Flake present emoji

impl Default for Config {
    fn default() -> Self {
        use Colour::*;
        use DataSource::*;
        use PromptElement::*;
        use TernaryTest::*;
        Self {
            left_prompt: vec![
                Ternary(
                    PsVarSet(5),
                    vec![Foreground(Red, vec![Hostname])],
                    vec![Hostname],
                ),
                Ternary(
                    PsVarSet(1),
                    vec![Foreground(
                        Blue,
                        vec![Literal("(".into()), PsVar(1), Literal(")".into())],
                    )],
                    vec![],
                ),
                Ternary(
                    PsVarSet(6),
                    vec![Literal("(".into()), PsVar(6), Literal(")".into())],
                    vec![],
                ),
                UserOrRoot,
                Literal(" ".into()),
            ],
            right_prompt: vec![Ternary(
                PsVarSet(1),
                vec![
                    PsVar(2),
                    Foreground(
                        Yellow,
                        vec![
                            Lit('['),
                            Ternary(PsVarSet(8), vec![PsVar(8), Lit(' ')], vec![]),
                            PsVar(3),
                            Lit(']'),
                        ],
                    ),
                    PsVar(4),
                ],
                vec![Path],
            )],
            sources: vec![Git(1), Sudo(5), Rust(6), Key(7), Flake(8)],
        }
    }
}

impl Config {
    pub fn left_prompt(&self) -> String {
        let mut ret = String::new();
        for e in &self.left_prompt {
            write!(ret, "{e}").unwrap();
        }
        ret
    }
    pub fn right_prompt(&self) -> String {
        let mut ret = String::new();
        for e in &self.right_prompt {
            write!(ret, "{e}").unwrap();
        }
        ret
    }

    pub fn render(&self) -> Vec<String> {
        let mut psvars = Vec::new();
        for src in &self.sources {
            src.fill_vars(&mut psvars);
        }
        psvars
    }
}
