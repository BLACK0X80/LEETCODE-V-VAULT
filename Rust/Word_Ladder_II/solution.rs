use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn find_ladders(begin: String, end: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end) { return vec![]; }

        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        let mut layer: HashSet<String> = HashSet::new();
        layer.insert(begin.clone());
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(begin.clone());
        let mut found = false;

        'bfs: while !layer.is_empty() {
            let mut next: HashSet<String> = HashSet::new();
            for word in &layer {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let orig = chars[i];
                    for c in 'a'..='z' {
                        if c == orig { continue; }
                        chars[i] = c;
                        let new_word: String = chars.iter().collect();
                        if word_set.contains(&new_word) && !visited.contains(&new_word) {
                            next.insert(new_word.clone());
                            parents.entry(new_word.clone()).or_default().push(word.clone());
                            if new_word == end { found = true; }
                        }
                        chars[i] = orig;
                    }
                }
            }
            if found { break 'bfs; }
            for w in &next { visited.insert(w.clone()); }
            layer = next;
        }

        if !found { return vec![]; }

        let mut result = vec![];
        let mut path = vec![end.clone()];
        Self::dfs(&end, &begin, &parents, &mut path, &mut result);
        result
    }

    fn dfs(word: &str, begin: &str, parents: &HashMap<String, Vec<String>>, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if word == begin {
            let mut p = path.clone();
            p.reverse();
            result.push(p);
            return;
        }
        if let Some(pars) = parents.get(word) {
            for par in pars {
                path.push(par.clone());
                Self::dfs(par, begin, parents, path, result);
                path.pop();
            }
        }
    }
}