use std::collections::HashMap;

struct Solution;

impl Solution {
        pub fn is_isomorphic(s: String, t: String) -> bool {
            if s.len() != t.len() {
                return false;
            }
            let mut dict : HashMap<char, char>= HashMap::new();
            for i in 0..s.len() {
                let cs = s.chars().nth(i).unwrap();
                let ct = t.chars().nth(i).unwrap();

                if dict.contains_key(&cs) {
                    if *dict.get(&cs).unwrap() != ct {
                        return false;
                    }

                    continue;
                }

                // ct already mapped ?
                if dict.values().any(|&x| x == ct) {
                    return false;
                }

                dict.insert(cs, ct);
            }

            return true;
        }
}

fn main() {
    // println!("{}", Solution::is_isomorphic("paper".to_string(), "title".to_string()));
    // println!("{}", Solution::is_isomorphic("foo".to_string(), "bar".to_string()));
    // println!("{}", Solution::is_isomorphic("badc".to_string(), "baba".to_string()));
    println!("{}", Solution::is_isomorphic("aaeaa".to_string(), "uuxyy".to_string()));
}
