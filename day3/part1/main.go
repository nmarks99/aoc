package main

import (
    "regexp"
    "os"
    "log"
    "strconv"
    "strings"
    "fmt"
)

func readPuzzleInput(filename string) string {
    contents, err := os.ReadFile(filename)
    if err != nil {
        log.Fatal(err)
    }
    return string(contents)
}

func do_mul(input string) int {
    mul_split := strings.Split(input, ",")
    re_num := regexp.MustCompile(`\d+`)

    num1, err := strconv.Atoi(re_num.FindString(mul_split[0]))
    if err != nil {
        log.Fatal(err)
    }
    num2, err := strconv.Atoi(re_num.FindString(mul_split[1]))
    if err != nil {
        log.Fatal(err)
    }

    return num1 * num2
}

func main() {

    var text string = readPuzzleInput("puzzle_input.txt")

    re := regexp.MustCompile(`mul\(\d{1,3},\d{1,3}\)`)
    var matches []string = re.FindAllString(text, -1)

    var total int = 0
    for _,v := range matches {
        total += do_mul(v)
    }

    fmt.Println("Total =", total)

}
