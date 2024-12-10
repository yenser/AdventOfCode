package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Calibration struct {
	Answer    int
	Operators []int
}

func ReadProblems(data []byte) []Calibration {
	lines := strings.Split(string(data), "\n")
	calibrations := make([]Calibration, len(lines))

	for i, line := range lines {
		problem := strings.Split(line, ": ")
		calibrations[i].Answer, _ = strconv.Atoi(problem[0])
		valuesStr := strings.Split(problem[1], " ")
		values := make([]int, len(valuesStr))
		for j, v := range valuesStr {
			values[j], _ = strconv.Atoi(v)
		}

		calibrations[i].Operators = values
	}

	return calibrations
}

func (c Calibration) Print() {
	fmt.Printf("%g: ", c.Answer)
	for _, v := range c.Operators {
		fmt.Printf("%d ", v)
	}
	fmt.Println()
}

func checkCalibration(expectedAnswer int, currentAnswer int, arr []int, idx int) bool {
	if idx >= len(arr) {
		// fmt.Printf("%g == %g\n", expectedAnswer, currentAnswer)
		return expectedAnswer == currentAnswer
	}

	nextIdx := idx + 1
	if idx == 0 {
		return checkCalibration(expectedAnswer, arr[idx], arr, nextIdx)
	}

	val1 := currentAnswer
	val2 := arr[idx]
	// fmt.Printf("%d + %d\n", val1, val2)
	if checkCalibration(expectedAnswer, val1+val2, arr, nextIdx) { // addition
		return true
	} else if checkCalibration(expectedAnswer, val1*val2, arr, nextIdx) { // multiplication
		return true
	} else {
		return false
	}
}

func part1(data []byte) int {
	total := 0

	calibrations := ReadProblems(data)

	for _, c := range calibrations {
		if checkCalibration(c.Answer, 0, c.Operators, 0) {
			// c.Print()
			total += c.Answer
		}
	}

	return total
}
