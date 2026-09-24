# fixture-next

I keep a season's fixtures in a text file and want one thing from it: when
does this team play next? Spreadsheets make that a five-click chore.
`fixture-next` answers that single question from the command line.

## Fixture file format

Plain text, one fixture per line, either pipe-delimited or CSV:

```
YYYY-MM-DD|Home Team|Away Team[|Competition[|Status]]
YYYY-MM-DD,Home Team,Away Team[,Competition[,Status]]
```

The delimiter is picked per line: a line containing `|` is read as
pipe-delimited, otherwise it's read as CSV. A CSV competition field can
contain a comma if it's wrapped in double quotes, e.g.
`"Premier League, rearranged"`. Blank lines and lines starting with `#` are
ignored, and the competition and status fields are optional. See
`data/fixtures.example.txt` and `data/fixtures.example.csv` for working
examples:

```
# Example fixture list for fixture-next.
2026-09-12|Arsenal|Chelsea|Premier League
2026-09-12|Leeds|Fulham|Premier League
2026-09-13|Arsenal U21|Chelsea U21|PL2
2026-09-05|Arsenal|Leeds|Premier League
2026-09-20|Arsenal|Newcastle|Premier League
2026-09-27|Everton|Arsenal|Premier League
```

Status is one of `scheduled` (the default when the field is left out or
empty), `postponed`, or `cancelled` (`ppd` and `canceled` are accepted as
aliases, and matching is case-insensitive). Postponed and cancelled fixtures
are skipped by both lookups below: a postponed game no longer has a
reliable date, and a cancelled one never happens. To give a status without
a competition, leave the competition field empty:

```
2026-09-12|Arsenal|Chelsea||postponed
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

## Reading from stdin

Omit `--file`, or pass `--file -`, to read the fixture list from stdin
instead of a file. This lets `fixture-next` compose with whatever produces
the list:

```
cat data/fixtures.example.txt | fixture-next --team Arsenal
```

## Building and testing

Standard `cargo build` / `cargo test`, no third-party dependencies. The
core logic lives in `src/lib.rs` and `src/date.rs`; `tests/table_tests.rs`
is a table-driven suite covering the fiddly cases: case-insensitive team
names, exact-match vs. substring ("Arsenal" vs. "Arsenal U21"), a team
playing away rather than at home, tied dates, leap-day fixtures, postponed
and cancelled fixtures, and malformed fixture lines.

## Status

Early skeleton. Next up: querying by date range instead of a single next
fixture.
