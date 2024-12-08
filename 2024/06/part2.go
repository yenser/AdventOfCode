package main

import "fmt"

func checkForLoop(grid Grid, coord Coord) bool {
	steps := 0
	for !grid.IsOutOfBounds(coord.Y, coord.X) {
		if grid.IsStuckInLoop(coord) {
			// grid.Print(coord)
			// fmt.Println("steps", steps)
			return true
		}

		if !coord.Move(grid) {
			coord.TurnRight(grid)
		}

		steps++
	}

	return false
}

func copyGrid(grid Grid) Grid {
	copy := make(Grid, len(grid))
	for y, row := range grid {
		copy[y] = make([]Tile, len(row))
		for x, tile := range row {
			if tile.HasObstacle {
				copy[y][x] = NewTile(true)
			} else {
				copy[y][x] = NewTile(false)
			}
		}
	}

	return copy
}

func part2(data []byte) int {
	coordStart, grid := ReadGrid(data)

	coord := coordStart

	fmt.Printf("Grid size | Y:%d X:%d\n", len(grid), len(grid[0]))

	for !grid.IsOutOfBounds(coord.Y, coord.X) {
		if !coord.Move(grid) {
			coord.TurnRight(grid)
		}
	}

	total := 0

	for y, row := range grid {
		for x, tile := range row {
			if tile.Touched {
				gridCopy := copyGrid(grid)
				gridCopy[y][x].HasObstacle = true
				coordCopy := coordStart
				if checkForLoop(gridCopy, coordCopy) {
					total++
				}
			}
		}
	}

	return total
}
