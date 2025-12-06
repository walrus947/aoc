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

func Day3() {
	file, err := os.Open("solutions/inputs/day3.txt")
	if err != nil {
		log.Fatalf("error: %s\n", err.Error())
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	part1, part2 := day3_Solve(scanner)
	log.Printf("part 1: %d\n", part1)
	log.Printf("part 2: %d\n", part2)
}

func part1_FindLargestJoltage(line string) int {
	var tens, ones int
	for idx := 0; idx < len(line) - 1; idx++ {
		currentDigit := int(line[idx] - '0')
		if tens < currentDigit {
			tens = currentDigit
			ones = -1
		} else if ones < currentDigit {
			ones = currentDigit
		}

		if tens == 9 && ones == 9 {
			break
		}
	}

	// if we've broken out of the loop, then tens has been assigned, 
	// but ones has not, so ones = max(ones, len(line) - 1)
	ones = int(math.Max(float64(ones), float64(int(line[len(line) - 1] - '0'))))

	return tens * 10 + ones
}

func part2_FindLargestJoltage(line string) int {
	result := make([]rune, 0, 12)

	for idx, ch := range line {
		for len(result) > 0 {
			d_prev := result[len(result)-1]
			
			isGreedy := ch > d_prev
			
			// we can only drop if we have enough characters still to parse to 
			// make a value with 12 digits
			isAbleToDrop := (len(result)) + (len(line) - (idx + 1)) >= 12

			if !isAbleToDrop {
				finalChars := line[idx:]
				result = append(result, []rune(finalChars)...)
				goto final_conversion
			}
			
			if isGreedy && isAbleToDrop {
				result = result[:len(result)-1] // Pop
			} else {
				// Stop dropping if the current digit isn't better OR if dropping 
				// would make the final result too short.
				break 
			}
		}

		result = append(result, ch)
	}
	
final_conversion:
	// sadly, despite allocating 12 bytes, the slices can grow dynamically past it
	// we need to trim it to the 12 MSD
	result = result[:12]

	battery, _ := strconv.Atoi(string(result))

	return battery
}

// unlike part1, I couldn't think of a good way to do this step in a single pass
// rather, we can do it two
// 1.) count the frequencies of digits to determine what the cutoff digit will be
// 2.) use the frequency map to filter the original string to construct the battery

// abandoned attempt after I figured out the greedy solution
func part2_FindLargestJoltage_FREQUENCY(line string) int {
	var digitFreq [10]int

	for _, ch := range line {
		d, _ := strconv.Atoi(string(ch)) // it's AOC, we assume input is good
		digitFreq[d]++ // same here; it's a digit
	}

	log.Printf("dist: %v", digitFreq)

	digitsToCutoff := 12
	cutoffDigit := -1

	for d := 9; d >= 0; d-- {  
		count := digitFreq[d]

		if count >= digitsToCutoff {
			cutoffDigit = d
			// the break should always happen; we'll always have enough characters
			// to satisfy digitsToCutoff
			break
		}

		digitsToCutoff -= count
	}

	log.Println("digitsToCutoff: ", digitsToCutoff)

	var result strings.Builder

	for _, ch := range line {
		d, _ := strconv.Atoi(string(ch)) // it's AOC, we assume input is good

		if d > cutoffDigit {
			result.WriteRune(ch)
		} else if d == cutoffDigit {
			if digitsToCutoff > 0 {
				result.WriteRune(ch)
				digitsToCutoff--
			}
		}

		if result.Len() == 12 {
			break
		}
	}
	
	log.Printf("line: %s", line)
	log.Printf("battery: %s", result.String())
	battery, _ := strconv.Atoi(result.String())
	return battery
}

// part1: 33.25µs
// part1 + part2: 157.25µs
func day3_Solve(scanner *bufio.Scanner) (int, int) {
	stop := utils.Timer("Day3")
	defer stop()

	var joltagePart1, joltagePart2 int
	for scanner.Scan() {
		line := scanner.Text()
		joltagePart1 += part1_FindLargestJoltage(line)
		joltagePart2 += part2_FindLargestJoltage(line)
	}

	return joltagePart1, joltagePart2 
}
