struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut negative = false;
        let st = s.trim();
        let mut ret = 0;
        let first ;

        let mut iter = st.chars();
        first = iter.nth(0).unwrap_or('0');
        let c = first as i32 - '0' as i32;

        if first == '-' {
            negative = true;
        } else if first == '+' {
            //pass;
        } else if c >9 || c < 0 {
            return 0;
        } else {
            ret = c;
        }

        for c in iter {
            let i = c as i32 - '0' as i32;
            if i < 0 || i > 9 {
                match negative {
                    true => {return -ret;}
                    false => {return ret;}
                }
            }
            let z = ret.checked_mul(10);
            if z.is_none() {
                if negative {
                    return i32::MIN
                } else {
                    return i32::MAX
                };
            }
            ret = z.unwrap();

            let z = ret.checked_add(i);
            if z.is_none() {
                if negative {
                    return i32::MIN
                } else {
                    return i32::MAX
                };
            }

            ret = z.unwrap();
        }

        match negative {
            true => {return -ret;}
            false => {return ret;}
        }
    }
}


fn main() {
    let ret = Solution::my_atoi("2147483648".to_string());
    println!("Solution is {}", ret);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }
}
