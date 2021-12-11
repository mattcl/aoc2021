use std::env::{self, VarError};
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct Solution<T, G> {
    part_one: T,
    part_two: G,
}

impl Default for Solution<&str, &str> {
    fn default() -> Self {
        Solution::new("not implemented", "not implemented")
    }
}

impl<T, G> Solution<T, G>
where
    T: Display + Serialize,
    G: Display + Serialize,
{
    pub fn new(part_one: T, part_two: G) -> Self {
        Self { part_one, part_two }
    }
}

impl<T, G> Display for Solution<T, G>
where
    T: Display + Serialize,
    G: Display + Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if env::var("AOC_OUTPUT_JSON").is_ok() {
            // So it's probably not idiomatic to panic here, but, I want this
            // specific behavior in this specific case. I would not do this in
            // code destined for production
            write!(
                f,
                "{}",
                serde_json::to_string(&self).expect("unable to convert self to json")
            )
        } else {
            write!(f, "part 1: {}\npart 2: {}", self.part_one, self.part_two)
        }
    }
}

impl<T, G> From<(T, G)> for Solution<T, G>
where
    T: Display + Serialize,
    G: Display + Serialize,
{
    fn from(value: (T, G)) -> Self {
        Self::new(value.0, value.1)
    }
}

/// Will attempt to load input from the specified AOC_INPUT file, otherwise
/// will default to loading the corresponding input file for the day given by
/// `default_day`.
///
/// ```no_run
/// use aoc::util::load_input;
/// let lines: Vec<String> = load_input("002").expect("could not load input");
/// ```
pub fn load_input(default_day: &str) -> Result<Vec<String>> {
    //
    // examples/003_toboggan-trajectory/input
    //
    load_external_input("AOC_INPUT").or_else(|e| {
        // If we errored because the var was not set, just return the
        // the default. Otherwise, we want to propagate the error because
        // it means that the var *was* set but we couldn't open/load the
        // file.
        if e.is::<VarError>() {
            load_named_input(default_day, "input")
        } else {
            Err(e)
        }
    })
}

pub fn load_named_input(day: &str, name: &str) -> Result<Vec<String>> {
    //
    // examples/003_toboggan-trajectory/<name>
    //
    let examples_dir = Path::new("examples");
    for entry in fs::read_dir(examples_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow!("Could not convert filename to string"))?
                .starts_with(day)
        {
            if let Some(file) = path.join(name).to_str() {
                return load_lines(file);
            }
        }
    }

    bail!(format!(
        "Could not find or load input for {}: '{}'",
        day, name
    ));
}

pub fn load_external_input(key: &str) -> Result<Vec<String>> {
    let path = env::var(key)?;
    load_lines(&path)
}

pub fn load_lines(file: &str) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    for line in BufReader::new(File::open(Path::new(file))?).lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub fn parse_input<T: FromStr>(
    input: &[String],
) -> std::result::Result<Vec<T>, <T as FromStr>::Err> {
    input.iter().map(|l| T::from_str(l)).collect()
}

pub fn test_input(input: &str) -> Vec<String> {
    // TODO: figure out if trim is the right thing to do - MCL - 2020-12-10
    input
        .trim()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_conversion() {
        let expected = vec![
            "abcd".to_string(),
            "".to_string(),
            "".to_string(),
            "efgh".to_string(),
            "ijkl".to_string(),
            "mnop".to_string(),
            "".to_string(),
            "qrs".to_string(),
        ];

        let input = "
            abcd


            efgh
            ijkl
            mnop

            qrs
        ";
        assert_eq!(test_input(input), expected);
    }
}
