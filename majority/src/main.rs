use std::collections::HashMap;

struct Solution;


impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut h: HashMap<i32, u32> = HashMap::new();

        for e in nums {
	    // Gets a reference and update it
	    *h.entry(e).or_insert(0) += 1;
        }

	let max = h.iter().max_by(|x, y| x.1.cmp(y.1));

	return *max.unwrap().0;
    }
}


fn main()
{
    // let a = Solution::majority_element(vec![3,2,3]);
    let a = Solution::majority_element(vec![2,2,1,1,1,2,2]);
    println!("{}", a);
    
}
