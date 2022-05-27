use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
struct Twitter {
    tweets: Vec<(i32, i32)>,
    users: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Self::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut res = vec![];
        for &(u, t) in self.tweets.iter().rev() {
            if res.len() == 10 {
                break;
            }
            if u == user_id {
                res.push(t);
            } else if let Some(follows) = self.users.get(&user_id) {
                if follows.contains(&u) {
                    res.push(t);
                }
            }
        }
        res
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_insert_with(HashSet::new)
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_insert_with(HashSet::new)
            .remove(&followee_id);
    }
}

fn main() {
    let mut twt = Twitter::new();
    twt.post_tweet(1, 5);
    println!("{:?}", twt.get_news_feed(1));
    twt.follow(1, 2);
    twt.post_tweet(2, 6);
    println!("{:?}", twt.get_news_feed(1));
    twt.unfollow(1, 2);
    println!("{:?}", twt.get_news_feed(1));
}
