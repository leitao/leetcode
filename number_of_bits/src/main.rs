struct Solution;

impl Solution {
        pub fn hammingWeight (n: u32) -> i32 {
            let mut m = n;
            let mut res = 0;
            while m > 0 {
                if m & 1 > 0 {
                    m = m - 1;
                    res += 1;
                }
                m >>= 1;
            }

            return res;
        }
}

fn main() {
    let i = 1024;;

    println!("{} {}", i, Solution::hammingWeight(i));
}
