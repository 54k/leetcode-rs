package day285

import "math"

// https://leetcode.com/problems/text-justification/description/
func fullJustify(words []string, maxWidth int) []string {
	getWords := func(i int) []string {
		currLength := 0
		currentLine := []string{}
		for i < len(words) && currLength+len(words[i]) <= maxWidth {
			currLength += len(words[i]) + 1
			currentLine = append(currentLine, words[i])
			i++
		}
		return currentLine
	}

	createLine := func(line []string, wordsIdx int) {
		baseLength := -1
		for _, word := range line {
			baseLength += len(word) + 1
		}
		extraSpaces := maxWidth - baseLength

		if len(line) == 1 || wordsIdx == len(words) {
			for i := 0; i < len(line)-1; i++ {
				line[i] += " "
			}
			for i := 0; i < extraSpaces; i++ {
				line[len(line)-1] += " "
			}
			return
		}

		wordsCount := len(line) - 1
		spacePerWord := extraSpaces / wordsCount
		needsExtraSpaces := extraSpaces % wordsCount

		for i := 0; i < needsExtraSpaces; i++ {
			line[i] += " "
		}

		for i := 0; i < wordsCount; i++ {
			for j := 0; j < spacePerWord; j++ {
				line[i] += " "
			}
		}

		for i := 0; i < len(line)-1; i++ {
			line[i] += " "
		}
	}

	ans := []string{}
	i := 0
	for i < len(words) {
		currentLine := getWords(i)
		i += len(currentLine)
		createLine(currentLine, i)
		strLine := ""
		for _, word := range currentLine {
			strLine += word
		}
		ans = append(ans, strLine)
	}
	return ans
}

type Node struct {
	pre, nxt *Node
	val      int
}

func NewNode(val int) *Node {
	node := &Node{nil, nil, val}
	node.pre = node
	node.nxt = node
	return node
}

func (this *Node) append(node *Node) {
	tmp := this.nxt
	this.nxt = node
	node.pre = this
	node.nxt = tmp
	tmp.pre = node
}

func (this *Node) remove() *Node {
	this.pre.nxt = this.nxt
	this.nxt.pre = this.pre
	this.nxt = this
	this.pre = this
	return this
}

type MRUQueue struct {
	nodes  []*Node
	bucket int
}

func Constructor(n int) MRUQueue {
	bucket := int(math.Sqrt(float64(n)))
	nodes := make([]*Node, (n+bucket-1)/bucket)
	for i := 0; i < len(nodes); i++ {
		nodes[i] = NewNode(-1)
	}
	for i := 1; i <= n; i++ {
		nodes[(i-1)/bucket].pre.append(NewNode(i))
	}
	return MRUQueue{nodes, bucket}
}

func (this *MRUQueue) Fetch(k int) int {
	k--
	ans := this.nodes[k/this.bucket].nxt
	for i := k % this.bucket; i > 0; i-- {
		ans = ans.nxt
	}
	ans.remove()
	for i := 1 + k/this.bucket; i < len(this.nodes); i++ {
		this.nodes[i-1].pre.append(this.nodes[i].nxt.remove())
	}
	this.nodes[len(this.nodes)-1].pre.append(ans)
	return ans.val
}
