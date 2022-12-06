package main

import (
	"fmt"

	input "2022/packages/input"
)

func main() {
	list := input.ReadFileToArray("test.txt")

	fmt.Println("-------- Part 1 --------")
	part1(*list)
	fmt.Println("\n\n-------- Part 2 --------")
	part2(*list)

}

func part1(list []string) {
	for _, l := range list {

		windowSize := 4
		window := make([]rune, windowSize)

		for i, c := range l {
			pushWindow(&window, c)

			found := false

			for j := 0; j < len(window); j++ {
				if window[j] == 0 { // calls when array is defaulted to 0 (length < 4)
					found = true
					break
				}

				for k := j + 1; k < len(window); k++ {
					if window[j] == window[k] {
						found = true
						break
					}
				}

				if found {
					break
				}
			}

			// fmt.Printf("%v %v\n", window, found)

			if !found {
				fmt.Printf("%v : %v\n", l, i+1)
				break
			}
		}
	}
}

func part2(list []string) {
	for _, l := range list {

		windowSize := 14
		window := make([]rune, windowSize)

		for i, c := range l {
			pushWindow(&window, c)

			found := false

			for j := 0; j < len(window); j++ {
				if window[j] == 0 { // calls when array is defaulted to 0 (length < 4)
					found = true
					break
				}

				for k := j + 1; k < len(window); k++ {
					if window[j] == window[k] {
						found = true
						break
					}
				}

				if found {
					break
				}
			}

			// fmt.Printf("%v %v\n", window, found)

			if !found {
				fmt.Printf("%v : %v\n", l, i+1)
				break
			}
		}
	}
}

func pushWindow(window *[]rune, x rune) {
	for i := 0; i < len(*window); i++ {
		if i != len(*window)-1 {
			(*window)[i] = (*window)[i+1]
		} else {
			(*window)[i] = x
		}
	}
}
