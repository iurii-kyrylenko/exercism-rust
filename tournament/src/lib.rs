use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut report = Report::new();

    let teams = match_results
        .lines()
        .map(TeamEntry::build_two)
        .flatten()
        .for_each(|team| report.add_team(&team));

    unimplemented!("===== {:#?}", report.teams);
}

struct Report {
    teams: HashMap<String, TeamStats>,
}

impl Report {
    fn new() -> Self {
        Self {
            teams: HashMap::new(),
        }
    }

    fn add_team(&mut self, team: &TeamEntry) {
        let entry = self.teams.entry(team.team.clone());

        let stats = entry.or_insert(TeamStats {
            matches: 0,
            wins: 0,
            drafts: 0,
            losses: 0,
            points: 0,
        });

        stats.matches += 1;
        // ...
    }
}

#[derive(Debug)]
struct TeamStats {
    matches: i32,
    wins: i32,
    drafts: i32,
    losses: i32,
    points: i32,
}

#[derive(Clone, Copy)]
enum MatchResult {
    Win,
    Loss,
    Draw,
}

struct TeamEntry {
    team: String,
    result: MatchResult,
}

struct MatchEntry {
    team1: String,
    team2: String,
    result: MatchResult,
}

impl MatchResult {
    fn new(s: &str) -> Self {
        match s {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            _ => MatchResult::Draw,
        }
    }

    fn compliment(&self) -> Self {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            _ => MatchResult::Draw,
        }
    }
}

impl TeamEntry {
    fn build_two(s: &str) -> Vec<Self> {
        let match_entry = MatchEntry::new(s);

        vec![
            Self {
                team: match_entry.team1,
                result: match_entry.result,
            },
            Self {
                team: match_entry.team2,
                result: match_entry.result.compliment(),
            },
        ]
    }
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
