impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = vec![[0usize; 26]; 1];
        let mut ends = vec![0usize];
        let mut word_id = 1;

        for word in &words {
            let mut node = 0;
            for c in word.bytes().map(|b| (b - b'a') as usize) {
                if trie[node][c] == 0 {
                    trie.push([0; 26]);
                    ends.push(0);
                    trie[node][c] = trie.len() - 1;
                }
                node = trie[node][c];
            }
            ends[node] = word_id;
            word_id += 1;
        }

        let mut result = vec![];
        let (m, n) = (board.len(), board[0].len());

        for i in 0..m {
            for j in 0..n {
                Self::dfs(&mut board, i, j, 0, &mut trie, &mut ends, &words, &mut result);
            }
        }

        result
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, node: usize,
           trie: &mut Vec<[usize; 26]>, ends: &mut Vec<usize>,
           words: &[String], result: &mut Vec<String>) {
        let c = board[i][j];
        if c == '#' { return; }
        let ci = (c as u8 - b'a') as usize;
        let next = trie[node][ci];
        if next == 0 { return; }

        if ends[next] > 0 {
            result.push(words[ends[next] - 1].clone());
            ends[next] = 0;
        }

        board[i][j] = '#';
        let dirs = [(0,1),(0,-1),(1,0),(-1,0)];
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && nj >= 0 && (ni as usize) < board.len() && (nj as usize) < board[0].len() {
                Self::dfs(board, ni as usize, nj as usize, next, trie, ends, words, result);
            }
        }
        board[i][j] = c;
    }
}