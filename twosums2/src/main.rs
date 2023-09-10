struct Solution;
use std::collections::HashMap;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = vec!();
        let mut h: HashMap<i32, i32> = HashMap::new();

        for i in nums.iter() {
            let entry = h.entry(*i).or_default();
            *entry = *entry + 1;
        }

        for i in nums.iter() {
            let des = target - i;

            if !h.contains_key(&des) {
                continue;
            }

            if des == *i && *h.get(&i).unwrap() < 2 {
                continue;
            }

            for (idx, x) in nums.iter().enumerate() {
                if *x == des || *x == *i {
                    ret.push(idx as i32);

                    continue;
                }
            }// found
            return ret;
        }

        return ret;
    }
}

fn main() {
    let mut vec = vec!(2,7,11,15);
    vec = vec!(3,2,4);
}
