package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

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

func getLetterValue(val rune) int {
	intVal := int(val)
	if intVal >= 97 {
		return intVal - 96
	} else {
		return intVal - 38
	}
}

func part1(list []string) {
	total := 0
	for _, l := range list {
		var string1, string2 string

		half := len(l) / 2

		string1 = l[0:half]
		string2 = l[half:]

		char := getDupFromTwoStrings(string1, string2)

		total += getLetterValue(*char)
	}

	fmt.Printf("Total: %d\n", total)
}

func part2(list []string) {
	total := 0
	var elves []string = make([]string, 3)
	for i, l := range list {
		k := i % 3 // every third

		elves[k] = l

		if k == 2 {
			char := getDupFromThreeStrings(elves[0], elves[1], elves[2])

			fmt.Printf("%d(%c)\n", getLetterValue(*char), *char)
			total += getLetterValue(*char)
		}
	}

	fmt.Printf("Total: %d\n", total)
}

func getDupFromTwoStrings(string1, string2 string) *rune {
	for i := 0; i < len(string1); i++ {
		for j := 0; j < len(string2); j++ {
			if string1[i] == string2[j] {
				char := rune(string1[i])

				fmt.Printf("%s - %s | %d(%c)\n", string1, string2, getLetterValue(char), char)

				return &char
			}
		}
	}

	return nil
}

func getDupFromThreeStrings(string1, string2, string3 string) *rune {
	for i := 0; i < len(string1); i++ {
		for j := 0; j < len(string2); j++ {
			if string1[i] == string2[j] {
				for k := 0; k < len(string3); k++ {
					if string2[j] == string3[k] {
						char := rune(string1[i])
						return &char
					}
				}
			}
		}
	}

	return nil
}
