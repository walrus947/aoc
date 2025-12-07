package solutions

import (
	"log"
	"os"
	"strings"

	"github.com/walrus947/aoc/_2025/utils"
)

const (
	START = 'S'
	SPLITTER = '^'
	SPLITTER_IN_USE = 'x'
	NOTHING = '.'
)

// part 1 (initial): 41.417µs
// part 2 (initial): 41.958µs 

// part 1 (after refactor): 31.542µs -> + 23.84%
// part 2 (after refactor): 38.5µs   -> +  8.24%

// lesson: should've looked for the iterative approach before recursion
func Day7() {
	data, err := os.ReadFile("solutions/inputs/day7.txt")
	if err != nil {
		log.Fatalf("error: %s\n", err.Error())
	}

	lines := strings.Split(string(data), "\n")
	lab := [][]rune{}

	for _, line := range lines {
		lab = append(lab, []rune(line))
	}

	// make a deep copy of the lab since we mutate the 2D array
	// this was from before a refactor where we mutated a lot more than just
	// `SPLITTER_IN_USE`. we could just change all `SPLITTER_IN_USE` back to `SPLITTER`,
	// but we don't time this part, so oh well ¯\_(ツ)_/¯
	lab_part2 := make([][]rune, len(lab))
	for idx, row := range lab {
		lab_part2[idx] = make([]rune, len(row))
		copy(lab_part2[idx], row)
	}

	// remove the last row since it's from parsing EOF and not a part of the data
	part1 := day7_Part1(lab[:len(lab) - 1])
	part2 := day7_Part2(lab_part2[:len(lab) - 1])

	log.Printf("part 1: %d\n", part1)
	log.Printf("part 2: %d\n", part2)
}

func day7_Part1(lab [][]rune) int {
	stop := utils.Timer("part1")
	defer stop()

	for jdx, ch := range lab[0] {
		if ch == START {
			return beam(lab, /* row */ 1, jdx)
		}
	}

	return -1 /* UNREACHABLE */ 
}

func day7_Part2(lab [][]rune) int {
	stop := utils.Timer("part2")
	defer stop()

	previousRow := make([]int, len(lab[0]))
	for row := range lab { 
		runningRow := make([]int, len(previousRow))
		for col := range lab[0] {
			if lab[row][col] == START {
				runningRow[col] = 1 
				goto next_row // i'd rather goto than have some boolean flag
			}

			if lab[row][col] == SPLITTER {
				runningRow[col - 1] += previousRow[col]
				runningRow[col + 1] += previousRow[col]
			} else {
				runningRow[col] += previousRow[col]
			}

		}
		next_row:
		previousRow = runningRow
	}

	var count int
	for _, val := range previousRow {
		count += val
	}

	return count
}

func beam(lab [][]rune, row int, col int) int {
	var count int 
	for {
		if row == len(lab) - 1 || lab[row][col] == SPLITTER_IN_USE {
			return count
		} 

		if lab[row][col] == NOTHING {
			row++
			continue
		}

		if lab[row][col] == SPLITTER {
			// the recursion causes a data race where we want to protect against
			// two paths reaching the same splitter

			// if another path gets to this point, we'd like to early return
			lab[row][col] = SPLITTER_IN_USE
			count += beam(lab, row + 1, col - 1) + 1
			count += beam(lab, row + 1, col + 1)
			break
		}
	}	
		
	return count
}
