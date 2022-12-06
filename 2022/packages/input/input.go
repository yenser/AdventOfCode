package input

import (
	"bufio"
	"log"
	"os"
)

func ReadFileToArray(fileName string) *[]string {
	lines := []string{}

	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return &lines
}

func ReadFileToScanner(fileName string) (*bufio.Scanner, *os.File) {
	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	return scanner, file
}
