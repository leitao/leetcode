struct Solution;


impl Solution {
    pub fn ret(negate: bool, mut t: i64) -> i32 {
        let times: i32;

        if negate {
            t = -t;
        }

        if t >= i32::MAX as i64{
            times = i32::MAX ;
        } else if t <= i32::MIN as i64{
            times = i32::MIN ;
        } else {
            times = t as i32;
        }

        times

    }

    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut times: i64 = 0;

        let divisor_negative = divisor < 0;
        let dividend_negative = dividend < 0;
        let real_divisor: i64;
        let real_dividend: i64;
        let negate = divisor_negative ^ dividend_negative;

        println!("Negate = {}", negate);
        if divisor_negative {
            real_divisor = -(divisor as i64);
        } else {
            real_divisor = divisor as i64;
        }

        if dividend_negative {
            real_dividend = -(dividend as i64);
        } else {
            real_dividend = dividend as i64;
        }

        if real_divisor == 1 {
            return Solution::ret(negate, real_dividend);
        }


        let mut target = real_dividend;

        while target > 0 {
            times += 1;
            target -= real_divisor;
        }

        if target < 0 {
            times -= 1;
        }

        Solution::ret(negate, times)

    }
}

fn main() {
    let d = -2147483648;
    let v = 1;
    let res = Solution::divide(d, v);
    println!("{}/{} = {}", d, v, res);
}
