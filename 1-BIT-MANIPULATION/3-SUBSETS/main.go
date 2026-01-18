package main

import "fmt"

func subsets(nums []int) [][]int {
	sizeArr := len(nums)
	powerSubsets := 1 << sizeArr

	ans := make([][]int, powerSubsets)

	for i := 0; i < sizeArr; i++ {
		for j := 0; j < powerSubsets; j++ {
			if ((j >> i) & 1) == 1 {
				ans[j] = append(ans[j], nums[i])
			}
		}
	}
	return ans
}

func main() {
	arr := []int{1, 2, 3}
	result := subsets(arr)
	fmt.Println(result)
}
