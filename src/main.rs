use std::env;
use std::fs;
use std::process;

use fixture_next::{next_fixture, parse_fixtures, Date};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file_path: Option<String> = None;
    let mut team: Option<String> = None;
    let mut on: Option<String> = None;

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

    let file_path = file_path.unwrap_or_else(|| {
        eprintln!("missing required --file <path>");
        print_usage();
        process::exit(2);
    });
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

    let contents = fs::read_to_string(&file_path).unwrap_or_else(|e| {
        eprintln!("could not read {}: {}", file_path, e);
        process::exit(1);
    });

    let fixtures = parse_fixtures(&contents).unwrap_or_else(|e| {
        eprintln!("{}: {}", file_path, e);
        process::exit(1);
    });

    match next_fixture(&fixtures, &team, reference_date) {
        Some(fixture) => {
            print!("{} {} vs {}", fixture.date, fixture.home, fixture.away);
            if let Some(c) = &fixture.competition {
                print!(" ({})", c);
            }
            println!();
        }
        None => {
            println!("no upcoming fixture found for {}", team);
            process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("usage: fixture-next --file <fixtures.txt> --team <name> [--on YYYY-MM-DD]");
}
