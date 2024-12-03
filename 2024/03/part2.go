package main

import (
	"regexp"
)

const mulExpWithInstructions = `mul\(([0-9]{1,3}\,[0-9]{1,3})\)|do\(\)|don\'t\(\)`

func mulLexerWithInstructions(data []byte) [][]string {
	r, _ := regexp.Compile(mulExpWithInstructions)

	return r.FindAllStringSubmatch(string(data), -1)

}

func part2(data []byte) int {

	lex := mulLexerWithInstructions(data)
	// fmt.Println(lex)

	total := 0

	do := true

	for i := 0; i < len(lex); i++ {
		if lex[i][0] == "do()" {
			do = true
			continue
		} else if lex[i][0] == "don't()" {
			do = false
			continue
		}

		if do {
			x := multiply(lex[i][1])
			total += x
		}
	}

	return total
}
