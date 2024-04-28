struct Solution;


struct Single<X, Y>(X, Y);

impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let size = m+n;

            for i in 0..n {
                if nums1.pop() != Some(0) {
                    panic!("Wrong indexing");
                }
            }

            while !nums2.is_empty() {
                match nums2.pop() {
                    Some(i) => nums1.push(i),
                    _ => panic!("What?")
                }
            }
            nums1.sort();
        }
}

fn main() {
    let mut nums1 = vec![1,40, 50, 0, 0, 0];
    let mut nums2 = vec![2, 12, 70];

    let n : i32 = nums2.len() as i32;
    Solution::merge(&mut nums1, 3, &mut nums2, n);

    println!("{:?}", nums1);
}
