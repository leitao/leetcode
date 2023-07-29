struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string = x.to_string();
        let mut vec: Vec<char> = Vec::new();

        for c in string.chars() {
            vec.push(c)
        }

        let mut a = 0;
        let mut b = vec.len() - 1;

        while a <= b {
            println!("{} {} and {:?}", a, b, vec);
            if vec[a] != vec[b] {
                return false
            }
            if b == 0 {
                break
            }
            a += 1;
            b -= 1;
        }
        return true;
    }
}
fn main() {
    let ret = Solution::is_palindrome(0);
    println!("Ret is {}", ret);
}
