package main

import "fmt"

type Solution struct{}

func (s Solution) CountOdds(low int, high int) int {
	return (high+1)/2 - low/2
}

func main() {
	s := Solution{}
	fmt.Println(s.CountOdds(3, 7))
}
