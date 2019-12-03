package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

// Calculates total fuel based on provided input masses
func main() {
	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	totalFuel := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		mass, err := strconv.Atoi(scanner.Text())
		if err == nil {
			totalFuel += Calc(mass)
		} else {
			log.Fatal(err)
		}
	}

	log.Printf("mass: %d", totalFuel)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func Calc(mass int) int {
	return int(mass/3) - 2
}
