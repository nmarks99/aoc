package main

import (
    "fmt"
    "bufio"
    "os"
    "log"
    "math"
    // "time"
)

func getLabMap(filename string) [][]int {
    file, err := os.Open(filename)
    if err != nil {
        log.Fatal(err)
    }
   
    var out[][]int
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        var line_tmp []int
        var line string = scanner.Text()
        for _, val := range line {
            line_tmp = append(line_tmp, int(val))
        }
        out = append(out, line_tmp)
    }

    return out
}

type Heading int8;

const (
    North Heading = iota
    South
    East
    West
)

type Guard struct {
    row, col int
    heading Heading
    in_map bool
    visited [][]int
}

func NewGuard(row0 int, col0 int) Guard {
    return Guard {
        row : row0,
        col : col0,
        heading: North,
        in_map : true,
    }
}

func (g Guard) PrintInfo() {
    var heading_str string
    switch g.heading {
    case North:
        heading_str = "North"
    case South:
        heading_str = "South"
    case East:
        heading_str = "East"
    case West:
        heading_str = "West"
    }
    if g.in_map {
        fmt.Printf("x = %d\ny = %d\nHeading = %s\n", g.row, g.col, heading_str)
    } else {
        fmt.Println("Guard is no longer in the mapped region")
    }
}

func (g *Guard) addIfUnique(point [2]int) {
    var unique bool = true
   
    if len(g.visited) > 0 {
        for i := 0; i < len(g.visited[0]); i++ {
            for j := 0; j < 2; j++ {
                if i == point[0] && j == point[1] {
                    unique = false
                    break
                }
            }
        }
    }
    
    if unique {
        g.visited = append(g.visited, point[:])
    }
}

func (g *Guard) Step(lab_map [][]int) {
    
    x_check, y_check := g.row, g.col

    switch g.heading {
    case North:
        x_check -= 1 
    case South:
        x_check += 1 
    case East:
        y_check -= 1 
    case West:
        y_check += 1
    }

    // check if the next position will be out of the map
    if x_check >= len(lab_map[0]) || y_check >= len(lab_map[1]) {
        g.in_map = false
    } else if lab_map[x_check][y_check] == '#' {
        if g.heading == 3 {
            g.heading = 0
        } else {
            g.heading += 1
        }
    } else {
        g.row = x_check
        g.col = y_check
        g.addIfUnique([2]int{g.row, g.col})
    }

}

func linearDistance(x1 int, x2 int, y1 int, y2 int) float64 {
    return math.Sqrt( math.Pow(float64(x2-x1), 2) + math.Pow(float64(y2-y1), 2) )
}

func printRed(text string) {
    fmt.Print("\033[31m")
    fmt.Print(text)
    fmt.Print("\033[0m")
}

func drawMap(g Guard, lab_map [][]int) {
    for i := 0; i < len(lab_map[0]); i++ {
        for j := 0; j < len(lab_map[1]); j++ {
            if i == g.row && j == g.col {
                var guard_char int
                switch g.heading {
                case North:
                    guard_char = '^'
                case South:
                    guard_char = 'v'
                case East:
                    guard_char = '>'
                case West:
                    guard_char = '<'
                }
                printRed(string(guard_char))
            } else {
                fmt.Print(string(lab_map[i][j]))
            }
        }
        fmt.Println()
    }
}

func clearScreen() {
    fmt.Print("\033[H\033[2J")
}

func main() {
    lab_map := getLabMap("./test_map.txt")
   
    fmt.Println(len(lab_map[0]))
    fmt.Println(len(lab_map[0]))

    row0 := 0
    col0 := 0
    for i := 0; i < len(lab_map[0]); i++ {
        for j := 0; j < len(lab_map[1]); j++ {
            if lab_map[i][j] == '^' {
                row0 = i
                col0 = j
            }
        }
    }

    guard := NewGuard(row0, col0)
    for {
        if !guard.in_map {
            break
        }
        guard.Step(lab_map)
        // clearScreen()
        // drawMap(guard, lab_map)
        // time.Sleep(time.Millisecond * 100)
        // guard.PrintInfo()
    }
    // fmt.Println(len(guard.visited))
}
