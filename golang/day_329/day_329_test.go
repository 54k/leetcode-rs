package day329

// https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/description
func numOfArrays(n int, m int, k int) int {
	const MOD = 1000000007

	memo := make([][][]int, n)
	for i := 0; i < n; i++ {
		memo[i] = make([][]int, m+1)
		for j := 0; j <= m; j++ {
			memo[i][j] = make([]int, k+1)
			for z := 0; z <= k; z++ {
				memo[i][j][z] = -1
			}
		}
	}

	var dp func(int, int, int) int
	dp = func(i, maxSoFar, remain int) int {
		if i == n {
			if remain == 0 {
				return 1
			}
			return 0
		}

		if remain < 0 {
			return 0
		}

		if memo[i][maxSoFar][remain] != -1 {
			return memo[i][maxSoFar][remain]
		}

		var ans int = 0
		for num := 1; num <= maxSoFar; num++ {
			ans = (ans%MOD + dp(i+1, maxSoFar, remain)%MOD) % MOD
		}

		for num := maxSoFar + 1; num <= m; num++ {
			ans = (ans%MOD + dp(i+1, num, remain-1)%MOD) % MOD
		}

		memo[i][maxSoFar][remain] = ans
		return ans
	}

	return dp(0, 0, k) % MOD
}

// https://leetcode.com/problems/encode-n-ary-tree-to-binary-tree/description/
type Node struct {
	Val      int
	Children []*Node
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Codec struct {
}

func Constructor() *Codec {
	return &Codec{}
}

func (this *Codec) encode(root *Node) *TreeNode {
	if root == nil {
		return nil
	}

	newRoot := &TreeNode{root.Val, nil, nil}
	if len(root.Children) > 0 {
		newRoot.Left = this.encode(root.Children[0])
	}
	sibling := newRoot.Left
	for i := 1; i < len(root.Children); i++ {
		sibling.Right = this.encode(root.Children[i])
		sibling = sibling.Right
	}
	return newRoot
}

func (this *Codec) decode(root *TreeNode) *Node {
	if root == nil {
		return nil
	}
	newRoot := &Node{root.Val, []*Node{}}
	sibling := root.Left
	for sibling != nil {
		newRoot.Children = append(newRoot.Children, this.decode(sibling))
		sibling = sibling.Right
	}
	return newRoot
}
