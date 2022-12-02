package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

// A = Rock
// B = Paper
// C = Scissors

// X = Rock?
// Y = Paper?
// Z = Scissors?

const win int = 6
const tie int = 3
const paper int = 2
const rock int = 1
const scissors = 3

var mapping = map[byte]string{
	'A': "rock",
	'X': "rock",
	'B': "paper",
	'Y': "paper",
	'C': "scissors",
	'Z': "scissors",
}

var playPoints = map[byte]int{
	'X': 1,
	'Y': 2,
	'Z': 3,
}

func readFileToArray(fileName string) *[]string {
	lines := []string{}

	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return &lines
}

func main() {
	list := readFileToArray("input.txt")

	fmt.Println("-------- Part 1 --------")
	part1(*list)
	fmt.Println("\n\n-------- Part 2 --------")
	part2(*list)

}

func part1(list []string) {
	total := 0
	for _, l := range list {
		var opp = l[0]
		var player = l[2]

		var points = playPoints[player]

		if (opp == 'A' && player == 'X') || (opp == 'B' && player == 'Y') || (opp == 'C' && player == 'Z') {
			points += tie
		} else if (opp == 'A' && player == 'Y') || (opp == 'B' && player == 'Z') || (opp == 'C' && player == 'X') {
			points += win
		}

		fmt.Printf("(%c)%s v (%c)%s = %d\n", opp, mapping[opp], player, mapping[player], points)
		total += points
	}
	fmt.Printf("total: %d\n", total)
}

func part2(list []string) {
	total := 0
	for _, l := range list {
		var opp = l[0]
		var player = l[2]

		var points = 0

		if player == 'Y' {
			points += tie
		} else if player == 'Z' {
			points += win
		}

		if opp == 'A' {
			if player == 'X' {
				points += scissors
			} else if player == 'Y' {
				points += rock
			} else {
				points += paper
			}
		} else if opp == 'B' {
			if player == 'X' {
				points += rock
			} else if player == 'Y' {
				points += paper
			} else {
				points += scissors
			}
		} else if opp == 'C' {
			if player == 'X' {
				points += paper
			} else if player == 'Y' {
				points += scissors
			} else {
				points += rock
			}
		}

		// fmt.Printf("(%c)%s v (%c)%s = %d\n", opp, mapping[opp], player, mapping[player], points)
		total += points
	}
	fmt.Printf("total: %d\n", total)
}
