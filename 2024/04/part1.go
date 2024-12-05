package main

const mulExp = `mul\(([0-9]{1,3}\,[0-9]{1,3})\)`

type Letter struct {
	Value  rune
	IsWord bool
}

func NewLetter(letter rune) Letter {
	return Letter{
		Value:  letter,
		IsWord: false,
	}
}

func part1(data []byte) int {

	puzzle := readPuzzle(data)
	total := completePuzzle(puzzle, "XMAS")
	// printPuzzle(puzzle)

	return total
}
