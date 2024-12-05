package main

import (
	"fmt"
)

func main() {

	testInput := readInput("./test.txt")
	input := readInput("./input.txt")

	fmt.Printf("Test Answer: %d\n", part1(testInput))
	fmt.Printf("Part1 Answer: %d\n", part1(input))
	fmt.Println("------------------")
	fmt.Printf("Test Answer: %d\n", part2(testInput))
	fmt.Printf("Part2 Answer: %d\n", part2(input))
}
