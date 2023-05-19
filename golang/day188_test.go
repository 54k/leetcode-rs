package main

// https://leetcode.com/problems/robot-room-cleaner/description/

import (
	"fmt"
	"testing"
)

// This is the robot's control interface.
// You should not implement it, or speculate about its implementation
type Robot struct {
}

// Returns true if the cell in front is open and robot moves into the cell.
// Returns false if the cell in front is blocked and robot stays in the current cell.
func (robot *Robot) Move() bool { return false }

// Robot will stay in the same cell after calling TurnLeft/TurnRight.
// Each turn will be 90 degrees.
func (robot *Robot) TurnLeft()  {}
func (robot *Robot) TurnRight() {}

// Clean the current cell.
func (robot *Robot) Clean() {}

func goBack(robo *Robot) {
	robo.TurnRight()
	robo.TurnRight()
	robo.Move()
	robo.TurnRight()
	robo.TurnRight()
}

func backtrack(row, col, d int, directions [][]int, visited map[string]bool, robo *Robot) {
	visited[fmt.Sprintf("%d%d", row, col)] = true
	robo.Clean()

	// going clockwise : 0 'up', 1 'right' 2 'down' 3 'left'
	for i := 0; i < 4; i++ {
		newD := (d + i) % 4
		newRow := row + directions[newD][0]
		newCol := col + directions[newD][1]

		if !visited[fmt.Sprintf("%d%d", newRow, newCol)] && robo.Move() {
			backtrack(newRow, newCol, newD, directions, visited, robo)
			goBack(robo)
		}

		robo.TurnRight()
	}
}

func cleanRoom(robot *Robot) {
	directions := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	visited := map[string]bool{}
	robo := robot
	backtrack(0, 0, 0, directions, visited, robo)
}

// https://leetcode.com/problems/is-graph-bipartite/description/
func dfs(v int, graph [][]int, colors map[int]int) bool {
	for _, u := range graph[v] {
		if _, ok := colors[u]; !ok {
			colors[u] = 1 - colors[v]
			if !dfs(u, graph, colors) {
				return false
			}
		} else if colors[v] == colors[u] {
			return false
		}
	}
	return true
}

func isBipartite(graph [][]int) bool {
	colors := map[int]int{}
	for v := 0; v < len(graph); v++ {
		if _, ok := colors[v]; !ok {
			colors[v] = 0
			if !dfs(v, graph, colors) {
				return false
			}
		}
	}
	return true
}

// https://leetcode.com/problems/sudoku-solver/description/
func sudokuBacktrack(board [][]byte, row, col int, cache map[string]bool) {
	for board[row][col] != '.' {
		if col < len(board[0]) {
			col++
		} else {
			row++
			col = 0
		}
	}

	if row >= len(board) {
		return
	}

	for num := 0; num < 10; num++ {
		val := byte(rune(num))

		if cache[fmt.Sprintf("r %d v %d", row, val)] ||
			cache[fmt.Sprintf("c %d v %d", col, val)] ||
			cache[fmt.Sprintf("r %d c %d v %d", row/3, col/3, val)] {
			continue
		}

		cache[fmt.Sprintf("r %d v %d", row, val)] = true
		cache[fmt.Sprintf("c %d v %d", col, val)] = true
		cache[fmt.Sprintf("r %d c %d v %d", row/3, col/3, val)] = true
		board[row][col] = val

		sudokuBacktrack(board, row, col, cache)

		cache[fmt.Sprintf("r %d v %d", row, val)] = false
		cache[fmt.Sprintf("c %d v %d", col, val)] = false
		cache[fmt.Sprintf("r %d c %d v %d", row/3, col/3, val)] = false
		board[row][col] = '.'
	}
}

func solveSudoku(board [][]byte) {
	cache := map[string]bool{}
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board); j++ {
			if board[i][j] != '.' {
				cache[fmt.Sprintf("r %d v %d", i, board[i][j])] = true
				cache[fmt.Sprintf("c %d v %d", j, board[i][j])] = true
				cache[fmt.Sprintf("r %d c %d v %d", i/3, j/3, board[i][j])] = true
			}
		}
	}
	sudokuBacktrack(board, 0, 0, cache)
}

// https://leetcode.com/problems/combinations/description/
func combineRec(n, k, i int, cur *[]int, res *[][]int) {
	if len(*cur) == k {
		dst := make([]int, len(*cur))
		copy(dst, *cur)
		*res = append(*res, dst)
		return
	}

	for j := i; j <= n; j++ {
		*cur = append(*cur, j)
		combineRec(n, k, j+1, cur, res)
		*cur = (*cur)[:len(*cur)-1]
	}
}

func combine(n int, k int) [][]int {
	cur := []int{}
	res := [][]int{}
	combineRec(n, k, 1, &cur, &res)
	return res
}

// https://leetcode.com/problems/generate-parentheses/description/
func generateParenthesis(n int) []string {
	ans := []string{}
	if n == 0 {
		ans = append(ans, "")
	} else {
		for i := 0; i < n; i++ {
			for _, left := range generateParenthesis(i) {
				for _, right := range generateParenthesis(n - i - 1) {
					ans = append(ans, fmt.Sprintf("(%s)%s", left, right))
				}
			}
		}
	}
	return ans
}

type Node struct {
	Val   int
	Left  *Node
	Right *Node
}

func treeToDoublyList(root *Node) *Node {
	if root == nil {
		return nil
	}
	dummy := &Node{Val: -1}
	prev := dummy.Right
	stack := []*Node{}

	for len(stack) > 0 || root != nil {
		for root != nil {
			stack = append(stack, root)
			root = root.Left
		}
		root = stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if prev != nil {
			prev.Right = root
			root.Left = prev
		} else {
			dummy.Right = root
		}
		prev = root

		root = root.Right
	}

	prev.Right = dummy.Right
	dummy.Right.Left = prev

	return dummy.Right
}

func TestTreeToDoublyList(t *testing.T) {
	tree := &Node{
		Val: 4,
		Left: &Node{
			Val: 2,
			Left: &Node{
				Val:   1,
				Left:  nil,
				Right: nil,
			},
			Right: &Node{
				Val:   3,
				Left:  nil,
				Right: nil,
			},
		},
		Right: &Node{
			Val:   5,
			Left:  nil,
			Right: nil,
		},
	}

	list := treeToDoublyList(tree)
	for list != nil {
		fmt.Println(list)
		list = list.Right
	}

	tree = &Node{
		Val: 2,
		Left: &Node{
			Val:   1,
			Left:  nil,
			Right: nil,
		},
		Right: &Node{
			Val:   3,
			Left:  nil,
			Right: nil,
		},
	}

	list = treeToDoublyList(tree)
	for list != nil {
		fmt.Println(list)
		list = list.Right
	}
}

// https://leetcode.com/problems/permutations/description/
func recPermute(mask int, nums []int, cur *[]int, ans *[][]int) {
	fmt.Println("mask ", mask)
	if len(*cur) == len(nums) {
		dst := make([]int, len(nums))
		copy(dst, *cur)
		*ans = append(*ans, dst)
		return
	}

	for i := 0; i < len(nums); i++ {
		if mask&(1<<i) == 0 {
			*cur = append(*cur, nums[i])
			recPermute(mask|(1<<i), nums, cur, ans)
			*cur = (*cur)[:len(*cur)-1]
		}
	}
}

func permute(nums []int) [][]int {
	ans := [][]int{}
	recPermute(0, nums, &[]int{}, &ans)
	return ans
}

func TestRecPermute(t *testing.T) {
	fmt.Println(permute([]int{1, 2, 3})) // [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
}
