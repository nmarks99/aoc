package main

import (
    "fmt"
    "bufio"
    "os"
    "log"
    "strings"
    "strconv"
)

type Rules map[int][]int


func readUpdatesFile(filename string) [][]int {
    file, err := os.Open(filename)
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()
    
    var out [][]int
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        var line string = scanner.Text()
        var line_split []string = strings.Split(line,",")
        var out_tmp []int
        for _, v := range line_split {
            v_int, err := strconv.Atoi(v)
            if err != nil {
                log.Fatal(err)
            }
            out_tmp = append(out_tmp, v_int)
        }
        out = append(out, out_tmp)
    }

    return out
}

func readRulesFile(filename string) Rules {
    
    file, err := os.Open(filename)
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()
    
    var rules_list [][2]int
        
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        var line string = scanner.Text()
        var line_split []string = strings.Split(line,"|")
        v1, err := strconv.Atoi(line_split[0])
        if err != nil {
            log.Fatal(err)
        }
        v2, err := strconv.Atoi(line_split[1])
        if err != nil {
            log.Fatal(err)
        }
        rule := [2]int{v1,v2}
        rules_list = append(rules_list, rule)
    }

    rules := make(Rules)
    
    for _,v := range rules_list {
        _, exists := rules[v[0]]
        if exists {
            rules[v[0]] = append(rules[v[0]], v[1])
        } else {
            rules[v[0]] = []int{v[1]}
        }
    }

    return rules

}

func shouldSwap(first int, second int, rules Rules) bool{

    swap := false
    _, exists := rules[second]
    if exists {
        for _, v := range rules[second] {
            if first == v {
                swap = true
                break
            }
        }
    }
    return swap
}

func bubbleSort(arr []int, rules Rules) int {
    
    var steps int = 0
    for {
        swapped := false
        for i := 0; i < len(arr)-1; i++ {
            if shouldSwap(arr[i], arr[i+1], rules) {
                steps += 1
                arr[i], arr[i+1] = arr[i+1], arr[i]
                swapped = true
            }
        }
        if !swapped {
            break
        }
    }
    return steps
}

func main() {

    var rules Rules = readRulesFile("rules.txt")
    var updates [][]int = readUpdatesFile("updates.txt") 
   
    var total int = 0
    for _,v := range updates {
        steps := bubbleSort(v, rules)
        if steps == 0 {
            total += v[len(v)/2]
        }
    }
    fmt.Println(total)

}
