use std::collections::BTreeMap;
use std::fmt;
use MatchResult::*;

pub fn tally(match_results: &str) -> String {
    let mut report = Report::new();

    match_results
        .lines()
        .map(TeamEntry::build_two)
        .flatten()
        .for_each(|team| report.add_team(&team));

    report.to_string()
}

struct Report {
    teams: BTreeMap<String, TeamStats>,
}

impl Report {
    fn new() -> Self {
        Self {
            teams: BTreeMap::new(),
        }
    }

    fn add_team(&mut self, team: &TeamEntry) {
        self.teams
            .entry(team.name.clone())
            .or_insert(TeamStats::default())
            .update(&team.result);
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        macro_rules! FMT {
            () => {
                "{:30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}"
            };
        }

        write!(f, FMT!(), "Team", "MP", "W", "D", "L", "P")?;

        let mut show: Vec<(&String, &TeamStats)> = self.teams.iter().collect();

        show.sort_by_key(|k| -k.1.points);

        for (name, s) in show {
            writeln!(f)?;
            write!(f, FMT!(), name, s.mp, s.wins, s.draws, s.losses, s.points)?;
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
struct TeamStats {
    mp: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points: i32,
}

impl TeamStats {
    fn update(&mut self, result: &MatchResult) {
        self.mp += 1;

        match result {
            Win => {
                self.wins += 1;
                self.points += 3;
            }
            Draw => {
                self.draws += 1;
                self.points += 1;
            }
            _ => {
                self.losses += 1;
            }
        }
    }
}

#[derive(Clone, Copy)]
enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    fn new(s: &str) -> Self {
        match s {
            "win" => Win,
            "loss" => Loss,
            _ => Draw,
        }
    }

    fn negate(&self) -> Self {
        match self {
            Win => Loss,
            Loss => Win,
            _ => Draw,
        }
    }
}

struct TeamEntry {
    name: String,
    result: MatchResult,
}

impl TeamEntry {
    fn build_two(s: &str) -> Vec<Self> {
        let MatchEntry {
            team1,
            team2,
            result,
        } = MatchEntry::new(s);

        vec![
            Self {
                name: team1,
                result,
            },
            Self {
                name: team2,
                result: result.negate(),
            },
        ]
    }
}

struct MatchEntry {
    team1: String,
    team2: String,
    result: MatchResult,
}

impl MatchEntry {
    fn new(s: &str) -> Self {
        let v: Vec<&str> = s.split(';').collect();

        Self {
            team1: v[0].to_string(),
            team2: v[1].to_string(),
            result: MatchResult::new(v[2]),
        }
    }
}
