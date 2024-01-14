struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut v = nums;
        v.sort();
        let len = v.len();

        let mut target_diff = i32::max_value();
        let mut output = 0;

        for i in 0..len {
            for j in i+1..len {
                for z in j+1..len {
                    let tot = v[i] + v[j] + v[z];

                    // if tot > output + target_diff {
                    //     break;
                    // }

                    if (tot - target).abs() < target_diff {
                        target_diff = (tot - target).abs(); 
                        println!("{} {} {} == {}", v[i], v[j], v[z], target_diff);
                        output = tot;
                    }
                }
            }
        }

        return output;
    }
}

fn main() {
    let vec = vec![-1,2,1,-4];
    // let vec = vec![0,0,0];
    println!("{:?}", vec);

    let x = Solution::three_sum_closest(vec, 1);
    println!("Diff for targetting {} is {}", 1, x);
}
