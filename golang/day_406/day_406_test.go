package day406

// https://leetcode.com/problems/kth-ancestor-of-a-tree-node/description/
type TreeAncestor struct {
	maxd      int
	kthparent [][]int
}

func Constructor(n int, parent []int) TreeAncestor {
	var maxd = 0
	for (1 << maxd) <= n {
		maxd++
	}

	var kthparent = make([][]int, n)
	for i := 0; i < len(kthparent); i++ {
		kthparent[i] = make([]int, maxd)
		for j := 0; j < maxd; j++ {
			kthparent[i][j] = -1
		}
	}

	for i := 0; i < maxd; i++ {
		for j := 0; j < n; j++ {
			if i == 0 {
				kthparent[j][i] = parent[j]
			} else if kthparent[j][i-1] != -1 {
				kthparent[j][i] = kthparent[kthparent[j][i-1]][i-1]
			}
		}
	}

	return TreeAncestor{maxd, kthparent}
}

func (this *TreeAncestor) GetKthAncestor(node int, k int) int {
	for i := 0; i < this.maxd; i++ {
		if ((1 << i) & k) != 0 {
			node = this.kthparent[node][i]
			if node == -1 {
				break
			}
		}
	}
	return node
}
