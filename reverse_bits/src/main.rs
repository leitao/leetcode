struct Solution;

impl Solution {
        pub fn reverse_bits(x: u32) -> u32 {
	    let mut result: u32 = 0;
	    for i in 0..32 {
		result |=  (((1 << i) & x) > 0) as u32;

		if i < 31 {
		    result <<= 1;
		}
	    }

	    return result;
	}
}


fn main() {
    // let input = 0b0101011;
    // let input = 43261596;
    let input = 3221225471;
    let ret = Solution::reverse_bits(input);


    println!("{:b} ({})  is {:b} ({})", input, input, ret, ret);
}
