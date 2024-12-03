package main

import (
	"log"
	"regexp"
	"strconv"
	"strings"
)

const mulExp = `mul\(([0-9]{1,3}\,[0-9]{1,3})\)`

func mulLexer(data []byte) [][]string {
	r, _ := regexp.Compile(mulExp)

	return r.FindAllStringSubmatch(string(data), -1)

}

func multiply(val string) int {
	s := strings.Split(val, ",")
	if len(s) != 2 {
		log.Fatalf("string not of length 2: %s\n", val)
	}

	val1, _ := strconv.Atoi(s[0])
	val2, _ := strconv.Atoi(s[1])

	return val1 * val2
}

func part1(data []byte) int {

	lex := mulLexer(data)
	// fmt.Println(lex)

	total := 0

	for _, val := range lex {
		x := multiply(val[1])
		total += x
	}

	return total
}
