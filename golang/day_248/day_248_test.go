package day_248

import (
	"fmt"
	"sort"
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
func lengthOfLongestSubstringKDistinctBinSearch(s string, k int) int {
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

func lengthOfLongestSubstringKDistinctSlidingWindow(s string, k int) int {
	n := len(s)
	maxSize := 0
	counter := map[byte]int{}

	for left, right := 0, 0; right < n; right++ {
		counter[s[right]]++

		for len(counter) > k {
			counter[s[left]]--
			if counter[s[left]] == 0 {
				delete(counter, s[left])
			}
			left++
		}

		if right-left+1 > maxSize {
			maxSize = right - left + 1
		}
	}

	return maxSize
}

func lengthOfLongestSubstringKDistinctMaxSlidingWindow(s string, k int) int {
	n := len(s)
	maxSize := 0
	counter := map[byte]int{}

	for right := 0; right < n; right++ {
		counter[s[right]]++

		if len(counter) <= k {
			maxSize++
		} else {
			counter[s[right-maxSize]]--
			if counter[s[right-maxSize]] == 0 {
				delete(counter, s[right-maxSize])
			}
		}
	}

	return maxSize
}

// https://leetcode.com/problems/subarray-sum-equals-k/description/
func subarraySum(nums []int, k int) int {
	sarr := map[int]int{}
	sarr[0] = 1
	count, sum := 0, 0
	for _, n := range nums {
		sum += n
		count += sarr[sum-k]
		sarr[sum]++
	}
	return count
}

// https://leetcode.com/problems/merge-intervals/description/
func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(a, b int) bool {
		return intervals[a][0] < intervals[b][0]
	})

	ans := [][]int{intervals[0]}

	for i := 1; i < len(intervals); i++ {
		if intervals[i][0] <= ans[len(ans)-1][1] {
			if intervals[i][1] > ans[len(ans)-1][1] {
				ans[len(ans)-1][1] = intervals[i][1]
			}
		} else {
			ans = append(ans, intervals[i])
		}
	}

	return ans
}

// https://leetcode.com/problems/candy/submissions/
func candy(ratings []int) int {
	candies := make([]int, len(ratings))
	for i := 0; i < len(candies); i++ {
		candies[i] = 1
	}

	for i := 1; i < len(ratings); i++ {
		if ratings[i] > ratings[i-1] {
			candies[i] = candies[i-1] + 1
		}
	}

	sum := candies[len(candies)-1]
	for i := len(ratings) - 2; i >= 0; i-- {
		if ratings[i] > ratings[i+1] {
			if candies[i+1]+1 > candies[i] {
				candies[i] = candies[i+1] + 1
			}
		}

		sum += candies[i]
	}

	return sum
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
