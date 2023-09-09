struct Solution;

impl Solution {

    pub fn get(amount: i32, coins: &Vec<i32>) -> Option<i32> {
        let mut total = None;

        return total;
    }

    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        return Solution::get(amount, &coins).unwrap_or(0);
    }
}

fn main() {
    println!("{:?}", Solution::change(5, vec![1,2,5]));
}
