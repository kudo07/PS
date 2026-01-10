package main

import "fmt"

func minBitFlips(start int, goal int) int {
	totalAns := start ^ goal
	var count int = 0
	for totalAns > 0 {
		count += totalAns & 1
		totalAns >>= 1
	}
	return count
}

func main() {
	fmt.Println(minBitFlips(10, 7))
}
