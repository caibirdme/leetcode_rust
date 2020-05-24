use std::collections::HashMap;

impl Solution {
    pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        let mut debt = HashMap::new();
        for entry in transactions {
            let (a,b,c) = (entry[0],entry[1],entry[2]);
            *debt.entry(a).or_insert(0) += c;
            *debt.entry(b).or_insert(0) -= c;
        }
        let keys: Vec<i32> = debt.iter().filter(|&(_, &v)| v == 0).map(|(&k, _)| k).collect();
        for key in keys {
            debt.remove(&key);
        }
        if debt.is_empty() {
            return 0;
        }
        let mut inverted_index = HashMap::new();
        for (&k,&v) in debt.iter() {
            inverted_index.entry(v).or_insert(vec![]).push(k);
        }
        let mut extra = 0;
        for (&v, arr) in inverted_index.iter() {
            if v > 0 {
                if let Some(indices) = inverted_index.get(&(-v)) {
                    for (k1, k2) in arr.iter().zip(indices) {
                        debt.remove(k1);
                        debt.remove(k2);
                        extra += 1;
                    }
                }
            }
        }
        if debt.is_empty() {
            return extra;
        }
        let neg_keys: Vec<i32> = debt.iter().filter(|&(_,&v)| v < 0).map(|(&k, _)| k).collect();
        let pos_keys: Vec<i32> = debt.iter().filter(|&(_,&v)| v > 0).map(|(&k, _)| k).collect();
        let mut ans = std::i32::MAX;
        Self::dfs(0, &mut debt, &pos_keys, &neg_keys, &mut ans, pos_keys.len());
        ans + extra
    }
    fn dfs(times: i32, debt: &mut HashMap<i32, i32>, pos_keys: &Vec<i32>, neg_keys: &Vec<i32>, ans: &mut i32, pos_count: usize) {
        if times >= *ans {
            return;
        }
        if pos_count == 0{
            *ans = times.min(*ans);
            return;
        }
        for &pk in pos_keys {
            let pk_v = if let Some(&v) = debt.get(&pk) { v } else {0};
            if pk_v <= 0 {
                continue;
            }
            for &nk in neg_keys {
                let nk_v = if let Some(&v) = debt.get(&nk) {v} else {0};
                if nk_v >= 0 {
                    continue;
                }
                let cmp = pk_v + nk_v;
                if cmp > 0 {
                    *debt.get_mut(&pk).unwrap() = cmp;
                    debt.remove(&nk);
                    Self::dfs(times+1, debt, pos_keys, neg_keys, ans, pos_count);
                    debt.insert(nk, nk_v);
                    *debt.get_mut(&pk).unwrap() -= nk_v;
                } else {
                    debt.remove(&pk);
                    *debt.get_mut(&nk).unwrap() = cmp;
                    Self::dfs(times+1, debt, pos_keys, neg_keys, ans, pos_count-1);
                    *debt.get_mut(&nk).unwrap() -= pk_v;
                    debt.insert(pk,pk_v);
                }
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_transfers() {
        let test_cases = vec![
            (vec![vec![0,1,5], vec![2,1,2], vec![2,3,1]], 3),
            (vec![vec![0,1,10], vec![1,0,1], vec![1,2,5], vec![2,0,5]], 1),
            (vec![vec![0,1,10], vec![2,0,5]], 2),
        ];
        for (transactions, expect) in test_cases {
            assert_eq!(Solution::min_transfers(transactions.clone()), expect, "tx: {:?}", transactions);
        }
    }
}