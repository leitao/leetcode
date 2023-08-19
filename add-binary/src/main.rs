struct Solution;

impl Solution {

        pub fn sum(a: char, b: char, carry: u32) -> (char, u32) {
            let aa = a.to_digit(10).unwrap();
            let bb = b.to_digit(10).unwrap();

            let sum = aa + bb + carry;

            match sum {
                0 => return ('0', 0),
                1 => return ('1', 0),
                2 => return ('0', 1),
                3 => return ('1', 1),
                _ => panic!("error with {}", sum)
            };
        }

        pub fn add_binary(a: String, b: String) -> String {
            let mut s = String::new();

            let mut len_a = a.len();
            let mut len_b = b.len();

            let mut a_gone = false;
            let mut b_gone = false;
            let mut carry = 0;

            while a_gone != true || b_gone != true || carry == 1{
                let last_a = if a_gone {'0'} else {a.chars().nth(len_a - 1).unwrap()};
                let last_b = if b_gone {'0'} else {b.chars().nth(len_b - 1).unwrap()};
                println!("{} {} and carry={}", last_a, last_b, carry);

                let tuple = Solution::sum(last_a, last_b, carry);
                carry = tuple.1;
                
                println!("Adding {} to {} missing carry = {}", tuple.0, s, carry);
                s.insert(0, tuple.0);
                if len_a > 1{
                    len_a -= 1;
                } else {
                    a_gone = true;
                }
                if len_b >  1{
                    len_b -= 1;
                } else {
                    b_gone = true;
                }

            }
            return s;
        }
}

fn main() {
    let a = "1010".to_string();
    let b = "1011".to_string();

    println!("{:?}", Solution::add_binary(a, b));
}
