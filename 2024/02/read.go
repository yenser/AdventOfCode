package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func readInput(path string) []byte {
	data, err := os.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	return data
}

func createLists(data []byte) [][]int {
	lines := strings.Split(string(data), "\n")

	list := make([][]int, len(lines))

	for i, line := range lines {
		diff := strings.Split(line, " ")
		list[i] = make([]int, len(diff))
		for j, v := range diff {
			list[i][j], _ = strconv.Atoi(v)
		}
	}

	return list
}
