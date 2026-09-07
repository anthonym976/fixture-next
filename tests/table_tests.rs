use fixture_next::{next_fixture, parse_fixtures, Date};

const SAMPLE: &str = "\
# 2026 fixtures used by the next_fixture table tests
2026-09-12|Arsenal|Chelsea|Premier League
2026-09-12|Leeds|Fulham|Premier League

2026-09-13|Arsenal U21|Chelsea U21|PL2
2026-09-05|Arsenal|Leeds|Premier League
2026-09-20|arsenal|Newcastle|Premier League
2026-09-20|Everton|arsenal|Premier League
";

struct Case {
    name: &'static str,
    team: &'static str,
    on: &'static str,
    want: Option<(&'static str, &'static str, &'static str)>,
}

#[test]
fn next_fixture_table() {
    let fixtures = parse_fixtures(SAMPLE).expect("sample fixtures should parse");

    let cases = [
        Case {
            name: "returns the earliest upcoming fixture for the team",
            team: "Arsenal",
            on: "2026-09-01",
            want: Some(("2026-09-05", "Arsenal", "Leeds")),
        },
        Case {
            name: "team matching is case-insensitive",
            team: "ARSENAL",
            on: "2026-09-06",
            want: Some(("2026-09-12", "Arsenal", "Chelsea")),
        },
        Case {
            name: "the reference date is inclusive of a same-day fixture",
            team: "Arsenal",
            on: "2026-09-12",
            want: Some(("2026-09-12", "Arsenal", "Chelsea")),
        },
        Case {
            name: "a team is found when it plays away",
            team: "Newcastle",
            on: "2026-09-01",
            want: Some(("2026-09-20", "arsenal", "Newcastle")),
        },
        Case {
            name: "team names must match in full, not as a substring",
            team: "Arsenal U21",
            on: "2026-09-01",
            want: Some(("2026-09-13", "Arsenal U21", "Chelsea U21")),
        },
        Case {
            name: "no fixture on or after the reference date yields none",
            team: "Arsenal",
            on: "2026-09-21",
            want: None,
        },
        Case {
            name: "a team that never appears yields none",
            team: "Sunderland",
            on: "2026-01-01",
            want: None,
        },
        Case {
            name: "surrounding whitespace on the query team is ignored",
            team: "  Leeds  ",
            on: "2026-09-01",
            want: Some(("2026-09-05", "Arsenal", "Leeds")),
        },
        Case {
            name: "same-day fixtures for other teams do not interfere",
            team: "Fulham",
            on: "2026-09-01",
            want: Some(("2026-09-12", "Leeds", "Fulham")),
        },
    ];

    for case in cases {
        let on = Date::parse(case.on).expect("case reference date should parse");
        let got = next_fixture(&fixtures, case.team, on)
            .map(|f| (f.date.to_string(), f.home.clone(), f.away.clone()));
        let want = case
            .want
            .map(|(d, h, a)| (d.to_string(), h.to_string(), a.to_string()));
        assert_eq!(got, want, "case '{}' failed", case.name);
    }
}

#[test]
fn next_fixture_breaks_ties_by_file_order() {
    let data = "\
2026-09-12|Arsenal|Chelsea|League Cup
2026-09-12|Arsenal|Chelsea|Community Shield
";
    let fixtures = parse_fixtures(data).expect("fixtures should parse");
    let on = Date::parse("2026-09-01").unwrap();

    let got = next_fixture(&fixtures, "Arsenal", on).expect("should find a fixture");
    assert_eq!(got.competition.as_deref(), Some("League Cup"));
}

struct ParseCase {
    name: &'static str,
    input: &'static str,
    want_ok: bool,
    want_count: usize,
}

#[test]
fn parse_fixtures_table() {
    let cases = [
        ParseCase {
            name: "blank lines and comments are skipped",
            input: "\n# comment\n2026-09-12|Arsenal|Chelsea\n\n",
            want_ok: true,
            want_count: 1,
        },
        ParseCase {
            name: "the competition field is optional",
            input: "2026-09-12|Arsenal|Chelsea",
            want_ok: true,
            want_count: 1,
        },
        ParseCase {
            name: "empty input yields zero fixtures",
            input: "",
            want_ok: true,
            want_count: 0,
        },
        ParseCase {
            name: "too few fields is an error",
            input: "2026-09-12|Arsenal",
            want_ok: false,
            want_count: 0,
        },
        ParseCase {
            name: "too many fields is an error",
            input: "2026-09-12|Arsenal|Chelsea|League|Extra",
            want_ok: false,
            want_count: 0,
        },
        ParseCase {
            name: "an invalid month is rejected",
            input: "2026-13-01|Arsenal|Chelsea",
            want_ok: false,
            want_count: 0,
        },
        ParseCase {
            name: "february 29 in a non-leap year is rejected",
            input: "2026-02-29|Arsenal|Chelsea",
            want_ok: false,
            want_count: 0,
        },
        ParseCase {
            name: "february 29 in a leap year is accepted",
            input: "2028-02-29|Arsenal|Chelsea",
            want_ok: true,
            want_count: 1,
        },
        ParseCase {
            name: "an empty team name is rejected",
            input: "2026-09-12||Chelsea",
            want_ok: false,
            want_count: 0,
        },
    ];

    for case in cases {
        let result = parse_fixtures(case.input);
        assert_eq!(
            result.is_ok(),
            case.want_ok,
            "case '{}': unexpected ok-ness",
            case.name
        );
        if let Ok(fixtures) = result {
            assert_eq!(
                fixtures.len(),
                case.want_count,
                "case '{}': unexpected fixture count",
                case.name
            );
        }
    }
}
