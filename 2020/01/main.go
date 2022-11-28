package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
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
	lines := readFileToArray("input.txt")

	report := make([]int, len(*lines))

	var err error
	for i, line := range *lines {
		if report[i], err = strconv.Atoi(line); err != nil {
			log.Fatal(err)
		}
	}
	// part1(report)
	part2(report)

}

func part1(report []int) {
	var answerFound bool = false
	var answerA, answerB int

	for i, a := range report {
		for b := i + 1; b < len(report); b++ {
			sum := a + report[b]
			fmt.Printf("%d + %d = %d\n", a, report[b], sum)

			if sum == 2020 {
				answerFound = true
				answerA = a
				answerB = report[b]
				break
			}
		}

		if answerFound {
			break
		}
	}

	fmt.Printf("Answer: %d * %d = %d\n", answerA, answerB, answerA*answerB)
}

func part2(report []int) {
	var answerFound bool = false
	var answerA, answerB, answerC int

	for i, a := range report {
		for b := i + 1; b < len(report); b++ {
			for x, c := range report {
				if x == i || x == b {
					continue
				}

				sum := a + report[b] + c
				fmt.Printf("%d + %d + %d = %d\n", a, report[b], c, sum)

				if sum == 2020 {
					answerFound = true
					answerA = a
					answerB = report[b]
					answerC = c
					break
				}
			}

			if answerFound {
				break
			}
		}

		if answerFound {
			break
		}
	}

	fmt.Printf("Answer: %d * %d * %d = %d\n", answerA, answerB, answerC, answerA*answerB*answerC)
}
