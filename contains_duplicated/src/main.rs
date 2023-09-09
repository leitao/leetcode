struct Solution;

impl Solution {
        pub fn contains_duplicate(nums: Vec<i32>) -> bool {
            let mut c = nums.clone();

            c.sort();

            for (i, val) in c.iter().enumerate() {
                if i == 0  {
                    continue;
                }

                if *val == c[i-1] {
                    return true;
                }
            }

            return false;
        }
}

fn main() {
    let vec = vec!(1,2,3,1);


    println!("{}", Solution::contains_duplicate(vec));
}
