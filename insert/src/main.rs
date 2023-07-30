struct Solution;

impl Solution {

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return Solution::_search_insert(nums.into_boxed_slice(), target);
    }
    pub fn _search_insert(nums: Box<[i32]>, target: i32) -> i32 {
        let max = nums.len();
        let l = nums.len() / 2;

        println!("{:?}", nums);
        if nums.len() == 0 {
            return 0;
        }
        loop {
            if nums[l] < target {
                if l == max - 1 || nums[l+1] > target {
                    return  (l + 1) as i32;
                }
            }
            if target > nums[l] {
                return l as i32 + Solution::_search_insert(nums[l..max].into(), target);
            } else {
                return Solution::_search_insert(nums[0..l].into(), target);
            }
        }
    }
}



fn main() {
    // [1,3,5,6]
    //
    let v = vec![1,3,5,6];
    let ret = Solution::search_insert(v.clone(), 0);
    println!("ret for {:?} is {ret}", v);

    // let v = vec![1,3,5,6];
    // let ret = Solution::search_insert(v.clone(), 5);
    // println!("ret for {:?} is {ret}", v);
    // let ret = Solution::search_insert(v.clone(), 2);
    // println!("ret for {:?} is {ret}", v);
    // let ret = Solution::search_insert(v.clone(), 7);
    // println!("ret for {:?} is {ret}", v);
}
