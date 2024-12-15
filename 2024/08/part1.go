package main

import (
	"fmt"
	"strings"
)

func parseMap(data []byte) (antennaMap map[rune][]Coord, xSize, ySize int) {
	lines := strings.Split(string(data), "\n")

	ySize = len(lines)

	antennaMap = make(map[rune][]Coord)

	for y, row := range lines {
		xSize = len(row)
		for x, char := range row {
			if char == '.' {
				continue
			}

			if _, ok := antennaMap[char]; ok {
				antennaMap[char] = append(antennaMap[char], NewCoord(x, y))
			} else {
				antennaMap[char] = []Coord{NewCoord(x, y)}
			}
		}
	}

	return
}

func printMap(antennaMap map[rune][]Coord, antiNodeMap []Coord, xSize, ySize int) {
	grid := make([][]rune, ySize)
	for y := range grid {
		grid[y] = make([]rune, xSize)
		for x := range grid[y] {
			grid[y][x] = '.'
		}
	}

	for k, v := range antennaMap {
		for _, c := range v {
			grid[c.Y][c.X] = k
		}
	}

	for _, c := range antiNodeMap {
		if !isOutOfBounds(xSize, ySize, c) {
			if grid[c.Y][c.X] == '.' {
				grid[c.Y][c.X] = '#'
			}
		}
	}

	for _, row := range grid {
		for _, v := range row {
			fmt.Printf(" %s ", string(v))
		}
		fmt.Println()
	}
}

func findAntiNodes(c []Coord, xSize, ySize int) []Coord {
	antiNodes := []Coord{}
	for i := 0; i < len(c)-1; i++ {
		for j := i + 1; j < len(c); j++ {
			c1 := c[i]
			c2 := c[j]

			an1, an2 := findAntiNodePair(c1, c2)

			antiNodes = append(antiNodes, an1, an2)
		}
	}

	return antiNodes
}

func findAntiNodePair(c1, c2 Coord) (n1, n2 Coord) {
	xDiff := c2.X - c1.X
	yDiff := c2.Y - c1.Y

	fmt.Printf("%d,%d | %d,%d | %d/%d\n", c1.X, c1.Y, c2.X, c2.Y, xDiff, yDiff)

	n1 = NewCoord(c1.X-xDiff, c1.Y-yDiff)
	n2 = NewCoord(c2.X+xDiff, c2.Y+yDiff)
	return
}

func getUniqueNodes(nodes []Coord) []Coord {
	nodesMap := make(map[Coord]int)

	for _, node := range nodes {
		if _, ok := nodesMap[node]; ok {
			nodesMap[node] += 1
		} else {
			nodesMap[node] = 0
		}
	}

	unique := []Coord{}
	for k, _ := range nodesMap {
		unique = append(unique, k)
	}

	return unique
}

func getInBoundNodes(nodes []Coord, xSize, ySize int) []Coord {
	inBoundNodes := make([]Coord, 0)
	for _, node := range nodes {
		if !(node.X < 0 || node.Y < 0 || node.X >= xSize || node.Y >= ySize) {
			inBoundNodes = append(inBoundNodes, node)
		}
	}

	return inBoundNodes
}

func isOutOfBounds(xSize, ySize int, c Coord) bool {
	if c.X < 0 || c.X > xSize-1 {
		return true
	} else if c.Y < 0 || c.Y > ySize-1 {
		return true
	}
	return false
}

func part1(data []byte) int {
	total := 0

	antennaMap, xSize, ySize := parseMap(data)

	antiNodes := []Coord{}

	for k, v := range antennaMap {
		nodes := findAntiNodes(v, xSize, ySize)
		fmt.Printf("%s: found %d anti nodes\n", string(k), len(nodes))
		antiNodes = append(antiNodes, nodes...)

	}

	uniqueAntiNodes := getUniqueNodes(antiNodes)
	inBoundNodes := getInBoundNodes(uniqueAntiNodes, xSize, ySize)
	fmt.Println("%2v\n", inBoundNodes)
	total = len(inBoundNodes)

	// fmt.Printf("%d, %d: %v\n", xSize, ySize, antennaMap)

	printMap(antennaMap, antiNodes, xSize, ySize)

	return total
}
