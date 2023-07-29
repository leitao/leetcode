struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string: String = x.to_string();
        let mut vec: Vec<char> = Vec::new();                                                                                                                   

        for c in string.chars() {
            vec.push(c)
        }

        let mut a: usize = 0;
        let mut b: usize = vec.len() - 1;

        while a <= b {
            if vec[a] != vec[b] {
                return false
            }
            a += 1;
            b -= 1;
        }
        return true;
    }
}
fn main() {
    let ret: bool = Solution::is_palindrome(121);
    println!("Ret is {}", ret);
}

