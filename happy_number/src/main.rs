use std::collections::HashSet;

struct Solution;

impl Solution {
        pub fn get_pieces(mut n: i32) -> Vec<i32> {
            let mut ret: Vec<i32> = Vec::new();

            while n > 0 {
                let val =  n % 10;
                ret.push(val);
                n /= 10;
            }

            return ret;
        }

        pub fn sum_them_all(pieces: Vec<i32>) -> i32 {
            let mut ret = 0;
            for i in pieces.iter().rev() {
                ret += i.pow(2);
            }

            return ret;
        }

        pub fn ret_sum(n: i32) -> i32 {
            let pieces = Solution::get_pieces(n);
            let s = Solution::sum_them_all(pieces);

            return s;
        }

        pub fn is_happy(n: i32) -> bool {
            let mut inter = n;
            let mut hs = HashSet::new();
            loop {
                hs.insert(inter);
                inter = Solution::ret_sum(inter);
                if inter == 1 {
                    return true;
                }
                if hs.contains(&inter) {
                    return false;
                }
                // println!("{}", inter);
            }

        }
}

fn main() {
    // let input = 19;
    let input = 2;

    println!("{}", Solution::is_happy(input));
}
