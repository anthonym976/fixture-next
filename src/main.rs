use std::env;
use std::fs;
use std::io::Read;
use std::process;

use fixture_next::{last_fixture, next_fixture, parse_fixtures, Date};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file_path: Option<String> = None;
    let mut team: Option<String> = None;
    let mut on: Option<String> = None;
    let mut last = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                file_path = args.get(i).cloned();
            }
            "--team" => {
                i += 1;
                team = args.get(i).cloned();
            }
            "--on" => {
                i += 1;
                on = args.get(i).cloned();
            }
            "--last" => {
                last = true;
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unrecognized argument: {}", other);
                print_usage();
                process::exit(2);
            }
        }
        i += 1;
    }

    let team = team.unwrap_or_else(|| {
        eprintln!("missing required --team <name>");
        print_usage();
        process::exit(2);
    });

    let reference_date = match on {
        Some(s) => Date::parse(&s).unwrap_or_else(|e| {
            eprintln!("invalid --on date: {}", e);
            process::exit(2);
        }),
        None => Date::today(),
    };

    // No --file, or "-", means read the fixture list from stdin instead of
    // a named file, so this composes with whatever produced the list.
    let source = file_path.as_deref().unwrap_or("-");
    let contents = if source == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
            eprintln!("could not read stdin: {}", e);
            process::exit(1);
        });
        buf
    } else {
        fs::read_to_string(source).unwrap_or_else(|e| {
            eprintln!("could not read {}: {}", source, e);
            process::exit(1);
        })
    };

    let fixtures = parse_fixtures(&contents).unwrap_or_else(|e| {
        eprintln!("{}: {}", source, e);
        process::exit(1);
    });

    let found = if last {
        last_fixture(&fixtures, &team, reference_date)
    } else {
        next_fixture(&fixtures, &team, reference_date)
    };

    match found {
        Some(fixture) => {
            print!("{} {} vs {}", fixture.date, fixture.home, fixture.away);
            if let Some(c) = &fixture.competition {
                print!(" ({})", c);
            }
            println!();
        }
        None => {
            let kind = if last { "past" } else { "upcoming" };
            println!("no {} fixture found for {}", kind, team);
            process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!(
        "usage: fixture-next [--file <fixtures.txt>|-] --team <name> [--on YYYY-MM-DD] [--last]\n\
         \n\
         If --file is omitted, or given as \"-\", fixtures are read from stdin."
    );
}
