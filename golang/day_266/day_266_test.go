package day266

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/unique-binary-search-trees-ii/description/
func generateTreesTopDown(n int) []*TreeNode {
	type pair struct {
		left  int
		right int
	}
	memo := map[pair][]*TreeNode{}

	var generate func(int, int) []*TreeNode
	generate = func(start, end int) []*TreeNode {
		res := []*TreeNode{}

		if start > end {
			res = append(res, nil)
			return res
		}

		key := pair{start, end}
		if _, ok := memo[key]; ok {
			return memo[key]
		}

		for i := start; i <= end; i++ {
			leftSubTree := generate(start, i-1)
			rightSubTree := generate(i+1, end)

			for _, l := range leftSubTree {
				for _, r := range rightSubTree {
					res = append(res, &TreeNode{i, l, r})
				}
			}
		}

		memo[key] = res
		return res
	}

	return generate(1, n)
}

func generateTrees(n int) []*TreeNode {

	var clone func(*TreeNode, int) *TreeNode
	clone = func(node *TreeNode, offset int) *TreeNode {
		if node == nil {
			return nil
		}
		cloned := &TreeNode{node.Val + offset, nil, nil}
		cloned.Left = clone(node.Left, offset)
		cloned.Right = clone(node.Right, offset)
		return cloned
	}

	dp := make([][]*TreeNode, n+1)
	for i := 0; i <= n; i++ {
		dp[i] = []*TreeNode{}
	}
	dp[0] = append(dp[0], nil)

	for numberOfNodes := 1; numberOfNodes <= n; numberOfNodes++ {
		for i := 1; i <= numberOfNodes; i++ {
			j := numberOfNodes - i
			for _, left := range dp[i-1] {
				for _, right := range dp[j] {
					root := &TreeNode{i, left, clone(right, i)}
					dp[numberOfNodes] = append(dp[numberOfNodes], root)
				}
			}
		}
	}

	return dp[n]
}

// https://leetcode.com/problems/range-sum-query-mutable/description/
type NumArray struct {
	tree []int
	k    int
}

func Constructor(nums []int) NumArray {
	arr := NumArray{make([]int, 2*len(nums)), len(nums)}
	for i, n := range nums {
		arr.Update(i, n)
	}
	return arr
}

func (this *NumArray) Update(index int, val int) {
	index += this.k
	this.tree[index] = val
	for i := index; i > 0; {
		i /= 2
		this.tree[i] = this.tree[i*2] + this.tree[i*2+1]
	}
}

func (this *NumArray) SumRange(left int, right int) int {
	left += this.k
	right += this.k
	sum := 0
	for left <= right {
		if left%2 == 1 {
			sum += this.tree[left]
			left++
		}
		if right%2 == 0 {
			sum += this.tree[right]
			right--
		}
		left /= 2
		right /= 2
	}
	return sum
}
