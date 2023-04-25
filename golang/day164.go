package main

import (
	"fmt"
	"math"
	"sort"
)

// https://leetcode.com/problems/implement-trie-prefix-tree/description/
type node struct {
	isEnd    bool
	children []*node
}

func newNode() *node {
	return &node{children: make([]*node, 26)}
}

func (this *node) insert(c rune) {
	this.children[c-'a'] = newNode()
}

func (this *node) contains(c rune) bool {
	return this.get(c) != nil
}

func (this *node) get(c rune) *node {
	return this.children[c-'a']
}

type Trie struct {
	root *node
}

func Constructor() Trie {
	return Trie{root: newNode()}
}

func (this *Trie) Insert(word string) {
	node := this.root
	for _, c := range word {
		if !node.contains(c) {
			node.insert(c)
		}
		node = node.get(c)
	}
	node.isEnd = true
}

func (this *Trie) Search(word string) bool {
	node := this.root
	for _, c := range word {
		if !node.contains(c) {
			return false
		}
		node = node.get(c)
	}
	return node.isEnd
}

func (this *Trie) StartsWith(prefix string) bool {
	node := this.root
	for _, c := range prefix {
		if !node.contains(c) {
			return false
		}
		node = node.get(c)
	}
	return true
}

func testTrie() {
	trie := Constructor()
	trie.Insert("apple")
	println(trie.Search("apple"))
	println(trie.Search("app"))
	println(trie.StartsWith("app"))
	trie.Insert("app")
	println(trie.Search("app"))
}

// https://leetcode.com/problems/meeting-rooms/description/
func canAttendMeetings(intervals [][]int) bool {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	for i := 1; i < len(intervals); i++ {
		if intervals[i][0] < intervals[i-1][1] {
			return false
		}
	}
	return true
}

func testCanAttendMeeting() {
	println(canAttendMeetings([][]int{{0, 30}, {5, 10}, {15, 20}}))
}

// https://leetcode.com/problems/insert-interval/description/
func insert(intervals [][]int, newInterval []int) [][]int {
	insert := func(intervals [][]int, newInterval []int) [][]int {
		inserted := false
		for i := 0; i < len(intervals); i++ {
			if newInterval[0] < intervals[i][0] {
				tmp := make([][]int, len(intervals)+1)
				copy(tmp[:i], intervals[:i])
				tmp[i] = newInterval
				copy(tmp[i+1:], intervals[i:])
				intervals = tmp
				inserted = true
				break
			}
		}
		if !inserted {
			intervals = append(intervals, newInterval)
		}
		return intervals
	}
	overlap := func(a, b []int) bool {
		return int(math.Min(float64(a[1]), float64(b[1])))-int(math.Max(float64(a[0]), float64(b[0]))) >= 0
	}
	merge := func(a, b []int) []int {
		return []int{int(math.Min(float64(a[0]), float64(b[0]))), int(math.Max(float64(a[1]), float64(b[1])))}
	}

	intervals = insert(intervals, newInterval)
	answer := make([][]int, 0)
	for i := 0; i < len(intervals); i++ {
		currentInterval := intervals[i]
		for i < len(intervals) && overlap(currentInterval, intervals[i]) {
			currentInterval = merge(currentInterval, intervals[i])
			i++
		}
		i--
		answer = append(answer, currentInterval)
	}
	return answer
}

func testInsert() {
	fmt.Println(insert([][]int{{1, 3}, {6, 9}}, []int{2, 5}))
}

func main() {
	testTrie()
	testCanAttendMeeting()
	testInsert()
}
