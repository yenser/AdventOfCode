package main

import (
	"fmt"
	"strconv"
	"strings"

	input "2022/packages/input"
	stack "2022/packages/stack"
)

func main() {
	list := input.ReadFileToArray("input.txt")

	fmt.Println("-------- Part 1 --------")
	part1(*list)
	fmt.Println("\n\n-------- Part 2 --------")
	part2(*list)

}

func part1(list []string) {

	// stacks := assembleTestCrates() // change me depending on the file
	stacks := assembleInputCrates() // change me depending on the file

	// setup complete

	for _, l := range list {
		words := strings.Split(l, " ")
		n, _ := strconv.Atoi(words[1])
		from, _ := strconv.Atoi(words[3])
		to, _ := strconv.Atoi(words[5])

		fmt.Printf("MOVE %d FROM %d TO %d\n", n, from, to)

		for i := 0; i < n; i++ {
			val := stacks[from-1].Pop()

			if val != nil {
				stacks[to-1].Push(*val)
			} else {
				fmt.Println("ERROR") // grabbing a crate that doesn't exist
			}
		}
	}

	text := ""

	for _, s := range stacks {
		text += string(*s.Pop())
	}

	fmt.Printf("answer: %s\n", text)

}

func part2(list []string) {
	// stacks := assembleTestCrates() // change me depending on the file
	stacks := assembleInputCrates() // change me depending on the file

	// setup complete

	for _, l := range list {
		words := strings.Split(l, " ")
		n, _ := strconv.Atoi(words[1])
		from, _ := strconv.Atoi(words[3])
		to, _ := strconv.Atoi(words[5])

		fmt.Printf("MOVE %d FROM %d TO %d\n", n, from, to)

		tempStack := stack.New[rune]()
		for i := 0; i < n; i++ {
			val := stacks[from-1].Pop()

			if val != nil {
				tempStack.Push(*val)
			} else {
				fmt.Println("ERROR") // grabbing a crate that doesn't exist
			}
		}

		for i := 0; i < n; i++ {
			val := tempStack.Pop()

			stacks[to-1].Push(*val)
		}
	}

	text := ""

	for _, s := range stacks {
		text += string(*s.Pop())
	}

	fmt.Printf("answer: %s\n", text)

}

func assembleTestCrates() []stack.Stack[rune] {
	stacks := make([]stack.Stack[rune], 3)
	for i, _ := range stacks {
		stacks[i] = stack.New[rune]()
	}

	// 1
	stacks[0].Push('Z')
	stacks[0].Push('N')

	// 2
	stacks[1].Push('M')
	stacks[1].Push('C')
	stacks[1].Push('D')

	// 3
	stacks[2].Push('P')

	return stacks
}

func assembleInputCrates() []stack.Stack[rune] {
	stacks := make([]stack.Stack[rune], 9)
	for i, _ := range stacks {
		stacks[i] = stack.New[rune]()
	}

	// 1
	stacks[0].Push('P')
	stacks[0].Push('F')
	stacks[0].Push('M')
	stacks[0].Push('Q')
	stacks[0].Push('W')
	stacks[0].Push('G')
	stacks[0].Push('R')
	stacks[0].Push('T')

	// 2
	stacks[1].Push('H')
	stacks[1].Push('F')
	stacks[1].Push('R')

	// 3
	stacks[2].Push('P')
	stacks[2].Push('Z')
	stacks[2].Push('R')
	stacks[2].Push('V')
	stacks[2].Push('G')
	stacks[2].Push('H')
	stacks[2].Push('S')
	stacks[2].Push('D')

	// 4
	stacks[3].Push('Q')
	stacks[3].Push('H')
	stacks[3].Push('P')
	stacks[3].Push('B')
	stacks[3].Push('F')
	stacks[3].Push('W')
	stacks[3].Push('G')

	// 5
	stacks[4].Push('P')
	stacks[4].Push('S')
	stacks[4].Push('M')
	stacks[4].Push('J')
	stacks[4].Push('H')

	// 6
	stacks[5].Push('M')
	stacks[5].Push('Z')
	stacks[5].Push('T')
	stacks[5].Push('H')
	stacks[5].Push('S')
	stacks[5].Push('R')
	stacks[5].Push('P')
	stacks[5].Push('L')

	// 7
	stacks[6].Push('P')
	stacks[6].Push('T')
	stacks[6].Push('H')
	stacks[6].Push('N')
	stacks[6].Push('M')
	stacks[6].Push('L')

	// 8
	stacks[7].Push('F')
	stacks[7].Push('D')
	stacks[7].Push('Q')
	stacks[7].Push('R')

	//9
	stacks[8].Push('D')
	stacks[8].Push('S')
	stacks[8].Push('C')
	stacks[8].Push('N')
	stacks[8].Push('L')
	stacks[8].Push('P')
	stacks[8].Push('H')

	return stacks
}
