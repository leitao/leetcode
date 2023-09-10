struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ret = Vec::new();
        let mut local = Vec::new();
        for c in s.as_bytes().to_vec() {
            if local.contains(&c) {
                if local.len() > ret.len() {
                    ret = local.clone();
                }
                let idx = local.iter().position(|x| *x == c).unwrap();

                local = local.split_off(idx + 1);
                local.push(c);
            } else {
                local.push(c);
            }
        }

        if local.len() > ret.len() {
            ret = local;
        }
        return ret.len() as i32;
    }
}

fn main() {
    // println!("{}", Solution::length_of_longest_substring("dvdf".to_string()));
    // println!("{}", Solution::length_of_longest_substring("bcabcbb".to_string()));
    // println!("{}", Solution::length_of_longest_substring("pwwkew".to_string()));
    println!("{}", Solution::length_of_longest_substring("abaab!bb".to_string()));
}
