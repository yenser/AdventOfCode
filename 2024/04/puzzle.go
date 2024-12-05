package main

import (
	"fmt"
	"strings"
)

func readPuzzle(data []byte) [][]Letter {
	lines := strings.Split(string(data), "\n")

	height := len(lines)
	width := len(lines[0])

	puzzle := make([][]Letter, height)

	for y, row := range lines {
		puzzle[y] = make([]Letter, width)

		for x, letter := range row {
			puzzle[y][x] = NewLetter(letter)
		}
	}

	return puzzle
}

func printPuzzle(puzzle [][]Letter) {
	for _, row := range puzzle {
		for _, letter := range row {
			if letter.IsWord {
				fmt.Printf("%c", letter.Value)
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Println()
	}
}

func completePuzzle(puzzle [][]Letter, word string) int {
	totalWords := 0
	firstLetterOfWord := word[0]

	for y, row := range puzzle {
		for x, letter := range row {
			if letter.Value == rune(firstLetterOfWord) {
				totalWords += checkForWord(puzzle, x, y, word)
			}
		}
	}

	return totalWords
}

func completeXMasPuzzle(puzzle [][]Letter) int {
	totalWords := 0

	for y, row := range puzzle {
		for x, letter := range row {
			if letter.Value == 'A' {
				if checkForXMas(puzzle, x, y) {
					totalWords += 1
				}
			}
		}
	}

	return totalWords
}

func checkForWord(puzzle [][]Letter, x, y int, word string) int {
	total := 0
	if checkUp(puzzle, x, y, word) {
		total += 1
	}
	if checkDown(puzzle, x, y, word) {
		total += 1
	}
	if checkLeft(puzzle, x, y, word) {
		total += 1
	}
	if checkRight(puzzle, x, y, word) {
		total += 1
	}
	if checkUpLeft(puzzle, x, y, word) {
		total += 1
	}
	if checkUpRight(puzzle, x, y, word) {
		total += 1
	}
	if checkDownLeft(puzzle, x, y, word) {
		total += 1
	}
	if checkDownRight(puzzle, x, y, word) {
		total += 1
	}

	return total
}

func checkUp(puzzle [][]Letter, x, y int, word string) bool {
	if y < len(word)-1 {
		return false
	}

	for i, letter := range word {
		if puzzle[y-i][x].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y-i][x].IsWord = true
	}

	return true
}

func checkDown(puzzle [][]Letter, x, y int, word string) bool {
	if len(puzzle)-y < len(word) {
		return false
	}

	for i, letter := range word {
		if puzzle[y+i][x].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y+i][x].IsWord = true
	}

	return true
}

func checkLeft(puzzle [][]Letter, x, y int, word string) bool {
	if x < len(word)-1 {
		return false
	}

	for i, letter := range word {
		if puzzle[y][x-i].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y][x-i].IsWord = true
	}

	return true
}

func checkRight(puzzle [][]Letter, x, y int, word string) bool {
	if len(puzzle[y])-x < len(word) {
		return false
	}

	for i, letter := range word {
		if puzzle[y][x+i].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y][x+i].IsWord = true
	}

	return true
}

func checkUpLeft(puzzle [][]Letter, x, y int, word string) bool {
	if y < len(word)-1 || x < len(word)-1 {
		return false
	}

	for i, letter := range word {
		if puzzle[y-i][x-i].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y-i][x-i].IsWord = true
	}

	return true
}

func checkUpRight(puzzle [][]Letter, x, y int, word string) bool {
	if y < len(word)-1 || len(puzzle[y])-x < len(word) {
		return false
	}

	for i, letter := range word {
		if puzzle[y-i][x+i].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y-i][x+i].IsWord = true
	}

	return true
}

func checkDownLeft(puzzle [][]Letter, x, y int, word string) bool {
	if len(puzzle)-y < len(word) || x < len(word)-1 {
		return false
	}

	for i, letter := range word {
		if puzzle[y+i][x-i].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y+i][x-i].IsWord = true
	}

	return true
}

func checkDownRight(puzzle [][]Letter, x, y int, word string) bool {
	if len(puzzle)-y < len(word) || len(puzzle[y])-x < len(word) {
		return false
	}

	for i, letter := range word {
		if puzzle[y+i][x+i].Value != letter {
			return false
		}
	}

	// debug print
	for i := range word {
		puzzle[y+i][x+i].IsWord = true
	}

	return true
}

func checkForXMas(puzzle [][]Letter, x, y int) bool {
	if y == 0 || y == len(puzzle)-1 || x == 0 || x == len(puzzle[y])-1 {
		return false
	}

	if puzzle[y-1][x-1].Value == 'M' {
		if puzzle[y+1][x+1].Value != 'S' {
			return false
		}
	} else if puzzle[y-1][x-1].Value == 'S' {
		if puzzle[y+1][x+1].Value != 'M' {
			return false
		}
	} else {
		return false // not a valid letter
	}

	if puzzle[y+1][x-1].Value == 'M' {
		if puzzle[y-1][x+1].Value != 'S' {
			return false
		}
	} else if puzzle[y+1][x-1].Value == 'S' {
		if puzzle[y-1][x+1].Value != 'M' {
			return false
		}
	} else {
		return false // not a valid letter
	}

	puzzle[y][x].IsWord = true
	puzzle[y-1][x-1].IsWord = true
	puzzle[y-1][x+1].IsWord = true
	puzzle[y+1][x-1].IsWord = true
	puzzle[y+1][x+1].IsWord = true

	return true
}
