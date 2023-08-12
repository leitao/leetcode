struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>>
    {
        let mut ret: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                for k in 0..nums.len() {
                    if (j != k && k != i && i != j) && (nums[i] + nums[j] + nums[k] == 0) {
                        let mut to_add = vec![nums[i], nums[j], nums[k]];
                        to_add.sort();
                        if !ret.contains(&to_add) {
                            ret.push(to_add);

                        }

                    }
                }
            }
        }
    return ret;
    }
}
fn main() {
    let n: Vec<i32> = vec![-1,0,1,2,-1,-4];
    let ret = Solution::three_sum(n);
    println!("= {:?}",  ret);
}
