package main

import (
	"fmt"
	"strings"
)

func ReadGrid(data []byte) (Coord, Grid) {
	lines := strings.Split(string(data), "\n")

	grid := make(Grid, len(lines))

	coord := Coord{}

	for y, line := range lines {
		grid[y] = make([]Tile, len(line))
		for x, v := range line {
			if v == '^' {
				coord.X = x
				coord.Y = y
				coord.SetDirection("up")
				grid[y][x] = NewTile(false)
				grid[y][x].Touch(coord.DirectionY, coord.DirectionX)
			} else if v == '#' {
				grid[y][x] = NewTile(true)
			} else {
				grid[y][x] = NewTile(false)
			}
		}
	}

	return coord, grid
}

func part1(data []byte) int {
	coord, grid := ReadGrid(data)

	fmt.Printf("Grid size | Y:%d X:%d\n", len(grid), len(grid[0]))

	for !grid.IsOutOfBounds(coord.Y, coord.X) {

		// grid.Print(coord)

		if !coord.Move(grid) {
			coord.TurnRight(grid)
		}

	}

	return grid.CountTouches()
}
