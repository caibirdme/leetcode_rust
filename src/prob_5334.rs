use std::collections::{HashMap, BTreeSet};
use std::ops::Bound::Included;

struct TweetCounts {
    map: HashMap<String, BTreeSet<usize>>
}

impl TweetCounts {

    pub fn new() -> Self {
        Self{
            map: HashMap::new(),
        }
    }

    pub fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.map.entry(tweet_name).or_insert(BTreeSet::new()).insert(time as usize);
    }

    pub fn get_tweet_counts_per_frequency(&self, freq: String, tweet_name: String, start_time: i32, end_time: i32) -> Vec<i32> {
        let mut it = Interval::new(start_time as usize, end_time as usize, Frequency::new(freq.as_str()));
        if let Some(set) = self.map.get(&tweet_name) {
            let mut ans = vec![];
            while let Some((s,e)) = it.next() {
                ans.push(set.range((Included(&s), Included(&e))).into_iter().count() as i32);
            }
            ans
        } else {
            let mut ans = vec![];
            for _ in it.into_iter() {
                ans.push(0);
            }
            ans
        }
    }
}

enum Frequency {
    Minute,
    Hour,
    Day
}

impl Frequency {
    fn new(s: &str) -> Self {
        match s {
            "minute" => Frequency::Minute,
            "hour" => Frequency::Hour,
            "day" => Frequency::Day,
            _ => unimplemented!(),
        }
    }
    fn to_sec(&self) -> usize {
        match *self {
            Frequency::Day => 86400,
            Frequency::Hour => 3600,
            Frequency::Minute => 60,
        }
    }
}

struct Interval {
    start: usize,
    end: usize,
    idx: usize,
    t: Frequency,
}

impl Interval {
    pub fn new(start: usize, end: usize, t: Frequency) -> Self {
        Interval { start, end, idx:0, t}
    }
}

impl Iterator for Interval {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let sec = self.t.to_sec();
        let next_start = self.start + self.idx * sec;
        if next_start > self.end {
            return None;
        }
        self.idx += 1;
        let next_end = self.end.min(self.start+self.idx*sec - 1);
        Some((next_start, next_end))
    }
}

#[cfg(test)]
mod tests {
    use super::TweetCounts;

    #[test]
    fn test_get_tweet_counts_per_frequency() {
        let mut tweetCounts = TweetCounts::new();
        tweetCounts.record_tweet("tweet3".to_string(), 0);
        tweetCounts.record_tweet("tweet3".to_string(), 60);
        tweetCounts.record_tweet("tweet3".to_string(), 10);
        let mut c = tweetCounts.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 59);
        assert_eq!(c, vec![2]);
        c = tweetCounts.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 60);
        assert_eq!(c, vec![2,1]);
        tweetCounts.record_tweet("tweet3".to_string(), 120);
        c = tweetCounts.get_tweet_counts_per_frequency("hour".to_string(), "tweet3".to_string(), 0, 210);
        assert_eq!(c, vec![4]);
    }
}