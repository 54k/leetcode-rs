package day263

import (
	"strconv"
	"strings"
)

// https://leetcode.com/problems/permutations/
func permute(nums []int) [][]int {
	cur := []int{}
	ans := [][]int{}

	var backtrack func(int, int)
	backtrack = func(pos, mask int) {
		if pos == len(nums) {
			dst := make([]int, len(nums))
			copy(dst, cur)
			ans = append(ans, dst)
			return
		}
		for i, n := range nums {
			if mask&(1<<i) == 0 {
				cur = append(cur, n)
				backtrack(pos+1, mask|(1<<i))
				cur = cur[:len(cur)-1]
			}
		}
	}

	backtrack(0, 0)
	return ans
}

// https://leetcode.com/problems/permutations-ii/description/
func permuteUnique(nums []int) [][]int {
	counter := map[int]int{}
	for _, num := range nums {
		counter[num] += 1
	}
	cur := []int{}
	ans := [][]int{}

	var backtrack func()
	backtrack = func() {
		if len(cur) == len(nums) {
			dst := make([]int, len(cur))
			copy(dst, cur)
			ans = append(ans, dst)
			return
		}

		for num, cnt := range counter {
			if cnt == 0 {
				continue
			}
			cur = append(cur, num)
			counter[num]--
			backtrack()
			counter[num]++
			cur = cur[:len(cur)-1]
		}
	}

	backtrack()
	return ans
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Codec struct {
}

func Constructor() Codec {
	return Codec{}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *TreeNode) string {
	if root == nil {
		return "nil,"
	}
	res := strconv.Itoa(root.Val) + ","
	res += this.serialize(root.Left)
	res += this.serialize(root.Right)
	return res
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {
	split := strings.Split(data, ",")
	split = split[:len(split)-1]
	i := -1

	var des func() *TreeNode
	des = func() *TreeNode {
		i++
		if split[i] == "nil" {
			return nil
		}

		val, _ := strconv.Atoi(split[i])
		root := &TreeNode{val, nil, nil}
		root.Left = des()
		root.Right = des()

		return root
	}

	return des()
}
