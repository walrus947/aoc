package solutions

import (
	"log"
	"os"
	"strconv"
	"strings"

	"github.com/walrus947/aoc/_2025/utils"
)

const (
	MULT = "*"
	ADD = "+" 
)

var (
	MULT_FUNC = func(nums ...int) int {
		result := 1
		for _, num := range nums {
			result *= num
		}

		return result
	}

	ADD_FUNC = func(nums ...int) int {
		var result int
		for _, num := range nums {
			result += num
		}

		return result
	}
)

// part 1: 151.667µs
// part 2: 239µs
func Day6() {
	data, err := os.ReadFile("solutions/inputs/day6.txt")
	if err != nil {
		log.Fatalf("error: %s\n", err.Error())
	}

	lines := strings.Split(string(data), "\n")

	part1 := day6_SolvePart1(lines)
	part2 := day6_SolvePart2(lines)

	log.Printf("part 1: %d\n", part1)
	log.Printf("part 2: %d\n", part2)
}

func day6_SolvePart1(lines []string) int {
	stop := utils.Timer("part1")
	defer stop()

	lines = lines[:len(lines) - 1]
	
	operands := getOperands(lines[:len(lines) - 1])
	operators := getOperators(lines[len(lines) - 1])

	return doOperations(operands, operators)
}

func day6_SolvePart2(lines []string) int {
	stop := utils.Timer("part2")
	defer stop()

	lines = lines[:len(lines) - 1]
	
	operands := getOperandsForSquids(lines[:len(lines) - 1])
	operators := getOperators(lines[len(lines) - 1])

	return doOperationsPart2(operands, operators)
}

func getOperands(lines []string) [][]int {
	operands := [][]int{}

	for _, line := range lines {
		ops := []int{}
		for op := range strings.FieldsSeq(line) {
			opToInt, _ := strconv.Atoi(op)
			ops = append(ops, opToInt)
		}

		operands = append(operands, ops)
	}

	return operands
}

// just to avoid 2D arrays and make the appends not require doing arr^T
// realistically, probably should've just made a struct for operands, operator
// from the beginning, but I don't wanna refactor this
type Operands struct {
	ops []int
}

func getOperandsForSquids(lines []string) []Operands {
	operands := []Operands{}

	input := [][]rune{}

	for _, line := range lines {
		input = append(input, []rune(line))
	}

	startIdx := 0
	for col := range input[0] {
		isColDivider := true
		for row := 0; row < len(input); row++ {
			if input[row][col] != ' ' {
				isColDivider = false
			}
		}

		if isColDivider { 
			operands = append(operands, parseSquidInts(input, startIdx, col))
			startIdx = col + 1
		}
	}

	operands = append(operands, parseSquidInts(input, startIdx, len(input[0])))

	return operands
}

func parseSquidInts(input [][]rune, startIdx int, divider int) Operands {
	var op Operands

	for col := startIdx; col < divider; col++ {
		chars := []rune{}

		for idx := range len(input) {
			chars = append(chars, input[idx][col])
		}

		var val int
		for _, ch := range chars {
			if ch == ' ' {
				continue
			}

			chAsInt, _ := strconv.Atoi(string(ch))
			val = val * 10 + chAsInt	
		}

		op.ops = append(op.ops, val)
	}

	return op
}

func getOperators(line string) []func(... int) int {
	operands := []func(... int) int {}
	for op := range strings.FieldsSeq(line) {
		if op == MULT {
			operands = append(operands, MULT_FUNC)
		} else /* operand = ADD */ {
			operands = append(operands, ADD_FUNC)
		}
	}

	return operands
}

func doOperations(operands [][]int, operators []func(... int) int) int {
	var result int

	for col, _ := range operands[0] {
		ops := []int{}
		for row, _ := range operands {
			ops = append(ops, operands[row][col])
		}

		result += operators[col](ops...)
	}

	return result 
}

func doOperationsPart2(operands []Operands, operators []func(... int) int) int {
	var result int

	for col, op := range operands {
		result += operators[col](op.ops...)
	}

	return result 
}
