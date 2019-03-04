use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph = HashMap::new();
        for ticket in &tickets {
            let (from, to) = (&ticket[0], &ticket[1]);
            let sub_map = graph.entry(from).or_insert(HashMap::new());
            let count = sub_map.entry(to).or_insert(0);
            *count += 1;
        }
        let n = graph.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return tickets[0].clone();
        }
        let edges = tickets.len();
        let mut visited = vec![];
        let mut found = false;
        for (&k,_) in graph.iter() {
            if k == "JFK" {
                visited.push(k.to_string());
                Self::dfs(k, &mut graph, &mut visited, edges, &mut found);
                break;
            }
        }
        visited
    }
    fn dfs(start: &String, graph: &mut HashMap<&String, HashMap<&String, i32>>, visit: &mut Vec<String>, edges: usize, found: &mut bool) {
        if *found {
            return;
        }
        if edges == 0 {
            *found = true;
            return;
        }
        let mut nodes = graph.get(start).map(|tos| {
            tos.iter()
                .filter(|(&to, &val)| val > 0)
                .map(|(&x,_)| x)
                .collect::<Vec<&String>>()
        });
        if nodes.is_none() {
            return;
        }
        nodes.as_mut().unwrap().sort();
        if let Some(node_list) = nodes {
            for tos in node_list {
                let count = graph.get_mut(start).unwrap().get_mut(tos).unwrap();
                *count -=1;
                visit.push(tos.clone());
                Self::dfs(tos, graph, visit, edges-1, found);
                if *found {
                    return;
                }
                let count = graph.get_mut(start).unwrap().get_mut(tos).unwrap();
                visit.pop();
                *count+=1;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_itinerary() {
        let test_cases = vec![
            (vec![["JFK","KUL"],["JFK","NRT"],["NRT","JFK"]], vec!["JFK", "NRT", "JFK","KUL"]),
            (vec![["JFK", "ASD"]], vec!["JFK", "ASD"]),
            (vec![["JFK", "CTU"], ["CTU", "BUA"]], vec!["JFK", "CTU", "BUA"]),
            (vec![["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]], vec!["JFK","ATL","JFK","SFO","ATL","SFO"]),
            (vec![["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"]], vec!["JFK", "MUC", "LHR", "SFO", "SJC"]),
        ];
        for (tickets, expect) in test_cases {
            let actual = Solution::find_itinerary(tickets.into_iter().map(|t| vec![t[0].to_string(), t[1].to_string()]).collect());
            assert_eq!(actual, expect);
        }
    }
}