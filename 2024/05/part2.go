package main

func reorderPages(pages []int, rules map[int][]int) []int {
	for i := len(pages) - 1; i >= 0; i-- {
		rule, ok := rules[pages[i]]
		if !ok {
			continue
		}

		if !validatePage(pages, i, rule) {
			for j := i; j >= 0; j-- {
				swap(pages, j, j-1)
				// fmt.Printf("%v\n", pages)

				if validatePage(pages, j, rule) {
					break
				}
			}

			i++ // reset i because the swapped value could also be wrong
		}
	}

	return pages
}

func swap(a []int, x, y int) {
	tmp := a[y]
	a[y] = a[x]
	a[x] = tmp
}

func part2(data []byte) int {
	rules, updates := getRulesAndPages(data)
	total := 0

	for _, pages := range updates {
		if !validateUpdate(pages, rules) {
			reorderedPages := reorderPages(pages, rules)
			total += reorderedPages[len(reorderedPages)/2]
		}
	}

	return total
}
