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
    // let f = 0;
    fn max_value(piles: &mut Vec<Vec<i32>>, k: i32) ->  i32 {

        if k == 0 {
            return 0;
        }

        // {
        //     let mut hash = HASH.lock().unwrap();
        //     let exists = hash.get(&(k, piles.clone()));
        //     println!("Exist? {:?}\n", exists);
        //     if exists.is_some() {
        //         println!("got cache\n");
        //         return *exists.unwrap();
        //     }
        // }

        // del hash;

        // let foo = hash.get(&(k, copy));

        let mut max_vec = vec![0; piles.len()];

        for i in 0..piles.len() {
            if piles[i].len() == 0 {
                max_vec[i] = 0;
                continue;
            }
            let mut p = piles.clone();
            let current = p[i][0];
            p[i].remove(0);
            max_vec[i] = current + Solution::max_value(&mut p, k - 1);
        }

        let ret = *max_vec.iter().max().unwrap();

        // {
        //     let mut hash = HASH.lock().unwrap();
        //     let copy = piles.clone();
        //     hash.insert((k,copy), ret);
        //     println!("Inserting Hash is {:?}\n", hash);
        // }

        return ret;
    }

    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut p  = piles.clone();
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
