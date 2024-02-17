use std::collections::HashMap;
use lazy_static::lazy_static;

struct Solution;

lazy_static!{
    static ref MAP: HashMap<char, Vec<char>> = [
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z'])
    ].iter().cloned().collect();

    static ref MEMO: HashMap<char, Vec<char>> = HashMap::new();
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        println!("Calling with {:?}", digits);
        let mut ret : Vec<String> = Vec::new();

        if digits.len() == 0 {
            return ret;
        }
        let c = digits.chars().nth(0).unwrap_or('-');
        let ds = MAP.get(&c).unwrap();
        println!("ds = {:?} for c = {}", ds, c);
        for d in ds {
            if digits.len() <= 1 {
                ret.push(d.to_string());
                continue;
            }
            let rest = Solution::letter_combinations(digits[1..].into());
            for r in rest {
                let mut st : String = String::new();
                st.push(*d);
                st.push_str(&r[..]);
                // println!("{}", st);
                ret.push(st);
            }
        }

        return ret;
    }
}

fn main() {
    let x = Solution::letter_combinations("23".to_string());
    let x = Solution::letter_combinations("2".to_string());
    let x = Solution::letter_combinations("".to_string());
    println!("{:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(Solution::letter_combinations("2".to_string()), vec!["a", "b", "c"]);
    }
}
