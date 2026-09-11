# fixture-next

I keep a season's fixtures in a text file and want one thing from it: when
does this team play next? Spreadsheets make that a five-click chore.
`fixture-next` answers that single question from the command line.

## Fixture file format

Plain text, one fixture per line, pipe-delimited:

```
YYYY-MM-DD|Home Team|Away Team[|Competition]
```

Blank lines and lines starting with `#` are ignored. The competition field
is optional. See `data/fixtures.example.txt` for a working example:

```
# Example fixture list for fixture-next.
2026-09-12|Arsenal|Chelsea|Premier League
2026-09-12|Leeds|Fulham|Premier League
2026-09-13|Arsenal U21|Chelsea U21|PL2
2026-09-05|Arsenal|Leeds|Premier League
2026-09-20|Arsenal|Newcastle|Premier League
2026-09-27|Everton|Arsenal|Premier League
```

## Usage

```
fixture-next --file data/fixtures.example.txt --team Arsenal
```

```
2026-09-12 Arsenal vs Chelsea (Premier League)
```

Pin the "as of" date instead of using today, to ask what a team's next
fixture would have been at a point in the past:

```
fixture-next --file data/fixtures.example.txt --team Arsenal --on 2026-09-01
```

```
2026-09-05 Arsenal vs Leeds (Premier League)
```

Team matching is case-insensitive but exact, so `--team Arsenal` never
matches an "Arsenal U21" row. When a team has two fixtures on the same
date, the one listed first in the file wins.

If there's no upcoming fixture for the team, the tool says so and exits
with a non-zero status:

```
$ fixture-next --file data/fixtures.example.txt --team Sunderland
no upcoming fixture found for Sunderland
```

Pass `--last` to look backwards instead: the most recent fixture strictly
before the reference date. A fixture dated exactly on the reference date
doesn't count as past yet, since it may not have kicked off:

```
fixture-next --file data/fixtures.example.txt --team Arsenal --on 2026-09-13 --last
```

```
2026-09-12 Arsenal vs Chelsea (Premier League)
```

## Building and testing

Standard `cargo build` / `cargo test`, no third-party dependencies. The
core logic lives in `src/lib.rs` and `src/date.rs`; `tests/table_tests.rs`
is a table-driven suite covering the fiddly cases: case-insensitive team
names, exact-match vs. substring ("Arsenal" vs. "Arsenal U21"), a team
playing away rather than at home, tied dates, leap-day fixtures, and
malformed fixture lines.

## Status

Early skeleton. Next up: reading fixtures from stdin so this composes
with other tools.
