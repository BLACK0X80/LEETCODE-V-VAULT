use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut hand_vec: Vec<char> = hand.chars().collect();
        hand_vec.sort();
        let hand_s: String = hand_vec.iter().collect();

        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert((board.clone(), hand_s.clone()));
        q.push_back((board, hand_s, 0i32));

        while let Some((cb, ch, step)) = q.pop_front() {
            let cb: Vec<char> = cb.chars().collect();
            let ch: Vec<char> = ch.chars().collect();

            for i in 0..=cb.len() {
                for j in 0..ch.len() {
                    if j > 0 && ch[j] == ch[j-1] { continue; }
                    if i > 0 && cb[i-1] == ch[j] { continue; }

                    let mut pick = false;
                    if i < cb.len() && cb[i] == ch[j] { pick = true; }
                    if i > 0 && i < cb.len() && cb[i-1] == cb[i] && cb[i] != ch[j] { pick = true; }

                    if pick {
                        let mut nb: Vec<char> = cb[..i].to_vec();
                        nb.push(ch[j]);
                        nb.extend_from_slice(&cb[i..]);
                        let nb = remove_same(nb, i as i32);
                        let mut nh = ch.clone();
                        nh.remove(j);
                        let nb_s: String = nb.iter().collect();
                        let nh_s: String = nh.iter().collect();
                        if nb_s.is_empty() { return step + 1; }
                        if visited.insert((nb_s.clone(), nh_s.clone())) {
                            q.push_back((nb_s, nh_s, step + 1));
                        }
                    }
                }
            }
        }
        -1
    }
}

fn remove_same(s: Vec<char>, i: i32) -> Vec<char> {
    if i < 0 || s.is_empty() { return s; }
    let i = i as usize;
    if i >= s.len() { return s; }
    let mut left = i;
    let mut right = i;
    while left > 0 && s[left-1] == s[i] { left -= 1; }
    while right + 1 < s.len() && s[right+1] == s[i] { right += 1; }
    if right - left + 1 >= 3 {
        let mut ns: Vec<char> = s[..left].to_vec();
        ns.extend_from_slice(&s[right+1..]);
        let ni = if left == 0 { 0 } else { left as i32 - 1 };
        remove_same(ns, ni)
    } else {
        s
    }
}
