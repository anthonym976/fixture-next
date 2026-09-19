mod date;

pub use date::Date;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    pub date: Date,
    pub home: String,
    pub away: String,
    pub competition: Option<String>,
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parses either the pipe-delimited or CSV fixture format:
///
///   YYYY-MM-DD|Home Team|Away Team[|Competition]
///   YYYY-MM-DD,Home Team,Away Team[,Competition]
///
/// The delimiter is sniffed per line: a line containing '|' is treated as
/// pipe-delimited, otherwise it's parsed as CSV (with support for
/// double-quoted fields, so a competition name can contain a comma). Blank
/// lines and lines starting with '#' are ignored. Whitespace around each
/// unquoted field is trimmed.
pub fn parse_fixtures(input: &str) -> Result<Vec<Fixture>, ParseError> {
    let mut fixtures = Vec::new();
    for (i, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields = split_fields(line);
        if fields.len() < 3 || fields.len() > 4 {
            return Err(ParseError {
                line: i + 1,
                message: format!("expected 3 or 4 fields, got {}", fields.len()),
            });
        }

        let date = Date::parse(&fields[0]).map_err(|message| ParseError {
            line: i + 1,
            message,
        })?;

        let home = fields[1].clone();
        let away = fields[2].clone();
        if home.is_empty() || away.is_empty() {
            return Err(ParseError {
                line: i + 1,
                message: "team name cannot be empty".to_string(),
            });
        }

        let competition = fields.get(3).filter(|s| !s.is_empty()).cloned();

        fixtures.push(Fixture {
            date,
            home,
            away,
            competition,
        });
    }
    Ok(fixtures)
}

/// Splits one line into fields, picking the delimiter based on content: '|'
/// wins if present, since a fixture line legitimately using CSV would only
/// ever contain a pipe inside a competition name by accident.
fn split_fields(line: &str) -> Vec<String> {
    if line.contains('|') {
        line.split('|').map(|f| f.trim().to_string()).collect()
    } else {
        parse_csv_line(line)
    }
}

/// Splits a single CSV line into fields. A field starting with `"` runs
/// until the next unescaped `"`, so commas and even newlines-within-a-field
/// aren't possible here since we operate one line at a time, but embedded
/// commas are; a doubled quote (`""`) inside a quoted field is an escaped
/// literal quote, matching the common CSV convention (RFC 4180).
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(c);
            }
        } else {
            match c {
                '"' if field.trim().is_empty() => {
                    in_quotes = true;
                    field.clear();
                }
                ',' => {
                    fields.push(field.trim().to_string());
                    field.clear();
                }
                _ => field.push(c),
            }
        }
    }
    fields.push(field.trim().to_string());
    fields
}

fn team_matches(fixture: &Fixture, team: &str) -> bool {
    fixture.home.eq_ignore_ascii_case(team) || fixture.away.eq_ignore_ascii_case(team)
}

/// Finds the earliest fixture involving `team` on or after `on_or_after`.
///
/// Team names are matched case-insensitively but must match in full, so
/// "Arsenal" never matches a fixture listing "Arsenal U21". When two
/// qualifying fixtures share the same date, the one appearing first in
/// `fixtures` is returned.
pub fn next_fixture<'a>(
    fixtures: &'a [Fixture],
    team: &str,
    on_or_after: Date,
) -> Option<&'a Fixture> {
    let team = team.trim();
    fixtures
        .iter()
        .filter(|f| f.date >= on_or_after && team_matches(f, team))
        .fold(None, |best: Option<&Fixture>, candidate| match best {
            Some(b) if b.date <= candidate.date => Some(b),
            _ => Some(candidate),
        })
}

/// Finds the most recent fixture involving `team` strictly before `before`.
///
/// A fixture dated `before` itself is not considered past yet (it has not
/// necessarily kicked off), so the comparison is exclusive on that end;
/// this is the mirror image of `next_fixture`'s inclusive lower bound. Ties
/// on the same date are broken the same way as `next_fixture`: the fixture
/// appearing first in `fixtures` wins.
pub fn last_fixture<'a>(fixtures: &'a [Fixture], team: &str, before: Date) -> Option<&'a Fixture> {
    let team = team.trim();
    fixtures
        .iter()
        .filter(|f| f.date < before && team_matches(f, team))
        .fold(None, |best: Option<&Fixture>, candidate| match best {
            Some(b) if b.date >= candidate.date => Some(b),
            _ => Some(candidate),
        })
}
