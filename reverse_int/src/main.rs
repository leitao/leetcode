struct Solution;

impl Solution {
    pub fn reverse(xx: i32) -> i32 {
        let mut x = xx;
        let mut i;
        let mut ret = 0;
        let negative = xx < 0;

        if negative {
            x = x.abs();
        }
        let mut max: u64;

        loop {
            i = x%10;
            max = (ret as u64)*10 + i as u64;
            if max > i32::MAX as u64 {
                return 0;
            }
            ret = max as i32;

            x /= 10;
            if x <= 0 {
                break;
            }
        }

        if ret > i32::MAX {
            return 0;
        }

        if negative {
            return -ret;
        }
        return ret;

    }
}

fn main() {
    println!(" = {}", Solution::reverse(123));
    println!(" = {}", Solution::reverse(1534236469));
    println!(" = {}", Solution::reverse(-543));
}
