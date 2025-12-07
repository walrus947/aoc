package solutions

import (
	"bufio"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"

	"github.com/walrus947/aoc/_2025/utils"
)

// part1: 107.458µs
// part1 + part2: 371.792µs on first run, 105.625µs for subsequent
// I guess warm TLB really is beneficial, huh
func Day5() {
	file, err := os.Open("solutions/inputs/day5.txt")
	if err != nil {
		log.Fatalf("error: %s\n", err.Error())
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	part1, part2 := day5_Solve(scanner)
	log.Printf("part 1: %d\n", part1)
	log.Printf("part 2: %d\n", part2)
}

type IdRange struct {
	start int
	end   int
}

func day5_Solve(scanner *bufio.Scanner) (int, int) {
	stop := utils.Timer("day5")
	defer stop()

	ranges, ids := parseInput(scanner)
	mergedRanges := reduceRanges(ranges)

	return len(getFreshIds(mergedRanges, ids)), scanDbForFreshIds(mergedRanges)
}

func getFreshIds(ranges []IdRange, ids []int) []int {
	freshIds := make([]int, 0)
	for _, id := range ids {
		for _, r := range ranges {
			if id >= r.start && id <= r.end {
				freshIds = append(freshIds, id)
				break
			}
		}
	}

	return freshIds
}

func parseInput(scanner *bufio.Scanner) ([]IdRange, []int) {
	ranges := []IdRange{}
	ids := []int{}

	for scanner.Scan() {
		line := scanner.Text()
		// log.Println("line: ", line)
		if line == "" {
			continue
		} else if strings.Contains(line, "-") {
			r := strings.Split(line, "-")
			start, _ := strconv.Atoi(r[0])
			end, _ := strconv.Atoi(r[1])
			// log.Printf("range: (%d, %d)", start, end)
			ranges = append(ranges, IdRange{start: start, end: end})
		} else {
			id, _ := strconv.Atoi(line)
			// log.Printf("id: %d", id)
			ids = append(ids, id)
		}
	}

	// we order ranges to make sure that alike ranges are close to each other
	// to be able to merge in a single pass of 'ranges'

	// before: 134.375µs
	// after: 107.458µs
	slices.SortFunc(ranges, func(this IdRange, that IdRange) int {
		return this.start - that.start
	})

	return ranges, ids
}

func reduceRanges(ranges []IdRange) []IdRange {
	mergedRanges := []IdRange{}

	for _, r := range ranges {
		for idx, mergedR := range mergedRanges {
			if r.start <= mergedR.end && mergedR.start <= r.end {
				newRange := IdRange{start: min(r.start, mergedR.start), end: max(r.end, mergedR.end)}
				mergedRanges[idx] = newRange
				goto skipMerge
			}
		}

		mergedRanges = append(mergedRanges, r)
	skipMerge:
	}

	return mergedRanges
}

func scanDbForFreshIds(ranges []IdRange) int {
	var count int

	for _, r := range ranges {
		// the range is inclusive
		count += (r.end - r.start + 1)
	}

	return count
}
