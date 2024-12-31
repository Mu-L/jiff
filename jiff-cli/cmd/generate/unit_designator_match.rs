/*!
A command for generating a prefix `match` expression for unit designator
labels.

This approach was devised as part of my work on
[benchmarking the task of parsing unit designator labels](https://github.com/BurntSushi/duration-unit-lookup).

Basically, a single `match` expression that recognizes a prefix of the
remaining input to parse ended up being roughly the fastest approach for a
number of different workloads.
*/

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use jiff::Unit;
use lexopt::{Arg, Parser};

use crate::args::{self, Usage};

const USAGE: &'static str = r#"
Generate Rust source code for a prefix `match` expression for unit designator
labels.

USAGE:
    jiff-cli generate unit-designator-match [<jiff-dir>]

For more context on what this `match` expression is doing and why we do it this
way, see: https://github.com/BurntSushi/duration-unit-lookup

This `match` expression is used as part of the "friendly" duration parser.

This program should be run from the root of the Jiff repository. Alternatively,
provide a path to the root of the Jiff repository as a positional argument.
"#;

pub fn run(p: &mut Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    args::configure(p, USAGE, &mut [&mut config])?;

    let jiff = config.jiff();
    let path = jiff.join("src/fmt/friendly/parser_label.rs");
    write_match(&path).with_context(|| {
        format!("failed to write `match` expression to {}", path.display())
    })?;

    Ok(())
}

#[derive(Debug)]
struct Config {
    jiff: Option<PathBuf>,
    verbose: bool,
}

impl Config {
    fn jiff(&self) -> &Path {
        self.jiff.as_deref().unwrap_or_else(|| Path::new("./"))
    }
}

impl Default for Config {
    fn default() -> Config {
        Config { jiff: None, verbose: false }
    }
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut Parser,
        arg: &mut Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            Arg::Short('v') | Arg::Long("verbose") => {
                self.verbose = true;
            }
            Arg::Value(ref mut value) => {
                if self.jiff.is_none() {
                    let path = PathBuf::from(std::mem::take(value));
                    self.jiff = Some(path);
                } else {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const USAGES: &'static [Usage] = &[Usage::new(
            "-v, --verbose",
            "Add more output.",
            r#"
This is a generic flag that expands output beyond the "normal" amount. Which
output is added depends on the command.
"#,
        )];
        USAGES
    }
}

fn write_match(path: &Path) -> anyhow::Result<()> {
    let mut out = BufWriter::new(File::create(path)?);

    let mut labels = LABELS.iter().copied().collect::<Vec<(&str, Unit)>>();
    labels.sort_by(|&(lab1, _), &(lab2, _)| {
        (lab1.len(), lab1).cmp(&(lab2.len(), lab2)).reverse()
    });

    writeln!(
        out,
        "// auto-generated by: jiff-cli generate unit-designator-match"
    )?;
    writeln!(out, "")?;

    writeln!(out, "use crate::Unit;")?;
    writeln!(out, "")?;
    writeln!(out, "#[inline(always)]")?;
    writeln!(
        out,
        "pub(super) fn find(haystack: &[u8]) -> Option<(Unit, usize)> {{"
    )?;
    writeln!(out, "  match haystack {{")?;
    for (label, unit) in labels {
        write!(out, "    &[")?;
        for &byte in label.as_bytes() {
            write!(out, "{}, ", ByteLiteral(byte))?;
        }
        writeln!(out, "..] => Some((Unit::{unit:?}, {})),", label.len())?;
    }
    writeln!(out, "  _ => None,")?;
    writeln!(out, "  }}")?;
    writeln!(out, "}}")?;

    out.flush()?;

    Ok(())
}

/// A helper type for formatting a byte literal in Rust source.
struct ByteLiteral(u8);

impl std::fmt::Display for ByteLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "b'")?;
        for escaped_byte in std::ascii::escape_default(self.0) {
            write!(f, "{}", escaped_byte as char)?;
        }
        write!(f, "'")?;
        Ok(())
    }
}

static LABELS: &[(&str, Unit)] = &[
    ("years", Unit::Year),
    ("year", Unit::Year),
    ("yrs", Unit::Year),
    ("yr", Unit::Year),
    ("y", Unit::Year),
    ("months", Unit::Month),
    ("month", Unit::Month),
    ("mos", Unit::Month),
    ("mo", Unit::Month),
    ("weeks", Unit::Week),
    ("week", Unit::Week),
    ("wks", Unit::Week),
    ("wk", Unit::Week),
    ("w", Unit::Week),
    ("days", Unit::Day),
    ("day", Unit::Day),
    ("d", Unit::Day),
    ("hours", Unit::Hour),
    ("hour", Unit::Hour),
    ("hrs", Unit::Hour),
    ("hr", Unit::Hour),
    ("h", Unit::Hour),
    ("minutes", Unit::Minute),
    ("minute", Unit::Minute),
    ("mins", Unit::Minute),
    ("min", Unit::Minute),
    ("m", Unit::Minute),
    ("seconds", Unit::Second),
    ("second", Unit::Second),
    ("secs", Unit::Second),
    ("sec", Unit::Second),
    ("s", Unit::Second),
    ("milliseconds", Unit::Millisecond),
    ("millisecond", Unit::Millisecond),
    ("millis", Unit::Millisecond),
    ("milli", Unit::Millisecond),
    ("msecs", Unit::Millisecond),
    ("msec", Unit::Millisecond),
    ("ms", Unit::Millisecond),
    ("microseconds", Unit::Microsecond),
    ("microsecond", Unit::Microsecond),
    ("micros", Unit::Microsecond),
    ("micro", Unit::Microsecond),
    ("usecs", Unit::Microsecond),
    ("usec", Unit::Microsecond),
    ("µsecs", Unit::Microsecond),
    ("µsec", Unit::Microsecond),
    ("us", Unit::Microsecond),
    ("µs", Unit::Microsecond),
    ("nanoseconds", Unit::Nanosecond),
    ("nanosecond", Unit::Nanosecond),
    ("nanos", Unit::Nanosecond),
    ("nano", Unit::Nanosecond),
    ("nsecs", Unit::Nanosecond),
    ("nsec", Unit::Nanosecond),
    ("ns", Unit::Nanosecond),
];