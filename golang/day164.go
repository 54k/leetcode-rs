package main

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

/**
 * Your Trie object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Insert(word);
 * param_2 := obj.Search(word);
 * param_3 := obj.StartsWith(prefix);
 */

func main() {
	trie := Constructor()
	trie.Insert("apple")
	println(trie.Search("apple"))
	println(trie.Search("app"))
	println(trie.StartsWith("app"))
	trie.Insert("app")
	println(trie.Search("app"))
}
