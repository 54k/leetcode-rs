// https://leetcode.com/problems/insert-delete-getrandom-o1/description/
package main

import "math/rand"

type RandomizedSet struct {
	m    map[int]int
	vals []int
	r    *rand.Rand
}

func NewRS() RandomizedSet {
	return RandomizedSet{
		m:    map[int]int{},
		vals: []int{},
		r:    rand.New(rand.NewSource(32198)),
	}
}

func (this *RandomizedSet) Insert(val int) bool {
	if _, ok := this.m[val]; ok {
		return false
	}
	this.vals = append(this.vals, val)
	this.m[val] = len(this.vals) - 1
	return true
}

func (this *RandomizedSet) Remove(val int) bool {
	if _, ok := this.m[val]; !ok {
		return false
	}
	lastElem := this.vals[len(this.vals)-1]
	i := this.m[val]
	this.vals[i] = this.vals[len(this.vals)-1]
	this.m[lastElem] = i
	delete(this.m, val)
	this.vals = this.vals[:len(this.vals)-1]
	return true
}

func (this *RandomizedSet) GetRandom() int {
	rIdx := this.r.Int() % len(this.vals)
	return this.vals[rIdx]
}

// https://leetcode.com/problems/top-k-frequent-elements/description/
func topKFrequent(nums []int, k int) []int {
	m1 := map[int]int{}
	for _, n := range nums {
		m1[n]++
	}
	m2 := map[int][]int{}
	for n, cnt := range m1 {
		m2[cnt] = append(m2[cnt], n)
	}
	res := []int{}
	for n := len(nums); n > 0; n-- {
		if v, ok := m2[n]; ok {
			for _, n := range v {
				if k == 0 {
					return res
				}
				res = append(res, n)
				k--
			}
		}
	}
	return res
}
