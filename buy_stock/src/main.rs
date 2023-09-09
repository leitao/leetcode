struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_idx = 0;
        let mut profit = 0;

        if prices.len() == 0 {
            return 0;
        }

        for (j, _) in prices.iter().enumerate() {
            if prices[j] < prices[min_idx] {
                min_idx = j;
                continue;
            }

            if prices[j] - prices[min_idx] > profit {
                profit = prices[j] - prices[min_idx];
            }
        }

        return profit;
    }
}

fn main() {
    // let prices = vec!(7,6,4,3,1);
    let prices = vec!(7,1,5,3,6,4);
    println!("{}", Solution::max_profit(prices));
}
