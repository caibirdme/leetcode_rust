use std::collections::{HashMap, BTreeMap};
use std::cmp::Reverse;

struct Leaderboard {
    user_score: HashMap<i32, i32>,
    rank: BTreeMap<Reverse<i32>, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Leaderboard {

    fn new() -> Self {
        Self{
            user_score: HashMap::new(),
            rank: BTreeMap::new(),
        }
    }

    fn add_score(&mut self, player_id: i32, score: i32) {
        if score == 0 {
            return;
        }
        let mut before = None;
        if let Some(&v) = self.user_score.get(&player_id) {
            before = Some(v);
        }
        let p = self.user_score.entry(player_id).or_insert(0);
        *p += score;
        if let Some(v) = before {
            if let Some(t) = self.rank.get_mut(&Reverse(v)) {
                *t -= 1;
            }
        }
        *self.rank.entry(Reverse(*p)).or_insert(0) += 1;
    }

    fn top(&self, k: i32) -> i32 {
        let mut count = 0;
        let mut n = k;
        for (k,&v) in self.rank.iter() {
           if n >= v {
               count += k.0 * v;
               n -= v;
           } else {
               count += k.0 * n;
               n = 0;
           }
            if n == 0 {
                break;
            }
        }
        count
    }

    fn reset(&mut self, player_id: i32) {
        if let Some(v) = self.user_score.get_mut(&player_id) {
            let t = *v;
            *v = 0;
            if t > 0 {
                *self.rank.get_mut(&Reverse(t)).unwrap() -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Leaderboard;

    enum Op {
        Add(i32, i32),
        Reset(i32),
        Top(i32, i32),
    }

    #[test]
    fn test_leader_board() {
        use Op::*;
        let commands = vec![
            Add(1,73),
            Add(2,56),
            Add(3,39),
            Add(4,51),
            Add(5, 4),
            Top(1, 73),
            Reset(1),
            Reset(2),
            Top(2, 90),
            Add(2, 51),
            Top(3, 141),
            Reset(2),
            Reset(3),
            Reset(4),
            Top(100, 4),
        ];
        let mut lb = Leaderboard::new();
        for cmd in commands {
            match cmd {
                Add(play_id, score) => {
                    lb.add_score(play_id, score);
                },
                Reset(play_id) => {
                    lb.reset(play_id);
                },
                Top(k, expect) => {
                    assert_eq!(lb.top(k), expect);
                }
            }
        }
    }
}