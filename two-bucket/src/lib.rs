use std::collections::{HashSet, VecDeque};
use std::iter;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

struct State {
    buckets: (u8, u8),
    level: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let buckets = match start_bucket {
        Bucket::One => (capacity_1, 0),
        _ => (0, capacity_2),
    };

    // BFS approach implemented with FIFO queue & set
    //
    let mut queue: VecDeque<State> = iter::once(State { buckets, level: 1 }).collect();
    let mut set: HashSet<(u8, u8)> = vec![(capacity_1, 0), (0, capacity_2)].into_iter().collect();

    loop {
        let state = queue.pop_front()?;

        if check(state.buckets, goal) {
            return Some(result(state, goal));
        }

        for buckets in next_buckets(state.buckets, capacity_1, capacity_2) {
            if set.contains(&buckets) {
                continue;
            }

            queue.push_back(State {
                buckets,
                level: state.level + 1,
            });

            set.insert(buckets);
        }
    }
}

fn transfuse_right(a: u8, b: u8, cb: u8) -> (u8, u8) {
    if a + b > cb {
        (a + b - cb, cb)
    } else {
        (0, a + b)
    }
}

fn transfuse_left(a: u8, b: u8, ca: u8) -> (u8, u8) {
    let p = transfuse_right(a, b, ca);
    (p.1, p.0)
}

fn next_buckets(buckets: (u8, u8), ca: u8, cb: u8) -> Vec<(u8, u8)> {
    let (a, b) = buckets;

    vec![
        (0, b),
        (ca, b),
        (a, 0),
        (a, cb),
        transfuse_right(a, b, cb),
        transfuse_left(a, b, ca),
    ]
}

fn check(buckets: (u8, u8), goal: u8) -> bool {
    buckets.0 == goal || buckets.1 == goal
}

fn result(state: State, goal: u8) -> BucketStats {
    let (goal_bucket, other_bucket) = if state.buckets.0 == goal {
        (Bucket::One, state.buckets.1)
    } else {
        (Bucket::Two, state.buckets.0)
    };

    BucketStats {
        moves: state.level,
        goal_bucket,
        other_bucket,
    }
}
