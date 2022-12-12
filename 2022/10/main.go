package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
	"time"

	input "2022/packages/input"
)

func main() {
	list := input.ReadFileToArray("input.txt")

	fmt.Println("-------- Part 1 --------")
	part1(*list)
	fmt.Println("\n\n-------- Part 2 --------")
	part2(*list)

}

type process struct {
	value  int
	cycles int
}

func newProcess(value, cycles int) process {
	process := process{
		value:  value,
		cycles: cycles,
	}

	return process
}

func createProgram(list []string) []process {
	program := make([]process, len(list))
	for i, l := range list {
		fields := strings.Fields(l)

		switch fields[0] {
		case "noop":
			{
				program[i] = newProcess(0, 1)
			}
		case "addx":
			{
				val, _ := strconv.Atoi(fields[1])
				program[i] = newProcess(val, 2)
			}
		}
	}

	return program
}

func part1(strList []string) {
	program := createProgram(strList)

	var cycle int = 1
	var register int = 1
	var i int = 0

	var total int = 0

	for {
		if i == len(program) {
			break
		}

		if cycle == 20 || ((cycle-20)%40) == 0 {
			fmt.Printf("Cycle: %d\tRegister: %d\tStrength: %d\n", cycle, register, cycle*register)
			total += (cycle * register)
		} else {
			fmt.Printf("Cycle: %d\tRegister: %d\n", cycle, register)
		}

		program[i].cycles--

		if program[i].cycles == 0 {
			register += program[i].value
			i++
		}

		// cycle turnover
		// time.Sleep(50 * time.Millisecond)
		cycle++
	}

	fmt.Println("Total: ", total)
}

func part2(strList []string) {
	program := createProgram(strList)

	var cycle int = 1
	var register int = 1
	var i int = 0

	for {
		if i == len(program) {
			break
		}

		x := (cycle - 1) % 40

		if x == 0 { // new line every 40 pixels
			fmt.Printf("\n")
		}

		isOn := false

		if math.Abs(float64(register-(x))) <= 1 {
			isOn = true
		}

		if isOn {
			fmt.Printf("#")
		} else {
			fmt.Printf(".")
		}

		program[i].cycles--

		if program[i].cycles == 0 {
			register += program[i].value
			i++
		}

		// cycle turnover
		time.Sleep(75 * time.Millisecond)
		cycle++
	}

	fmt.Println()
}
