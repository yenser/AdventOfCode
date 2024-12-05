package main

func part2(data []byte) int {

	puzzle := readPuzzle(data)
	total := completeXMasPuzzle(puzzle)
	printPuzzle(puzzle)

	return total
}
