struct Solution;
// [1,2,3,3,3,5]


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut x: u32 = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[x as usize] {
                if i - x as usize > 1 {
                    nums[x as usize + 1] = nums[i];
                }
                x += 1;
            }
        }
        return i32::try_from(x).unwrap() + 1;
    }
}

fn main() {
    let mut v = vec![1,2,3,3, 5, 5,5,6,7];

    let ret =  Solution::remove_duplicates(&mut v);

}
