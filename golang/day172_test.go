package main

// https://leetcode.com/problems/peeking-iterator/
type Iterator struct {
}

func (this *Iterator) hasNext() bool {
	// Returns true if the iteration has more elements.
	return true
}
func (this *Iterator) next() int {
	// Returns the next element in the iteration.
	return 0
}

type PeekingIterator struct {
	stack []int
	it    *Iterator
}

func constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{
		stack: make([]int, 0),
		it:    iter,
	}
}

func (this *PeekingIterator) hasNext() bool {
	return len(this.stack) > 0 || this.it.hasNext()
}

func (this *PeekingIterator) next() int {
	if len(this.stack) > 0 {
		ret := this.stack[len(this.stack)-1]
		this.stack = this.stack[:len(this.stack)-1]
		return ret
	} else {
		return this.it.next()
	}
}

func (this *PeekingIterator) peek() int {
	if len(this.stack) > 0 {
		return this.stack[len(this.stack)-1]
	} else {
		peek := this.it.next()
		this.stack = append(this.stack, peek)
		return peek
	}
}
