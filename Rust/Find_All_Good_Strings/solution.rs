impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let evil = evil.as_bytes();
        let m = evil.len();

        let mut fail = vec![0usize; m];
        let mut j = 0usize;
        for i in 1..m {
            while j > 0 && evil[i] != evil[j] { j = fail[j-1]; }
            if evil[i] == evil[j] { j += 1; }
            fail[i] = j;
        }

        fn kmp_next(fail: &[usize], state: usize, c: u8, evil: &[u8]) -> usize {
            let mut j = state;
            while j > 0 && c != evil[j] { j = fail[j-1]; }
            if c == evil[j] { j += 1; }
            j
        }

        fn count(
            s: &[u8], evil: &[u8], fail: &[usize],
            tight: bool, started: bool, pos: usize, evil_state: usize,
            memo: &mut Vec<Vec<Vec<Vec<u64>>>>,
            n: usize,
        ) -> u64 {
            const MOD: u64 = 1_000_000_007;
            if evil_state == evil.len() { return 0; }
            if pos == n { return 1; }
            let ti = tight as usize;
            let si = started as usize;
            if memo[pos][evil_state][ti][si] != u64::MAX {
                return memo[pos][evil_state][ti][si];
            }
            let limit = if tight { s[pos] } else { b'z' };
            let mut res = 0u64;
            for c in b'a'..=limit {
                let ns = kmp_next(fail, evil_state, c, evil);
                if ns == evil.len() { continue; }
                res = (res + count(s, evil, fail, tight && c == limit, true, pos+1, ns, memo, n)) % MOD;
            }
            memo[pos][evil_state][ti][si] = res;
            res
        }

        let n = n as usize;
        let m = evil.len();
        let mut memo1 = vec![vec![vec![vec![u64::MAX; 2]; 2]; m+1]; n];
        let mut memo2 = vec![vec![vec![vec![u64::MAX; 2]; 2]; m+1]; n];
        let a = count(s2, evil, &fail, true, false, 0, 0, &mut memo2, n);
        let b = count(s1, evil, &fail, true, false, 0, 0, &mut memo1, n);
        let _ = memo1;
        let evil_in_s1 = {
            let mut j = 0usize;
            let mut found = false;
            for &c in s1 {
                while j > 0 && c != evil[j] { j = fail[j-1]; }
                if c == evil[j] { j += 1; }
                if j == m { found = true; break; }
            }
            found
        };
        let res = (a + MOD - b + if evil_in_s1 { 0 } else { 1 }) % MOD;
        res as i32
    }
}