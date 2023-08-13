struct Solution;

impl Solution {
    pub fn three_sum(numm: Vec<i32>) -> Vec<Vec<i32>>
    {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut nums = numm.clone();
        nums.sort();

        for i in 0..nums.len() {
            for j in i..nums.len() {
                if i == j {
                    continue;
                }
                let missing =  - (nums[i] + nums[j]);

                let contains = nums[j+1..].binary_search(&missing);
                if contains.is_err() {
                    continue;
                }

                let mut to_add = vec![nums[i], nums[j], missing];
                to_add.sort();

                if !ret.contains(&to_add) {
                    ret.push(to_add);
                }
            }
        }
    return ret;
    }
}

fn main() {
    // let n: Vec<i32> = vec![-1,0,1,2,-1,-4];
    let n: Vec<i32> = vec![-1,0,1,2,-1,-4];
    println!("Array is {:?}", n);
    let ret = Solution::three_sum(n);
    println!("= {:?}",  ret);
}
