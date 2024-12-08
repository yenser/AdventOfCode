package main

import "fmt"

type Grid [][]Tile

type Tile struct {
	HasObstacle  bool
	Touched      bool
	TouchedLeft  bool
	TouchedRight bool
	TouchedUp    bool
	TouchedDown  bool
}

func NewTile(hasObstacle bool) Tile {
	return Tile{
		HasObstacle:  hasObstacle,
		Touched:      false,
		TouchedLeft:  false,
		TouchedRight: false,
		TouchedUp:    false,
		TouchedDown:  false,
	}
}

func (t *Tile) Touch(directionY, directionX int) {
	if directionX < 0 {
		t.TouchedLeft = true
	} else if directionX > 0 {
		t.TouchedRight = true
	} else if directionY < 0 {
		t.TouchedUp = true
	} else if directionY > 0 {
		t.TouchedDown = true
	}

	t.Touched = true
}

func (grid Grid) Print(coord Coord) {
	for y, row := range grid {
		for x, tile := range row {
			if coord.X == x && coord.Y == y {
				fmt.Printf(coord.getDirectionCharacter())
			} else if tile.HasObstacle {
				fmt.Printf("#")
			} else if tile.Touched {
				if (tile.TouchedLeft || tile.TouchedRight) && (tile.TouchedUp || tile.TouchedDown) {
					fmt.Printf("+")
				} else if tile.TouchedLeft || tile.TouchedRight {
					fmt.Printf("-")
				} else {
					fmt.Printf("|")
				}
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Printf("\n")
	}
	fmt.Println("-------------------------------")
}

func (grid Grid) IsOutOfBounds(y, x int) bool {
	return (y >= len(grid) || y < 0 || x >= len(grid[0]) || x < 0)
}

func (grid Grid) CountTouches() int {
	total := 0

	for _, row := range grid {
		for _, tile := range row {
			if tile.Touched {
				total++
			}
		}
	}
	return total
}

func (grid Grid) IsStuckInLoop(coord Coord) bool {
	newY := coord.Y + coord.DirectionY
	newX := coord.X + coord.DirectionX
	if grid.IsOutOfBounds(newY, newX) {
		return false
	}

	tile := grid[newY][newX]

	if tile.Touched {
		if (tile.TouchedLeft && coord.DirectionX < 0) || (tile.TouchedRight && coord.DirectionX > 0) || (tile.TouchedUp && coord.DirectionY < 0) || (tile.TouchedDown && coord.DirectionY > 0) {
			// fmt.Println("Found an infinite loop")
			return true
		}
	}
	return false
}
