package day264

// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
func letterCombinations(digits string) []string {
	mapping := map[rune][]rune{}
	mapping['2'] = []rune("abc")
	mapping['3'] = []rune("def")
	mapping['4'] = []rune("ghi")
	mapping['5'] = []rune("jkl")
	mapping['6'] = []rune("mno")
	mapping['7'] = []rune("pqrs")
	mapping['8'] = []rune("tuv")
	mapping['9'] = []rune("wxyz")

	ans := []string{}
	cur := []rune{}

	var rec func(int)
	rec = func(i int) {
		if i == len(digits) {
			dst := ""
			if string(cur) == dst {
				return
			}
			for _, ch := range cur {
				dst += string(ch)
			}
			ans = append(ans, dst)
			return
		}

		for _, ch := range mapping[rune(digits[i])] {
			cur = append(cur, ch)
			rec(i + 1)
			cur = cur[:len(cur)-1]
		}

	}

	rec(0)

	return ans
}

// https://leetcode.com/problems/design-linked-list/description/
type Node struct {
	val  int
	next *Node
}

type MyLinkedList struct {
	head *Node
}

func Constructor() MyLinkedList {
	return MyLinkedList{nil}
}

func (this *MyLinkedList) getNode(index int) *Node {
	cur := this.head
	for cur != nil && index > 0 {
		cur = cur.next
		index--
	}
	return cur
}

func (this *MyLinkedList) getTail() *Node {
	cur := this.head
	for cur != nil && cur.next != nil {
		cur = cur.next
	}
	return cur
}

func (this *MyLinkedList) Get(index int) int {
	cur := this.getNode(index)
	if cur == nil {
		return -1
	}
	return cur.val
}

func (this *MyLinkedList) AddAtHead(val int) {
	this.head = &Node{val, this.head}
}

func (this *MyLinkedList) AddAtTail(val int) {
	if this.head == nil {
		this.AddAtHead(val)
		return
	}
	tail := this.getTail()
	tail.next = &Node{val, nil}
}

func (this *MyLinkedList) AddAtIndex(index int, val int) {
	if index == 0 {
		this.AddAtHead(val)
		return
	}
	prev := this.getNode(index - 1)
	if prev == nil {
		return
	}
	cur := &Node{val, prev.next}
	prev.next = cur
}

func (this *MyLinkedList) DeleteAtIndex(index int) {
	cur := this.getNode(index)
	if cur == nil {
		return
	}
	if index == 0 {
		this.head = cur.next
	} else {
		prev := this.getNode(index - 1)
		prev.next = cur.next
	}
}
