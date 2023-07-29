struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j = 0; //trailing pointer

        println!("vec bef is {:?}\n", nums);
        for i in 0..nums.len() {
            let v = nums[i];

            if i != j {
                nums[j] = v;
            }

            if v != val {
                j += 1;
                continue;
            } else {
                continue;
            }
        }


        println!("vec now is {:?}\n", nums);
        return j as i32;
    }
}


fn main() {
    let mut v = vec![1,23,5,12,3,5];
    let mut v2 = vec![12, 1,23,12, 5,12,3,5];

    Solution::remove_element(&mut v, 12);
    Solution::remove_element(&mut v2, 12);
}
