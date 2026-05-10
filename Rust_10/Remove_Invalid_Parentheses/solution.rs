use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();

        let mut left_rem = 0i32;
        let mut right_rem = 0i32;
        for &c in &chars {
            if c == '(' {
                left_rem += 1;
            } else if c == ')' {
                if left_rem > 0 { left_rem -= 1; } else { right_rem += 1; }
            }
        }

        let mut result = HashSet::new();
        backtrack(&chars, 0, left_rem, right_rem, 0, &mut Vec::new(), &mut result);
        result.into_iter().collect()
    }
}

fn backtrack(
    chars: &[char],
    index: usize,
    left_rem: i32,
    right_rem: i32,
    open: i32,
    current: &mut Vec<char>,
    result: &mut HashSet<String>,
) {
    if index == chars.len() {
        if left_rem == 0 && right_rem == 0 && open == 0 {
            result.insert(current.iter().collect());
        }
        return;
    }

    let c = chars[index];

    if c == '(' && left_rem > 0 {
        backtrack(chars, index + 1, left_rem - 1, right_rem, open, current, result);
    }

    if c == ')' && right_rem > 0 {
        backtrack(chars, index + 1, left_rem, right_rem - 1, open, current, result);
    }

    current.push(c);

    if c == '(' {
        backtrack(chars, index + 1, left_rem, right_rem, open + 1, current, result);
    } else if c == ')' {
        if open > 0 {
            backtrack(chars, index + 1, left_rem, right_rem, open - 1, current, result);
        }
    } else {
        backtrack(chars, index + 1, left_rem, right_rem, open, current, result);
    }

    current.pop();
}