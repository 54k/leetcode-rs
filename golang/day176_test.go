package main

import (
	"fmt"
	"sort"
	"strconv"
	"testing"
)

func groupAnagrams(strs []string) [][]string {
	f := func(s string) string {
		bytes := []byte(s)
		sort.Slice(bytes[:], func(i, j int) bool {
			return bytes[i] < bytes[j]
		})
		return string(bytes)
	}
	m := make(map[string][]string)
	for _, s := range strs {
		hash := f(s)
		m[hash] = append(m[hash], s)
	}
	ans := [][]string{}
	for _, v := range m {
		ans = append(ans, v)
	}
	return ans
}

func groupStrings(strings []string) [][]string {
	h := func(s string) string {
		hash := make([]byte, len(s))
		bytes := []byte(s)
		for i := 1; i < len(bytes); i++ {
			hash[i] = (bytes[i] - bytes[i-1] + 26) % 26
		}
		return string(hash)
	}
	m := make(map[string][]string)
	for _, s := range strings {
		hash := h(s)
		m[hash] = append(m[hash], s)
	}
	ans := [][]string{}
	for _, v := range m {
		ans = append(ans, v)
	}
	return ans
}

// https://leetcode.com/problems/valid-sudoku/description/
func isValidSudoku(board [][]byte) bool {
	rows := [9]int{}
	cols := [9]int{}
	subs := [3][3]int{{}, {}, {}}
	for row := 0; row < len(board); row++ {
		for col := 0; col < len(board[0]); col++ {
			if board[row][col] == '.' {
				continue
			}
			if n, err := strconv.Atoi(string(board[row][col])); err == nil {
				k := 1 << n
				if ((rows[row] & k) | (cols[col] & k) | (subs[row/3][col/3] & k)) == k {
					return false
				}
				rows[row] |= k
				cols[col] |= k
				subs[row/3][col/3] |= k
			}
		}
	}
	return true
}

// https://leetcode.com/problems/find-duplicate-subtrees/description/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	m := map[string]int{}
	var dfs func(*TreeNode, *[]*TreeNode) string
	dfs = func(node *TreeNode, res *[]*TreeNode) string {
		if node == nil {
			return ""
		}
		key := fmt.Sprintf("(%s)%d(%s)", dfs(node.Left, res), node.Val, dfs(node.Right, res))
		m[key]++
		if m[key] == 2 {
			*res = append(*res, node)
		}
		return key
	}
	res := []*TreeNode{}
	dfs(root, &res)
	return res
}

func TestGroupAnagrams(t *testing.T) {
	res := groupAnagrams([]string{"eat", "tea", "tan", "ate", "nat", "bat"})
	fmt.Println(res)
}

func TestGroupStrings(t *testing.T) {
	res := groupStrings([]string{"abc", "xyz", "acd", "wyz"})
	fmt.Println(res)
}

func TestSudoku(t *testing.T) {
	board := [][]string{
		{"5", "3", ".", ".", "7", ".", ".", ".", "."},
		{"6", ".", ".", "1", "9", "5", ".", ".", "."},
		{".", "9", "8", ".", ".", ".", ".", "6", "."},
		{"8", ".", ".", ".", "6", ".", ".", ".", "3"},
		{"4", ".", ".", "8", ".", "3", ".", ".", "1"},
		{"7", ".", ".", ".", "2", ".", ".", ".", "6"},
		{".", "6", ".", ".", ".", ".", "2", "8", "."},
		{".", ".", ".", "4", "1", "9", ".", ".", "5"},
		{".", ".", ".", ".", "8", ".", ".", "7", "9"},
	}
	b := [][]byte{}
	for i := 0; i < len(board); i++ {
		b = append(b, make([]byte, 9))
		for j := 0; j < len(b); j++ {
			b[i][j] = board[i][j][0]
		}
	}
	fmt.Println(isValidSudoku(b)) // true
}
