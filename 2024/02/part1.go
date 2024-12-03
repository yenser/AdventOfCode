package main

import "math"

func validateReport(report []int) bool {

	if len(report) < 2 {
		return false
	}

	isIncreasing := report[0] < report[len(report)-1]

	for i := 1; i < len(report); i++ {
		diff := report[i-1] - report[i]
		if diff == 0 || math.Abs(float64(diff)) > 3 {
			return false
		} else if isIncreasing && diff > 0 { // should increase but difference is positive (which means decreasing)
			return false
		} else if !isIncreasing && diff < 0 { // should decrease but difference is negative (which means increasing)
			return false
		}
	}

	return true
}

func part1(data []byte) int {
	reports := createLists(data)

	total := 0
	for _, r := range reports {
		if validateReport(r) {
			total += 1
		}
	}

	return total
}
