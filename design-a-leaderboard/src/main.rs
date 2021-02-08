use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Leaderboard {
    players: HashMap<i32, i32>,
    scores: BTreeMap<i32, usize>,
}

impl Leaderboard {
    fn new() -> Self {
        Leaderboard::default()
    }

    fn add_score(&mut self, player_id: i32, score: i32) {
        let player_score = self.players.entry(player_id).or_default();
        if *player_score != 0 {
            let count = self.scores.entry(*player_score).or_default();
            *count -= 1;
            if *count == 0 {
                self.scores.remove(player_score);
            }
        }
        *player_score += score;
        *self.scores.entry(*player_score).or_default() += 1;
    }

    fn top(&mut self, mut k: i32) -> i32 {
        let mut sum = 0;
        for (s, &v) in self.scores.iter().rev() {
            if k < v as i32 {
                sum = sum + s * k;
                break;
            } else {
                sum = sum + s * v as i32;
                k -= v as i32;
            }
        }
        sum
    }

    fn reset(&mut self, player_id: i32) {
        let player_score = self.players.entry(player_id).or_default();
        let count = self.scores.entry(*player_score).or_default();
        *count -= 1;
        if *count == 0 {
            self.scores.remove(player_score);
        }
        *player_score = 0;
    }
}

fn main() {
    let mut obj = Leaderboard::new();
    obj.add_score(1, 73);
    obj.add_score(2, 56);
    obj.add_score(3, 39);
    obj.add_score(4, 51);
    obj.add_score(5, 4);
    assert_eq!(obj.top(1), 73);
    obj.reset(1);
    obj.reset(2);
    obj.add_score(2, 51);
    assert_eq!(obj.top(3), 141);
}
