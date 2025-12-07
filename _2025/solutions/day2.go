package solutions

import (
	"bufio"
	"log"
	"math"
	"os"
	"strconv"
	"strings"

	"github.com/walrus947/aoc/_2025/utils"
)

func Day2() {
	file, err := os.Open("solutions/inputs/day2.txt")
	if err != nil {
		log.Fatalf("error: %s\n", err.Error())
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	line := ""
	for scanner.Scan() {
		line = scanner.Text()
	}

	part1_nieve, part2_nieve := day2_SolveNieve(line)
	log.Printf("part 1 (nieve): %d\n", part1_nieve)
	log.Printf("part 2 (nieve): %d\n", part2_nieve)

	// part1 := part1_SolveWithSubsequences(line)
	// log.Printf("part 1: %d\n", part1)
}

/* nieve solution */

func part1_isValidId_NIEVE(id string) bool {
	// since an invalid id only repeats twice, any invalid id with an odd number of digits is valid
	if len(id)%2 != 0 {
		return true
	}

	leftIdx := 0
	rightIdx := len(id) / 2
	// while this is nice, it's about 10ms slower than the dumb for loop
	// return id[:midpoint] == id[midpoint:]

	// since all odd digit-ed ids have been filtered, we only need to check one bound
	for ; rightIdx < len(id); leftIdx, rightIdx = leftIdx+1, rightIdx+1 {
		if id[leftIdx] != id[rightIdx] {
			return true
		}
	}

	return false
}

func part2_isValidId_NIEVE(id string) bool {
	for substrSize := 1; substrSize <= len(id)/2; substrSize++ {
		if isSubstrRepeated(id, substrSize) {
			return false
		}
	}

	return true
}

func isSubstrRepeated(id string, substrSize int) bool {
	substr := id[:substrSize]

	for startChunk := substrSize; startChunk < len(id); startChunk += substrSize {
		endChunk := int(math.Min(float64(startChunk)+float64(substrSize), float64(len(id))))

		if id[startChunk:endChunk] != substr {
			return false
		}
	}

	return true
}

// since this is aoc, we're going to assume the input is well-formatted
func day2_SolveNieve(line string) (int, int) {
	stop := utils.Timer("day2_nieve")
	defer stop()

	invalidIdsPart1 := 0
	invalidIdsPart2 := 0

	ranges := day2_parseRanges(line)

	for _, r := range ranges {
		for idAsInt := r.Start; idAsInt <= r.End; idAsInt++ {
			id := strconv.Itoa(int(idAsInt))
			if !part1_isValidId_NIEVE(id) {
				invalidIdsPart1 += int(idAsInt)
			}
			if !part2_isValidId_NIEVE(id) {
				invalidIdsPart2 += int(idAsInt)
			}
		}
	}

	return invalidIdsPart1, invalidIdsPart2
}

func day2_parseRanges(line string) []Range {
	var ranges []Range
	log.Println("line: ", line)

	for _, idSet := range strings.Split(line, ",") {
		ids := strings.Split(idSet, "-")
		start, err1 := strconv.ParseInt(ids[0], 10, 64)
		if err1 != nil {
			log.Fatalf("error: %s\n", err1.Error())
			return nil
		}
		end, err2 := strconv.ParseInt(ids[1], 10, 64)
		if err2 != nil {
			log.Fatalf("error: %s\n", err2.Error())
			return nil
		}
		ranges = append(ranges, Range{Start: start, End: end})
	}

	return ranges
}

// thought: the nieve solution goes over all ranges, so it's super slow at SUM(N_range_size * time_per_check) ~= O(N_range_size)
// which is linear to the range size, which sucks

// instead, we can generate all invalid IDs and then check if they belong to any given range
// assume an ID of length L is invalid if L % 2 == 0 && digits[0:L/2) == digits [L/2: L)

// generate all subsequences S such that they form the space of invalid IDs s.t. L <= 18 as SS is an invalid ID
// for L = 2, you have 11, 22, 33 ... 88, 99 = 9 => 9 * 10^0
// for L = 3, you have no invalid IDs since L % 2 == 1
// for L = 4, you have { L = 2 } + 1111, 1212, 1313 ... = 90 => 9 * 10^1

// the geometric series for L = 18 becomes sum_K=1^9{ 9 * 10^(K - 1) } ~= 10^9
// or for any L, sum_K=1^(L / 2){ (L / 2) * 10^(K - 1) }

// this gives us some working function s.t. we can build any invalid ID as SS where S is a substring of length L / 2

// reality:
// the problem with this approach is that it assumes that the space of all ranges is significantly large s.t. the
// computation of all invalid subsequences is worth the squeeze
// for this input, it's not worth it at all. the nieve solution that goes over all ranges takes ~50ms while the
// "optimized" solution takes ~17s

type Range struct {
	Start int64
	End   int64
}

func part1_SolveWithSubsequences(line string) int64 {
	stop := utils.Timer("optimized_part1")
	defer stop()

	ranges := day2_parseRanges(line)
	if ranges == nil {
		return -1
	}

	var totalSum int64
	for k := 1; k <= 9; k++ {
		pow10K := int64(math.Pow10(k))

		startS := int64(math.Pow10(k - 1))
		endS := int64(math.Pow10(k))

		for S := startS; S < endS; S++ {
			SS := S * (pow10K + 1)

			for _, r := range ranges {
				if SS >= r.Start && SS <= r.End {
					totalSum += SS
				}
			}
		}
	}

	return totalSum
}
