impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        const ONES: &[&str] = &[
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven",
            "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen",
            "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
        ];
        const TENS: &[&str] = &[
            "", "", "Twenty", "Thirty", "Forty", "Fifty",
            "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        const THOUSANDS: &[&str] = &["", "Thousand", "Million", "Billion"];

        fn helper(n: i32, ones: &[&str], tens: &[&str]) -> String {
            if n == 0 {
                return String::new();
            } else if n < 20 {
                return ones[n as usize].to_string();
            } else if n < 100 {
                let t = tens[(n / 10) as usize].to_string();
                let o = ones[(n % 10) as usize];
                return if o.is_empty() { t } else { format!("{} {}", t, o) };
            } else {
                let h = format!("{} Hundred", ones[(n / 100) as usize]);
                let rest = helper(n % 100, ones, tens);
                return if rest.is_empty() { h } else { format!("{} {}", h, rest) };
            }
        }

        let mut num = num;
        let mut parts: Vec<String> = Vec::new();
        let mut i = 0;

        while num > 0 {
            let chunk = num % 1000;
            if chunk != 0 {
                let chunk_str = helper(chunk, ONES, TENS);
                if THOUSANDS[i].is_empty() {
                    parts.push(chunk_str);
                } else {
                    parts.push(format!("{} {}", chunk_str, THOUSANDS[i]));
                }
            }
            num /= 1000;
            i += 1;
        }

        parts.reverse();
        parts.join(" ")
    }
}
