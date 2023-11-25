struct Solution;

impl Solution {
    pub fn get_row(num_rows: i32) -> Vec<i32> {
        let mut prev: Vec<i32> = vec![0];

        for i in 0..=num_rows as usize {
            let mut sub: Vec<i32> = vec![0; i + 1];

            sub[0] = 1;
            sub[i] = 1;

            for z in 1..i as usize {
                sub[z] = prev[z -1] + prev[z];
            }
            prev = sub;
        }

        return prev;
    }
}

fn main() {
    let vec  = Solution::get_row(5);
    println!("Vec is {:?}\n", vec);
}


