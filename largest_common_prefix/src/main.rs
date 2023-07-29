struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut st = String::new();
        let mut i = 0;
        // let mut strss = strs.clone();

        if strs.len() == 0 {
            return st;
        }
        let first = &strs[0];

        loop {
            let c: char;
            let tmp = first.chars().nth(i);

            match tmp {
                Some(x) => c = x,
                _ => return st,
            };

            for elem in &strs {
                let d = elem.chars().nth(i);
                if d.is_none() {
                    return st;
                }
                if d.unwrap() != c {
                    return st;
                }
            }
            st.push(c);
            i += 1;
        }


    }
}

fn main() {
    println!("Result is = {}", Solution::longest_common_prefix(vec!["foo".to_string(), "breno".to_string(), "bruno".to_string()]));
}
