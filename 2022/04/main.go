package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
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

func assignmentToSections(assignment string) []int {
	sec := make([]int, 2)

	loc := strings.Split(assignment, "-")
	sec[0], _ = strconv.Atoi(loc[0])
	sec[1], _ = strconv.Atoi(loc[1])

	return sec
}

func part1(list []string) {
	total := 0
	for _, l := range list {
		assignments := strings.Split(l, ",")
		section1 := assignmentToSections(assignments[0])
		section2 := assignmentToSections(assignments[1])

		if section1[0] <= section2[0] {
			if section1[1] >= section2[1] {
				total++
				fmt.Printf("%v %v *\n", section1, section2)

				continue
			}
		}

		if section2[0] <= section1[0] {
			if section2[1] >= section1[1] {
				total++
				fmt.Printf("%v %v *\n", section1, section2)

				continue
			}
		}

		fmt.Printf("%v %v\n", section1, section2)
	}

	fmt.Printf("Total: %d\n", total)
}

func part2(list []string) {
	total := 0
	for _, l := range list {
		assignments := strings.Split(l, ",")
		section1 := assignmentToSections(assignments[0])
		section2 := assignmentToSections(assignments[1])

		if section1[0] >= section2[0] && section1[0] <= section2[1] {
			total++
			fmt.Printf("%v %v *\n", section1, section2)
		} else if section1[1] >= section2[0] && section1[1] <= section2[1] {
			total++
			fmt.Printf("%v %v *\n", section1, section2)
		} else if section2[0] >= section1[0] && section2[0] <= section1[1] {
			total++
			fmt.Printf("%v %v *\n", section1, section2)
		} else if section2[1] >= section1[0] && section2[1] <= section1[1] {
			total++
			fmt.Printf("%v %v *\n", section1, section2)
		} else {
			fmt.Printf("%v %v\n", section1, section2)
		}
		//[68 87] [8 67] *

	}

	fmt.Printf("Total: %d\n", total)
}
