package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	input "2022/packages/input"
)

const totalAvailableSpace = 70_000_000
const spaceNeeded = 30_000_000

var total = 0
var deleteDir = []int{}

func main() {
	list := input.ReadFileToArray("input.txt")

	fmt.Println("-------- Part 1 --------")
	part1(*list)
	fmt.Println("\n\n-------- Part 2 --------")
	part2(*list)

}

func part1(strList []string) {

	root := File{name: "/", isDir: true, subDir: map[string]*File{}}

	cursor := root
	for _, l := range strList {
		strout := strings.Fields(l)

		fmt.Println(l)
		cursor = *parseStrout(strout, &cursor)

	}

	fmt.Println("\n")

	printDir(root, 0)

	fmt.Println("\n")

	sumDir(root)

	fmt.Println("\n")

	fmt.Printf("Total: %d\n", total)
}

func part2(strList []string) {
	root := File{name: "/", isDir: true, subDir: map[string]*File{}}

	cursor := root
	for _, l := range strList {
		strout := strings.Fields(l)

		fmt.Println(l)
		cursor = *parseStrout(strout, &cursor)

	}

	used := sumDir(root)

	spaceToRemove := spaceNeeded - (totalAvailableSpace - used)

	fmt.Printf("Total: %d\nUsed: %d\nNeeded: %d\nClear: %d\n", totalAvailableSpace, used, spaceNeeded, spaceToRemove)
	fmt.Println("\n")

	sumDirPt2(root, spaceToRemove)

	sort.Ints(deleteDir)

	fmt.Printf("Answer: %d\n", deleteDir[0])
}

func parseStrout(input []string, cursor *File) *File {
	if input[0] == "$" {
		switch input[1] {
		case "cd": // handle CD
			{
				if input[2] == "/" {
					fmt.Println("change directory to root")
					return cursor
				}

				if findNameIdx(cursor.subDir, input[2]) {
					fmt.Printf(" - change directory from %s to %s\n", cursor.name, cursor.subDir[input[2]].name)
					return cursor.subDir[input[2]]
				} else {
					fmt.Println(" - err: invalid index")
				}

			}
		}
	} else if input[0] == "dir" { // add dir
		addDir(cursor, input[1])
		fmt.Printf(" - new dir %s : .. = %s\n", cursor.subDir[input[1]].name, cursor.subDir[input[1]].subDir[".."].name)

	} else { // add file
		size, _ := strconv.Atoi(input[0])
		addFile(cursor, input[1], size)
	}

	fmt.Println()

	return cursor
}

type File struct {
	isDir  bool
	subDir map[string]*File
	name   string
	size   int
}

func findNameIdx(subDir map[string]*File, val string) bool {

	if _, ok := subDir[val]; ok {
		return true
	} else if _, ok := subDir[val+"/"]; ok {
		return true
	} else {
		return false
	}
}

func printDir(folder File, depth int) {
	if folder.name == "/" {
		depth = 0
	}

	tab := ""
	for i := 0; i < depth; i++ {
		tab += "  "
	}

	if folder.isDir {
		fmt.Printf("%s- %s (dir)\n", tab, folder.name)
	} else {
		fmt.Printf("%s- %s (file, size=%d)\n", tab, folder.name, folder.size)
	}

	if folder.isDir {
		for k, file := range folder.subDir {
			if k != ".." {
				printDir(*file, depth+1)
			}
		}
	}
}

func sumDir(folder File) int {
	sum := 0

	if folder.isDir {
		for k, file := range folder.subDir {
			if k != ".." {
				sum += sumDir(*file)
			}
		}
	} else {
		sum += folder.size
	}

	if sum <= 100_000 && folder.isDir { // pt 1
		// fmt.Printf("%s is %d\n", folder.name, sum)
		total += sum
	}

	return sum
}

func sumDirPt2(folder File, spaceNeeded int) int {
	sum := 0

	if folder.isDir {
		for k, file := range folder.subDir {
			if k != ".." {
				sum += sumDirPt2(*file, spaceNeeded)
			}
		}
	} else {
		sum += folder.size
	}

	if sum >= spaceNeeded && folder.isDir { // pt 2
		fmt.Printf("%s is %d\n", folder.name, sum)
		deleteDir = append(deleteDir, sum)
	}

	return sum
}

func addDir(cur *File, name string) {
	var dir File = File{name: name + "/", isDir: true, subDir: map[string]*File{}}
	parentDir := *cur // used to prevent subDir from pointing at cursor, but instead what the value of cur is
	dir.subDir[".."] = &parentDir

	cur.subDir[name] = &dir

	// fmt.Printf(" - Adding %s to %s\n", dir.name, cur.name)
}

func addFile(cur *File, name string, size int) {
	var file File = File{name: name, isDir: false, size: size, subDir: nil}
	cur.subDir[name] = &file

	// fmt.Printf(" - Adding %s %d to %s\n", file.name, file.size, cur.name)
}
