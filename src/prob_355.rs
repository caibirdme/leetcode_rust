use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

struct Twitter {
    follow_relation: HashMap<i32,HashSet<i32>>,
    primary_key: usize,
    tweets: HashMap<i32, HashMap<i32, usize>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self{
            follow_relation: HashMap::new(),
            primary_key: 0,
            tweets: HashMap::new(),
        }
    }

    /** Compose a new tweet. */
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.primary_key+=1;
        self.tweets.entry(user_id).or_insert(HashMap::new()).insert(tweet_id, self.primary_key);
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        if let Some(tweets) = self.tweets.get(&user_id) {
            for (&id,&pk) in tweets {
                if heap.len() < 10 {
                    heap.push(PostItem::new(id, pk));
                } else {
                    let top = heap.peek().unwrap().pk;
                    if pk > top {
                        heap.pop();
                        heap.push(PostItem::new(id, pk));
                    }
                }
            }
        }
        if let Some(followee_set) = self.follow_relation.get(&user_id) {
            for followee in followee_set {
                if let Some(tweets) = self.tweets.get(followee) {
                    for (&id,&pk) in tweets {
                        if heap.len() < 10 {
                            heap.push(PostItem::new(id, pk));
                        } else {
                            let top = heap.peek().unwrap().pk;
                            if pk > top {
                                heap.pop();
                                heap.push(PostItem::new(id, pk));
                            }
                        }
                    }
                }
            }
        }
        let mut v: Vec<i32> = heap.into_sorted_vec().into_iter().map(|x| x.id).collect();
        if v.len() > 10 {
            v.truncate(10);
        }
        v
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.follow_relation.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_relation.entry(follower_id).and_modify(|v| {v.remove(&followee_id);});
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct PostItem {
    id: i32,
    pk: usize,
}

impl PostItem {
    pub fn new(id: i32, pk: usize) -> Self {
        Self{
            id,pk,
        }
    }
}

impl Ord for PostItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.pk.cmp(&self.pk)
    }
}

impl PartialOrd for PostItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

#[cfg(test)]
mod tests {
    use super::Twitter;

    #[test]
    fn test_twitter() {
        let mut obj = Twitter::new();
        obj.post_tweet(1,5);
        assert_eq!(vec![5], obj.get_news_feed(1));
        obj.follow(1,2);
        obj.post_tweet(2,6);
        assert_eq!(vec![6,5], obj.get_news_feed(1));
        obj.unfollow(1,2);
        assert_eq!(vec![5], obj.get_news_feed(1));
        assert_eq!(vec![6], obj.get_news_feed(2));
        assert_eq!(obj.get_news_feed(3), vec![]);
        obj.follow(2,1);
        assert_eq!(vec![6,5], obj.get_news_feed(2));
        obj.post_tweet(2,7);
        assert_eq!(vec![7,6,5], obj.get_news_feed(2));
        assert_eq!(vec![5], obj.get_news_feed(1));
    }
    #[test]
    fn test_tc() {
        let mut obj = Twitter::new();
        let posts = vec![(1,5),(1,3),(1,101),(1,13),(1,10),(1,2),(1,94),(1,505),(1,333)];
        for (uid,tid) in posts {
            obj.post_tweet(uid, tid);
        }
        assert_eq!(vec![333,505,94,2,10,13,101,3,5], obj.get_news_feed(1));
    }
    #[test]
    fn test_another() {
        let posts = vec![(1,5),(1,3),(1,101),(1,13),(1,10),(1,2),(1,94),(1,505),(1,333),(1,22),(1,11)];
        let mut obj = Twitter::new();
        for (uid,tid) in posts {
            obj.post_tweet(uid, tid);
        }
        assert_eq!(vec![11,22,333,505,94,2,10,13,101,3], obj.get_news_feed(1));

    }
}