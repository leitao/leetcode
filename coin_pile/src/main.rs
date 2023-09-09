struct Solution;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;



lazy_static! {
    static ref HASH: Mutex<HashMap<(i32, Vec<Vec<i32>>), i32>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

impl Solution {
    fn max_value(piles: &mut Vec<Vec<i32>>, k: i32) ->  i32 {
        if k == 0 {
            return 0;
        }

        let mut max = 0;

        for i in 0..piles.len() {
            if piles[i].len() == 0 {
                continue;
            }
            let current = piles[i][0];
            let mut lmax = current;
            if k > 1 {
                piles[i].remove(0);
                lmax += Solution::max_value(piles, k - 1);
                piles[i].insert(0, current);
            }

            if lmax > max {
                max = lmax;
            }
        }

        return max;
    }

    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut p = piles.clone();
        let ret = Solution::max_value(&mut p, k);

        return ret;
    }
}

fn main() {
    let mut piles = vec!(vec!(1,100,3),vec!(7,8,9));

    println!("Piles is {:?}", piles);
    let mut ret = Solution::max_value_of_coins(piles, 2);
    println!("Ret {}", ret);
    println!("----");
    piles = vec!(vec!(1,2,3),vec!(7));
    println!("Piles is {:?}", piles);
    ret = Solution::max_value_of_coins(piles, 2);
    println!("Ret {}", ret);
}
