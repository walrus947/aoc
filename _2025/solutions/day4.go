package solutions

import (
	"log"
	"os"
	"strings"

	"github.com/walrus947/aoc/_2025/utils"
)

const (
	TPRoll = '@'
	Empty = '.'

	// Forklift = 'x'
)

type Coordinates struct {
	row int
	col int
}

// part 1: 409.625µs
// part 2: 6.850333ms
func Day4() {
	data, err := os.ReadFile("solutions/inputs/day4.txt")
	if err != nil {
		log.Fatalf("error: %s\n", err.Error())
	}

	lines := strings.Split(string(data), "\n")
	warehouse := [][]rune{}

	for _, line := range lines {
		warehouse = append(warehouse, []rune(line))
	}

	part1 := day4_SolvePart1(warehouse)
	part2 := day4_SolvePart2(warehouse)
	log.Printf("part 1: %d\n", part1)
	log.Printf("part 2: %d\n", part2)
}

func scanWarehouse(warehouse [][]rune) []*Coordinates {
	adjTPRollCoords := []*Coordinates{}

	for row := range warehouse {
		for col := range warehouse[row] {
			if warehouse[row][col] == TPRoll && canAccessTP(row, col, warehouse) {
				adjTPRollCoords = append(adjTPRollCoords, &Coordinates{row, col})
			}
		}
	}

	return adjTPRollCoords
}

func canAccessTP(row int, col int, warehouse [][]rune) bool {
	adjCells := []Coordinates {
		{row - 1, col - 1}, 
		{row, col - 1},     
		{row + 1, col - 1}, 

		{row - 1, col},     
		{row + 1, col},     

		{row - 1, col + 1}, 
		{row, col + 1},     
		{row + 1, col + 1}, 
	}

	var count int
	for _, coord := range adjCells {
		if inBounds(coord.row, coord.col, warehouse) && warehouse[coord.row][coord.col] == TPRoll {
			count++
		}
	}

	return count < 4
}

func inBounds(row int, col int, warehouse [][]rune) bool {
	return row >= 0 && col >= 0 && col < len(warehouse) && row < len(warehouse[col])
}

func removeTp(warehouse [][]rune, tpToRemove []*Coordinates) {
	for _, coord := range tpToRemove {
		warehouse[coord.row][coord.col] = Empty
	}
}

func day4_SolvePart1(warehouse [][]rune) int {
	stop := utils.Timer("day4_part1")
	defer stop()

	return len(scanWarehouse(warehouse))
}

func day4_SolvePart2(warehouse [][]rune) int {
	stop := utils.Timer("day4_part2")
	defer stop()

	var count int	
	for {
		tpToRemove := scanWarehouse(warehouse)
		if len(tpToRemove) == 0 {
			return count
		}

		removeTp(warehouse, tpToRemove)
		count += len(tpToRemove)
	}
}

