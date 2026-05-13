impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let b = encoded_text.as_bytes();
        let cols = b.len() / rows;
        let mut res = Vec::new();

        for col in 0..cols {
            for i in 0..rows {
                if col + i >= cols { break; }
                res.push(b[i * cols + col + i]);
            }
        }

        let end = res.iter().rposition(|&c| c != b' ').map_or(0, |i| i + 1);
        String::from_utf8(res[..end].to_vec()).unwrap()
    }
}