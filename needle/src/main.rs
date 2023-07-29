struct Solution;

impl Solution {

    fn compare(haystack: &String, needle: &String, i: usize) -> bool{
        let slice = &haystack[i..];

        println!("{} : checking if -- {} -- starts with -- {}", i, slice, needle);
        return slice.starts_with(needle);

    }
    pub fn str_str(haystack: String, needle: String) -> i32 {
        for i in 0..haystack.len() {
           if Solution::compare(&haystack, &needle, i) {
                return i as i32;
           }
        }

        return -1;

    }
}
fn main() {
    Solution::str_str("sadbutsad".to_string(), "sad".to_string());
}
