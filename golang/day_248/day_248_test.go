package day_248

import (
	"fmt"
	"testing"
)

type Node struct {
	key  int
	val  int
	next *Node
	prev *Node
}

type DLL struct {
	head *Node
	tail *Node
}

func newDLL() *DLL {
	head := &Node{-1, -1, nil, nil}
	tail := &Node{-1, -1, nil, nil}
	head.next = tail
	head.prev = head
	tail.prev = head
	tail.next = tail
	return &DLL{head, tail}
}

func (this *DLL) add(node *Node) {
	prevEnd := this.tail.prev
	prevEnd.next = node
	node.prev = prevEnd
	node.next = this.tail
	this.tail.prev = node
}

func (this *DLL) remove(node *Node) {
	node.prev.next = node.next
	node.next.prev = node.prev
}

type LRUCache struct {
	dic      map[int]*Node
	dll      *DLL
	capacity int
}

func Constructor(capacity int) LRUCache {
	return LRUCache{map[int]*Node{}, newDLL(), capacity}
}

func (this *LRUCache) Get(key int) int {
	if _, ok := this.dic[key]; !ok {
		return -1
	}

	node := this.dic[key]
	this.dll.remove(node)
	this.dll.add(node)
	return node.val
}

func (this *LRUCache) Put(key int, value int) {
	if _, ok := this.dic[key]; ok {
		this.dll.remove(this.dic[key])
		delete(this.dic, key)
	}

	node := &Node{key, value, nil, nil}
	this.dll.add(node)
	this.dic[key] = node

	if len(this.dic) > this.capacity {
		toRem := this.dll.head.next
		this.dll.remove(toRem)
		delete(this.dic, toRem.key)
	}
}

// https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters/description/
func lengthOfLongestSubstringKDistinct(s string, k int) int {
	isValid := func(s string, size int) bool {
		n := len(s)
		counter := map[rune]int{}
		for i := 0; i < size; i++ {
			c := rune(s[i])
			counter[c]++
		}

		if len(counter) <= k {
			return true
		}

		for i := size; i < n; i++ {
			c1 := rune(s[i])
			counter[c1]++

			c2 := rune(s[i-size])
			counter[c2]--

			if counter[c2] == 0 {
				delete(counter, c2)
			}

			if len(counter) <= k {
				return true
			}
		}

		return false
	}

	n := len(s)
	if k >= n {
		return n
	}
	left, right := k, n
	for left < right {
		mid := (left + right + 1) / 2

		if isValid(s, mid) {
			left = mid
		} else {
			right = mid - 1
		}
	}

	return left
}

func TestLRUCache(t *testing.T) {
	lru := Constructor(2)
	lru.Put(1, 1)
	lru.Put(2, 2)
	fmt.Println(lru.Get(1))
	lru.Put(3, 3)
	lru.Put(4, 4)
	fmt.Println(lru.Get(1))
	fmt.Println(lru.Get(3))
	fmt.Println(lru.Get(4))
}
