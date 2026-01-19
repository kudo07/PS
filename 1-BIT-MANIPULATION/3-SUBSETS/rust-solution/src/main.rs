struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let size_arr = nums.len();
        let power_subsets = 1 << size_arr;
        let mut ans: Vec<Vec<i32>> = vec![Vec::new(); power_subsets];
        // vec![value;count]
        // here Vec::new() create the empty array
        for i in 0..size_arr {
            for j in 0..power_subsets {
                if (j >> i) & 1 == 1 {
                    ans[j].push(nums[i])
                }
            }
        }
        ans
    }
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3];
    let result = Solution::subsets(arr);
    println!("{:?}", result);
}
