struct Solution;

impl Solution {
    fn is_palin(chars: &[char]) -> bool {
        if chars.is_empty() {
            return true;
        }

        let mut last = chars.len() - 1;
        let mut first = 0;


        // print!("is_palin({:?})? ", chars);
        while first < last {
            if chars[first] != chars[last] {
                // println!("False");
                return false;
            }
            first += 1;
            last -= 1;
        }

        return true;
    }

    fn longest(chars: &[char], longest_so_far: usize) -> usize{
        let len = chars.len();

        for i in (0..=len).rev() {
            if i < longest_so_far {
                return 0;
            }
            if Solution::is_palin(&chars[0..i]) {
                return i;
            }
        }
        return 0;
    }

    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        let mut longest = usize::MIN;
        let mut pal = String::new();

        for i in 0..len {
            if len - i < longest {
                break;
            }

            let longest_local = Solution::longest(&chars[i..], longest);

            if longest_local >  longest {
                longest = longest_local;
                pal = chars[i..i+longest_local].into_iter().collect();
            }
        }

        return pal;
    }
}


fn main() {
    // let s = "babad".to_string();
    let s = "a".to_string();
    // let s2 = "bb".to_string();
    let s3 = "cbbd".to_string();

    println!("{}", Solution::longest_palindrome(s));
    // println!("{}", Solution::longest_palindrome(s2));
    println!("{}", Solution::longest_palindrome(s3));
}
