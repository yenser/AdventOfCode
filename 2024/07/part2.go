package main

import (
	"fmt"
	"strconv"
)

func checkCalibrationPt2(expectedAnswer int, currentAnswer int, arr []int, idx int) bool {
	if idx >= len(arr) {
		// fmt.Printf("%g == %g\n", expectedAnswer, currentAnswer)
		return expectedAnswer == currentAnswer
	}

	nextIdx := idx + 1
	if idx == 0 {
		return checkCalibrationPt2(expectedAnswer, arr[idx], arr, nextIdx)
	}

	val1 := currentAnswer
	val2 := arr[idx]
	// fmt.Printf("%d + %d\n", val1, val2)
	if checkCalibrationPt2(expectedAnswer, val1+val2, arr, nextIdx) { // addition
		return true
	} else if checkCalibrationPt2(expectedAnswer, val1*val2, arr, nextIdx) { // multiplication
		return true
	}

	concat, _ := strconv.Atoi(fmt.Sprintf("%d%d", val1, val2))
	if checkCalibrationPt2(expectedAnswer, concat, arr, nextIdx) { // concatt
		return true
	}

	return false
}

func part2(data []byte) int {
	total := 0

	calibrations := ReadProblems(data)

	for _, c := range calibrations {
		if checkCalibrationPt2(c.Answer, 0, c.Operators, 0) {
			// c.Print()
			total += c.Answer
		}
	}

	return total
}
