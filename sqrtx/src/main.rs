struct Solution;

enum Where {
    TOOSMALL,
    FOUND,
    TOOBIG,
}
impl Solution {
    pub fn check(i: i32, target: i32) -> Where {
        println!("Checking {}", i);

        if i>0 && i32::MAX/i < i {
            return Where::TOOBIG;
        }

        let m = i*i;

        if m > i32::MAX {
            return Where::TOOBIG;
        }

        if i > 0 && m < target && i32::MAX/(i+1) < (i+1) {
            println!("Here?");
            return Where::FOUND;
        }

        if m == target {
            println!("m ={} and target = {}", m, target);
            return Where::FOUND;
        }
        if m < target && (i+1)*(i+1) > target {
            println!("m ={} and target = {}", m, target);
            return Where::FOUND;
        }

        if m > target {
            println!("Too big");
            return Where::TOOBIG;
        }

        if (i+1)*(i+1)  <= target {
            println!("Too small");
            return Where::TOOSMALL;
        }

        unreachable!("not here");
    }


    pub fn my_sqrt(x: i32) -> i32 {
        let mut min = 0;
        let mut max = x/2;
        let mut i = (max + min)/2;


        while min <= max {
            println!("Checking with i={}", i);
            match Solution::check(i, x) {
                Where::FOUND => return i,
                Where::TOOBIG => {max = i - 1;}
                Where::TOOSMALL => {min = i + 1;}
            }
            i = (max + min)/2;
            println!("max={} min={} New i={}", max, min, i);
        }

        return 1;

    }
}

fn main() {
    println!("SOlution is = {}", Solution::my_sqrt(2147395599));
    // println!("SOlution is = {}", Solution::my_sqrt(1));
    // println!("SOlution is = {}", Solution::my_sqrt(4));
    // println!("SOlution is = {}", Solution::my_sqrt(8));
    // println!("SOlution is = {}", Solution::my_sqrt(2147483647));
}
