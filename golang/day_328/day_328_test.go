package day328

// https://leetcode.com/problems/integer-break/description/
func integerBreak(n int) int {
	memo := map[int]int{}

	var dp func(int) int
	dp = func(num int) int {
		if num <= 3 {
			return num
		}

		if _, ok := memo[num]; ok {
			return memo[num]
		}

		ans := num // don't split at all

		for i := 2; i < num; i++ {
			split := i * dp(num-i)
			if split > ans {
				ans = split
			}
		}
		memo[num] = ans
		return ans
	}

	if n <= 3 {
		return n - 1
	}
	return dp(n)
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
		firstChild := root.Children[0]
		newRoot.Left = this.encode(firstChild)
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
