package utils

import (
	"bufio"
	"log"
	"time"
)

func Timer(name string) func() {
	start := time.Now()

	return func() {
		elapsed := time.Since(start)
		log.Printf("%s took %s", name, elapsed)
	}
}

func TimeExecution(scanner *bufio.Scanner, f func(scanner *bufio.Scanner)) {
	start := time.Now()
	f(scanner)
	elapsed := time.Since(start)
	log.Printf("-> took %s", elapsed)
}
