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

func doMul(input string) int {
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

    re_mul := regexp.MustCompile(`mul\(\d{1,3},\d{1,3}\)`)
    re_do := regexp.MustCompile(`do\(\)`)
    re_dont := regexp.MustCompile(`don't\(\)`)

    mul_matches_ind := re_mul.FindAllStringIndex(text,-1)
    do_matches_ind := re_do.FindAllStringIndex(text, -1)
    dont_matches_ind := re_dont.FindAllStringIndex(text, -1)

    max_index := mul_matches_ind[len(mul_matches_ind)-1][0]
    
    clean := make([]string, len(mul_matches_ind)+len(do_matches_ind)+len(dont_matches_ind))
    var clean_i int = 0

    for i := 0; i <= max_index; i++ {

        // check mul
        for _,v := range mul_matches_ind {
            if v[0] == i {
                clean[clean_i] = text[v[0]:v[1]]
                clean_i += 1
                break
            }
        }

        // check do
        for _,v := range do_matches_ind {
            if v[0] == i {
                clean[clean_i] = text[v[0]:v[1]]
                clean_i += 1
                break
            }
        }

        // check dont
        for _,v := range dont_matches_ind {
            if v[0] == i {
                clean[clean_i] = text[v[0]:v[1]]
                clean_i += 1
                break
            }
        }
    }

    var do_mode bool = true
    var total int = 0
    for _, v := range clean {
        if v == "do()" {
            do_mode = true
        } else if v == "don't()" {
            do_mode = false
        } else {
            if do_mode {
                total += doMul(v)
            }
        }
    }

    fmt.Println("Total =", total)

}
