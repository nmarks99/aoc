package main

import (
    "fmt"
    "log"
    "os"
    "bufio"
    "strconv"
    "strings"
    "sort"
)

func slice_count(arr []int, val int) int {
    var count int = 0
    for i := range arr {
	if arr[i] == val {
	    count += 1
	}
    }
    return count
}

func main() {
    
    file, err := os.Open("puzzle_input.txt")
    if err != nil {
	log.Fatal(err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)

    var col1 []int
    var col2 []int
    
    // fill col1 and col2 with the integers
    for scanner.Scan() {
	line := scanner.Text()
	var line_split []string = strings.Split(line, "   ")
	v1, err := strconv.Atoi(line_split[0])
	if err != nil {
	    log.Fatal(err)
	}
	v2, err := strconv.Atoi(line_split[1])
	if err != nil {
	    log.Fatal(err)
	}
	col1 = append(col1, v1)
	col2 = append(col2, v2)
    }

    // sort col1 and col2 slices
    sort.Ints(col1)
    sort.Ints(col2)
    
    var score int
    for i := range col1 {
	score += col1[i] * slice_count(col2, col1[i])
    }
    
    fmt.Println("Similarity score = ", score)

}
