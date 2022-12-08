package main

import (
	"fmt"

	input "2022/packages/input"
)

const totalAvailableSpace = 70_000_000
const spaceNeeded = 30_000_000

var total = 0
var deleteDir = []int{}

func main() {
	list := input.ReadFileToArray("input.txt")

	fmt.Println("-------- Part 1 --------")
	part1(*list)
	fmt.Println("\n\n-------- Part 2 --------")
	part2(*list)

}

func part1(strList []string) {

	var grid = makeGrid(strList)
	printGrid(grid)

	var edgeTrees = (len(grid) * 4) - 4
	var visibleInteriorTree = 0

	for y, row := range grid {

		if y == 0 || y == (len(strList)-1) { // ignore edges
			continue
		}

		for x, _ := range row {
			if x == 0 || x == (len(row)-1) { // ignore edges
				continue
			}

			if checkX(row, x) || checkY(grid, y, x) {
				visibleInteriorTree += 1
			}
		}
	}

	fmt.Println("Edge Trees: ", edgeTrees)
	fmt.Println("Visible Interior Trees: ", visibleInteriorTree)
	fmt.Println("Total: ", edgeTrees+visibleInteriorTree)
}

func part2(strList []string) {
	var grid = makeGrid(strList)
	printGrid(grid)

	var bestViewX = 0
	var bestViewY = 0
	var bestViewScore = 0

	for y, row := range grid {

		if y == 0 || y == (len(strList)-1) { // ignore edges
			continue
		}

		for x, _ := range row {
			if x == 0 || x == (len(row)-1) { // ignore edges
				continue
			}

			left := viewScoreLeft(row, x)
			right := viewScoreRight(row, x)
			up := viewScoreUp(grid, y, x)
			down := viewScoreDown(grid, y, x)

			score := left * right * up * down

			if score > bestViewScore {
				bestViewX = x
				bestViewY = y
				bestViewScore = score
			}
		}
	}

	fmt.Println("Best View Score: ", bestViewScore)
	fmt.Println("Best View X: ", bestViewX)
	fmt.Println("Best View Y: ", bestViewY)
}

func makeGrid(strList []string) [][]int {
	var size = len(strList)
	grid := make([][]int, size)
	for y, l := range strList {
		grid[y] = make([]int, size)

		for x, c := range l {
			grid[y][x] = int(c - '0')
		}
	}

	return grid
}

func printGrid(grid [][]int) {
	for _, row := range grid {
		fmt.Printf("%v\n", row)
	}
}

func checkX(row []int, valX int) bool {
	left := true
	right := true

	for i := 0; i < valX; i++ {
		if row[i] >= row[valX] {
			left = false
		}
	}

	for i := valX + 1; i < len(row); i++ {
		if row[i] >= row[valX] {
			right = false
		}
	}

	return left || right
}

func checkY(grid [][]int, valY, valX int) bool {
	up := true
	down := true
	for i := 0; i < valY; i++ {
		if grid[i][valX] >= grid[valY][valX] {
			up = false
		}
	}

	for i := valY + 1; i < len(grid); i++ {
		if grid[i][valX] >= grid[valY][valX] {
			down = false
		}
	}

	return up || down
}

func viewScoreLeft(row []int, valX int) int {
	trees := 0

	for i := valX - 1; i >= 0; i-- {
		if row[i] >= row[valX] {
			trees++
			return trees
		} else {
			trees++
		}
	}

	return trees
}

func viewScoreRight(row []int, valX int) int {
	trees := 0

	for i := valX + 1; i < len(row); i++ {
		if row[i] >= row[valX] {
			trees++
			return trees
		} else {
			trees++
		}
	}

	return trees
}

func viewScoreUp(grid [][]int, valY, valX int) int {
	trees := 0

	for i := valY - 1; i >= 0; i-- {
		if grid[i][valX] >= grid[valY][valX] {
			trees++
			return trees
		} else {
			trees++
		}
	}

	return trees
}

func viewScoreDown(grid [][]int, valY, valX int) int {
	trees := 0

	for i := valY + 1; i < len(grid); i++ {
		if grid[i][valX] >= grid[valY][valX] {
			trees++
			return trees
		} else {
			trees++
		}
	}

	return trees
}
