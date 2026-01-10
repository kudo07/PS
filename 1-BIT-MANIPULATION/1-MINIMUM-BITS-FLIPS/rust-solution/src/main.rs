struct Solution;
impl Solution {     
     pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
            let mut total_ans=start ^ goal;
            let mut count = 0;
             while total_ans !=0 {
                count+=total_ans & 1;
                total_ans>>=1;
             }
             count
        }
 }

fn main() {
    let start: i32 = 10;
    let goal: i32 = 7;

    let result = Solution::min_bit_flips(start, goal);
    println!("{}", result);
}