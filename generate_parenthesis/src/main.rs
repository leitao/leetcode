struct Solution;

impl Solution {
    pub fn generate_parenthesis_inner(can_close: i32, can_open: i32) -> Vec<String> {
        let mut ret = Vec::new();

        if can_open <= 0 && can_close <= 0 {
            return vec!["".to_string()];
        }

        if can_open > 0{
            let m = Solution::generate_parenthesis_inner(can_close + 1, can_open - 1);

            for mut i in m {
                i.insert(0, '(');
                ret.push(i)
            }
        }

        if can_close > 0{
            let m = Solution::generate_parenthesis_inner(can_close - 1, can_open);

            for mut i in m {
                i.insert(0, ')');
                ret.push(i)
            }
        }
        ret
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Solution::generate_parenthesis_inner(0, n)
    }
}
fn main() {
    let f = Solution::generate_parenthesis(4);

    println!("{:?}", f);
}
