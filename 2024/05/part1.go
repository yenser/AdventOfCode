package main

import (
	"slices"
	"strconv"
	"strings"
)

func getRulesAndPages(data []byte) (rules map[int][]int, updates [][]int) {
	lines := strings.Split(string(data), "\n")

	seperatorIdx := slices.IndexFunc(lines, func(s string) bool { return s == "" })
	rules = map[int][]int{}

	// get rules
	for i := 0; i < seperatorIdx; i++ {
		ruleText := strings.Split(lines[i], "|")
		leftVal, _ := strconv.Atoi(ruleText[0])
		rightVal, _ := strconv.Atoi(ruleText[1])
		if _, ok := rules[leftVal]; ok {
			rules[leftVal] = append(rules[leftVal], rightVal)
		} else {
			rules[leftVal] = []int{rightVal}
		}
	}

	numOfPages := len(lines) - seperatorIdx - 1
	updates = make([][]int, numOfPages)

	for i := 0; i < numOfPages; i++ {
		pageIdx := i + seperatorIdx + 1
		pageText := strings.Split(lines[pageIdx], ",")

		updates[i] = make([]int, len(pageText))

		for j, page := range pageText {
			updates[i][j], _ = strconv.Atoi(page)
		}

	}

	// fmt.Printf("%v\n", rules)
	// fmt.Printf("%v\n", updates)
	return
}
func validateUpdate(update []int, rules map[int][]int) bool {
	for i := len(update) - 1; i >= 0; i-- {
		v, ok := rules[update[i]]

		if !ok {
			continue
		}

		if !validatePage(update, i, v) {
			return false
		}
	}

	return true
}

func validatePage(update []int, pageIdx int, rule []int) bool {
	for j := 0; j < pageIdx; j++ {
		if slices.Contains(rule, update[j]) {
			return false
		}
	}
	return true
}

func part1(data []byte) int {
	rules, updates := getRulesAndPages(data)
	total := 0

	for _, pages := range updates {
		if validateUpdate(pages, rules) {
			total += pages[len(pages)/2]
		}
	}

	return total
}
