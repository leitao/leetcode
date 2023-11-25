struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows as usize {
            let mut sub: Vec<i32> = vec![0; i + 1];

            sub[0] = 1;
            sub[i] = 1;

            for z in 1..i as usize {
                let prev = &ret[i - 1];
                sub[z] = prev[z -1] + prev[z];
            }

            ret.push(sub);
        }

        return ret;
    }
}

fn main() {
    let vec  = Solution::generate(30);
    println!("Vec is {:?}\n", vec);
}


