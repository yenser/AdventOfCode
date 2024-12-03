package main

func RemoveIndex(s []int, index int) []int {
	return append(s[:index], s[index+1:]...)
}

// brute force ew
func validateReportWithTolerate(report []int) bool {

	for i := 0; i < len(report); i++ {
		c := make([]int, len(report))
		copy(c, report)
		if validateReport(RemoveIndex(c, i)) {
			return true
		}
	}

	return false
}

func part2(data []byte) int {
	reports := createLists(data)

	total := 0
	for _, r := range reports {
		if validateReportWithTolerate(r) {
			total += 1
		}
	}

	return total
}
