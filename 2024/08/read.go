package main

import (
	"log"
	"os"
)

func readInput(path string) []byte {
	data, err := os.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	return data
}
