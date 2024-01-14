struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in height.iter().enumerate() {
            for j in height[i.0..].iter().enumerate().rev() {
                let hlocal = i.1.min(j.1);
                let dlocal = j.0 as i32;

                if i.1*dlocal < max {
                    break;
                }


                let local = *hlocal * dlocal;

                max = i32::max(max, local);
            }
        }
        return max;
    }
}

fn main() {
    let v = vec![1,8,6,2,5,4,8,3,7];

    let r = Solution::max_area(v);

    println!("val is {}", r);
}
