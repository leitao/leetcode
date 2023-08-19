struct Solution;

impl Solution {
        pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
            let mut ret = digits.clone();
            let mut l: i32 = (ret.len() - 1) as i32;

            loop {
                if l == -1 {
                    ret.insert(0, 1);
                    break;
                }
                let len = l as usize;

                if ret[len] == 9 {
                    ret[len] = 0;
                    l -= 1;
                    continue;
                }

                ret[len] += 1;
                break;
            }

            return ret;
        }
}

fn main() {
    let input = vec![1,2,3];
    let input2 = vec![3,2,1];

    println!("{:?}", Solution::plus_one(input));
    println!("{:?}", Solution::plus_one(input2));
}
