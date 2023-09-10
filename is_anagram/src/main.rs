use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut h: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
           let v = h.entry(c).or_insert(0);
           *v = *v + 1;
        }
        for c in t.chars() {
            match h.get_mut(&c) {
                Some(val) =>  {*val -= 1;}
                None => {return false;}
            }
        }

        for v in h.values() {
            if *v != 0 {
                return false;
            }
        }
        return true;
    }
}
fn main() {
    println!("{}", Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
}
