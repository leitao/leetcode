struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ret: i32 = 0;

        let mut i = 0;
        while i < s.len() {
            let c = s.chars().nth(i).unwrap();
            let d = match s.chars().nth(i+1) {
                Some(x) => x,
                None => '!',
            };
            // println!("{}", c);
            match c {
                // println!("chars are {} and {}", c, d);
                'I' => {
                    if d == 'V' {
                        ret += 4;
                        i += 1;
                    } else if d == 'X' {
                        ret += 9;
                        i += 1;
                    } else {
                        ret += 1;
                    }
                }
                'V' => {
                    ret += 5;
                }
                'X' => {
                    if d == 'L' {
                        ret += 40;
                        i += 1;
                    } else if d == 'C' {
                        ret += 90;
                        i += 1;
                    } else {
                        ret += 10;
                    }
                }
                'L' => {
                    ret += 50;
                }
                'C' => {
                    if d == 'D' {
                        ret += 400;
                        i += 1;
                    } else if d == 'M' {
                        ret += 900;
                        i += 1;
                    } else {
                        ret += 100;
                    }
                }
                'D' => {
                    ret += 500;
                }
                'M' => {
                    ret += 1000;
                }
                _ => {
                    panic!("Invalid char = {}", c);
                }
            }
            i += 1;
            // println!("{}", ret);
        }

        return ret;
    }
}

fn main() {
    println!("result III = {}", Solution::roman_to_int("III".to_string()));
    println!("result IV = {}", Solution::roman_to_int("IV".to_string()));
    println!("result LVIII = {}", Solution::roman_to_int("LVIII".to_string()));
    println!("result MCMXCIV = {}", Solution::roman_to_int("MCMXCIV".to_string()));

}
