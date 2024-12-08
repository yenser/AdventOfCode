package main

type Coord struct {
	X          int
	Y          int
	DirectionX int
	DirectionY int
}

func (c *Coord) SetDirection(direction string) {
	if direction == "up" {
		c.DirectionY = -1
		c.DirectionX = 0
	} else if direction == "down" {
		c.DirectionY = 1
		c.DirectionX = 0
	} else if direction == "left" {
		c.DirectionY = 0
		c.DirectionX = -1
	} else if direction == "right" {
		c.DirectionY = 0
		c.DirectionX = 1
	}
}

func (c *Coord) TurnRight(grid Grid) {

	if c.DirectionX == 1 {
		c.DirectionX = 0
		c.DirectionY = 1
	} else if c.DirectionX == -1 {
		c.DirectionX = 0
		c.DirectionY = -1
	} else if c.DirectionY == 1 {
		c.DirectionX = -1
		c.DirectionY = 0
	} else if c.DirectionY == -1 {
		c.DirectionX = 1
		c.DirectionY = 0
	}

	grid[c.Y][c.X].Touch(c.DirectionY, c.DirectionX)
}

func (c *Coord) Move(grid Grid) bool {
	newY := c.Y + c.DirectionY
	newX := c.X + c.DirectionX

	if grid.IsOutOfBounds(newY, newX) {
		c.Y = newY
		c.X = newX
		return true
	}

	if grid[newY][newX].HasObstacle {
		return false
	} else {

		c.Y = newY
		c.X = newX

		grid[c.Y][c.X].Touch(c.DirectionY, c.DirectionX)

		return true
	}

}

func (c Coord) getDirectionCharacter() string {
	if c.DirectionX > 0 {
		return ">"
	} else if c.DirectionX < 0 {
		return "<"
	} else if c.DirectionY > 0 {
		return "v"
	} else {
		return "^"
	}
}
