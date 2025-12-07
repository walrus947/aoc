package solutions

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

const (
	DIRECTION_LEFT  = -1
	DIRECTION_RIGHT = 1
)

type Dial struct {
	value   int
	counter int
}

func part1_rotateAndCarry(dial *Dial, dir int, rotations int) {
	// get unsigned mod of dial rotation -> ((a (mod n) + n) mod n)
	dial.value = (dial.value + (rotations * dir)) % 100
	if dial.value < 0 {
		dial.value += 100
	}

	// we can reduce the above to the following one-liner if we wanted to
	// dial.value = ((dial.value + rotations * dir) % 100 + 100) % 100

	if dial.value == 0 {
		dial.counter++
	}
}

func part2_rotateAndCarry(dial *Dial, dir int, rotations int) {
	// `rotations` is always postitive
	dial.counter += rotations / 100
	rotations %= 100

	started_at_zero := dial.value == 0

	// since we modulo reduce the rotations, this becomes bounded by [-99, 198]
	// so unless you start at 0, any value <= 0 is +1 to the counter
	// and any value > 99 is +1 to the counter
	dial.value = dial.value + (rotations * dir)
	if (!started_at_zero && dial.value <= 0) || (dial.value > 99) {
		dial.counter++
	}

	dial.value %= 100
	if dial.value < 0 {
		dial.value += 100
	}
}

// since this is aoc, we're going to assume the input is well-formatted
func read_file_and_solve(scanner *bufio.Scanner) (int, int) {
	dial_part1 := Dial{
		value:   50,
		counter: 0,
	}
	dial_part2 := Dial{
		value:   50,
		counter: 0,
	}

	for scanner.Scan() {
		line := scanner.Text()
		direction := DIRECTION_LEFT
		if line[0] == 'R' {
			direction = DIRECTION_RIGHT
		}
		rotations, err := strconv.Atoi(line[1:])
		if err != nil {
			log.Fatalf("error: %s\n", err.Error())
			return -1, -1
		}

		part1_rotateAndCarry(&dial_part1, direction, rotations)
		part2_rotateAndCarry(&dial_part2, direction, rotations)
	}

	return dial_part1.counter, dial_part2.counter
}

func Day1() {
	file, err := os.Open("solutions/inputs/day1.txt")
	if err != nil {
		log.Fatalf("error: %s\n", err.Error())
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	part1, part2 := read_file_and_solve(scanner)
	log.Printf("part 1: %d\n", part1)
	log.Printf("part 2: %d\n", part2)

}
