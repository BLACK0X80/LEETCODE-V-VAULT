impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let digits: Vec<u8> = num.bytes().collect();
        let target = target as i64;
        let mut result = Vec::new();
        let mut path = Vec::new();

        backtrack(&digits, target, 0, 0, 0, &mut path, &mut result);
        result
    }
}

fn backtrack(
    digits: &[u8],
    target: i64,
    index: usize,
    eval: i64,
    last: i64,
    path: &mut Vec<u8>,
    result: &mut Vec<String>,
) {
    if index == digits.len() {
        if eval == target {
            result.push(String::from_utf8(path.clone()).unwrap());
        }
        return;
    }

    let path_len = path.len();

    let mut val: i64 = 0;
    for i in index..digits.len() {
        val = val * 10 + (digits[i] - b'0') as i64;

        if index == 0 {
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, val, val, path, result);
            path.truncate(path_len);
        } else {
            path.push(b'+');
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, eval + val, val, path, result);
            path.truncate(path_len);

            path.push(b'-');
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, eval - val, -val, path, result);
            path.truncate(path_len);

            path.push(b'*');
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, eval - last + last * val, last * val, path, result);
            path.truncate(path_len);
        }

        if val == 0 {
            break;
        }
    }
}
