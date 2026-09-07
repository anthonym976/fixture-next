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

/// Parses the pipe-delimited fixture format:
///
///   YYYY-MM-DD|Home Team|Away Team[|Competition]
///
/// Blank lines and lines starting with '#' are ignored. Whitespace around
/// each field is trimmed.
pub fn parse_fixtures(input: &str) -> Result<Vec<Fixture>, ParseError> {
    let mut fixtures = Vec::new();
    for (i, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> = line.split('|').map(|f| f.trim()).collect();
        if fields.len() < 3 || fields.len() > 4 {
            return Err(ParseError {
                line: i + 1,
                message: format!("expected 3 or 4 fields, got {}", fields.len()),
            });
        }

        let date = Date::parse(fields[0]).map_err(|message| ParseError {
            line: i + 1,
            message,
        })?;

        let home = fields[1].to_string();
        let away = fields[2].to_string();
        if home.is_empty() || away.is_empty() {
            return Err(ParseError {
                line: i + 1,
                message: "team name cannot be empty".to_string(),
            });
        }

        let competition = fields
            .get(3)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        fixtures.push(Fixture {
            date,
            home,
            away,
            competition,
        });
    }
    Ok(fixtures)
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
