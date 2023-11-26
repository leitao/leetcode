use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let v = Vec::with_capacity(nums.len());


        for i in 0..nums.len() {
            let val = nums[i];
            if h.contains(&val) {
                h.remove(&val);
                continue;
            }

            h.insert(val);
        }

        if h.len() != 1 {
            panic!("{:?} hs more than one element", h);
        }
        for element in h {
            return element;
        }

        panic!("error\n");
    }
}

fn main() {
    // let v = vec![1,2,3,1,3];
    let v = vec![4,1,2,1,2];
    println!("{:?}", v);
    let a = Solution::single_number(v);
    println!("{}", a); 
}
