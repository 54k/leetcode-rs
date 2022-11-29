use std::collections::{HashMap, HashSet};

static mut TWEET_SEQ: usize = 0;
const HISTORY_SIZE: usize = 10;

type Tweet = Option<Box<Node>>;

struct User {
    id: i32,
    followers: HashSet<i32>,
    head: Tweet,
}

impl User {
    fn new(id: i32) -> Self {
        Self {
            id,
            followers: HashSet::new(),
            head: None,
        }
    }

    fn push_tweet(&mut self, tweet_id: i32) {
        self.head = Some(Box::new(Node {
            seq: unsafe {
                TWEET_SEQ += 1;
                TWEET_SEQ
            },
            id: tweet_id,
            next: self.head.take(),
        }));
    }

    fn get_tweets(&self) -> Vec<(usize, i32)> {
        let mut tweets = vec![];

        let mut prev = self.head.as_ref();
        for _ in 0..HISTORY_SIZE {
            if let Some(t) = prev {
                tweets.push((t.seq, t.id));
                prev = t.next.as_ref();
            } else {
                break;
            }
        }

        tweets
    }
}

struct Node {
    seq: usize,
    id: i32,
    next: Tweet,
}

struct Twitter {
    users: HashMap<i32, User>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn register(&mut self, user_id: i32) {
        if let None = self.users.get(&user_id) {
            let mut u = User::new(user_id);
            self.users.insert(u.id, u);
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.register(user_id);

        if let Some(u) = self.users.get_mut(&user_id) {
            u.push_tweet(tweet_id);
        }
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        self.register(user_id);

        if let Some(u) = self.users.get(&user_id) {
            let mut tweets = vec![];
            for f in u.followers.iter() {
                tweets.extend(self.users.get(f).unwrap().get_tweets());
            }
            tweets.extend(u.get_tweets());
            tweets.sort();
            tweets
                .into_iter()
                .rev()
                .take(HISTORY_SIZE)
                .map(|t| t.1)
                .collect()
        } else {
            vec![]
        }
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.register(follower_id);
        self.register(followee_id);

        if let Some(u) = self.users.get_mut(&follower_id) {
            u.followers.insert(followee_id);
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(u) = self.users.get_mut(&follower_id) {
            u.followers.remove(&followee_id);
        }
    }
}
