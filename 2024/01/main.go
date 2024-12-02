package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func readInput(path string) []byte {
	data, err := os.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	return data
}

func createLists(data []byte) ([]int, []int) {
	lines := strings.Split(string(data), "\n")

	list1 := make([]int, len(lines))
	list2 := make([]int, len(lines))

	for i, line := range lines {
		diff := strings.Split(line, " ")
		list1[i], _ = strconv.Atoi(diff[0])
		list2[i], _ = strconv.Atoi(diff[len(diff)-1])
	}

	return list1, list2
}

func part1(data []byte) int {
	list1, list2 := createLists(data)

	sort.Ints(list1)
	sort.Ints(list2)

	total := 0

	for i, l1 := range list1 {
		l2 := list2[i]
		if l1 >= l2 {
			total += l1 - l2
		} else {
			total += l2 - l1
		}
	}

	return total
}

func part2(data []byte) int {
	list1, list2 := createLists(data)

	m := make(map[int]int)

	for _, v := range list2 {
		if _, ok := m[v]; ok {
			m[v] += 1
		} else {
			m[v] = 1
		}
	}

	total := 0

	for _, v := range list1 {
		if i, ok := m[v]; ok {
			total += (v * i)
		}
	}

	return total
}

func main() {

	testInput := readInput("./test.txt")
	input := readInput("./input.txt")

	fmt.Printf("Test Answer: %d\n", part1(testInput))
	fmt.Printf("Part1 Answer: %d\n", part1(input))
	fmt.Println("------------------")
	fmt.Printf("Test Answer: %d\n", part2(testInput))
	fmt.Printf("Part2 Answer: %d\n", part2(input))
}
