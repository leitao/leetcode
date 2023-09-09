struct Solution;

impl Solution {
    pub fn length_of_last_word(x: String) -> i32 {
        let s = x.trim();
        let mut last = s.len();

        while last >= 1 &&  s.chars().nth(last - 1).unwrap() != ' ' {
            last -= 1;
        }
        return (s.len() - last) as i32;
    }
}
fn main() {
    // let ret = Solution::length_of_last_word("Hello, world!".to_string());
    let ret = Solution::length_of_last_word("a w".to_string());
    println!("ret = {ret}");
    let ret = Solution::length_of_last_word("w".to_string());
    println!("ret = {ret}");
}
