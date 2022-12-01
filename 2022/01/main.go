package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
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
	list := readFileToArray("input.txt")

	fmt.Println("-------- Part 1 --------")
	part1(*list)
	fmt.Println("\n\n-------- Part 2 --------")
	part2(*list)

}

func parseElfFood(list []string) []int {
	var elfPacks []int

	var v int = 0
	for _, a := range list {

		if a == "" {
			elfPacks = append(elfPacks, v)
			v = 0 // reset counter
			continue
		}

		cal, err := strconv.Atoi(a)
		if err != nil {
			log.Panic(err)
		}

		v += cal
	}

	return elfPacks
}

func part1(list []string) {

	var largestPack int = 0
	var elfPacks []int

	var v int = 0
	for _, a := range list {

		if a == "" {
			elfPacks = append(elfPacks, v)
			if v > largestPack {
				largestPack = v
			}
			v = 0 // reset counter
			continue
		}

		cal, err := strconv.Atoi(a)
		if err != nil {
			log.Panic(err)
		}

		v += cal
	}

	// fmt.Printf("%v\n", elfPacks)
	fmt.Printf("Largest Pack: %d\n", largestPack)
}

func part2(list []string) {
	packs := parseElfFood(list)

	sort.Sort(sort.Reverse(sort.IntSlice(packs)))

	fmt.Printf("1st: %d\n", packs[0])
	fmt.Printf("2st: %d\n", packs[1])
	fmt.Printf("3st: %d\n", packs[2])
	fmt.Printf("total: %d\n", packs[0]+packs[1]+packs[2])
}
