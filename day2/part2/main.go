package main

import (
    "fmt"
    "bufio"
    "log"
    "os"
    "strings"
    "strconv"
)

func readPuzzleInput(filename string, delimiter string) [][]int {
    // open puzzle input file
    file, err := os.Open(filename)
    if err != nil {
	log.Fatal(err)
    }
    defer file.Close()
    
    var out [][]int
    
    // scan each row into a slice of int, and append it to 
    // out which is a slice of slices of int
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
	var line string = scanner.Text()
	var line_split []string = strings.Split(line,delimiter)
	var line_out []int
	for _,v := range line_split {
	    v_int, err := strconv.Atoi(v)
	    if err != nil {
		log.Fatal(err)
	    }
	    line_out = append(line_out, v_int)
	}
	out = append(out, line_out)
    }
    return out
}

func isAllSafe(levels []int) bool {
  
    var safe bool = true
    var ascend bool

    if levels[1] > levels[0] {
	ascend = true
    } else {
	ascend = false
    }

    for i := 0; i < len(levels)-1; i++ {
	diff := levels[i+1] - levels[i]

	// ensures direction is correct
	if ascend {
	    if diff < 0 {
		safe = false
	    }
	} else {
	    if diff > 0 {
		safe = false
	    } else {
		diff = -diff // abs of diff
	    }
	}

	// ensures increment is safe
	if diff < 1 || diff > 3 {
	    safe = false
	}
    }

    return safe
}

func isSafeAterDampener(levels []int) bool {
    var safe bool = false
    for i := 0; i < len(levels); i++ {

	test_levels := make([]int, len(levels))
	copy(test_levels, levels)
    
	test_levels = append(test_levels[:i], test_levels[i+1:]...)

	if isAllSafe(test_levels) {
	    safe = true
	    break
	}
    }
    return safe
}

func main() {
    data := readPuzzleInput("puzzle_input.txt", " ")

    var safe_count int = 0 
    for _, levels := range data {
	if isAllSafe(levels) {
	    safe_count += 1
	} else if isSafeAterDampener(levels) {
	    safe_count += 1
	}
    }

    fmt.Println("Total safe =", safe_count)
}
