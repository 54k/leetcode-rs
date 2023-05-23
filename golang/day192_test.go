package main

import (
	"github.com/emirpasic/gods/trees/binaryheap"
	h "github.com/emirpasic/gods/trees/binaryheap"
)

type heap struct {
	h.Heap
}

type KthLargest struct {
	heap heap
	k    int
}

func (heap *heap) kth(val, k int) int {
	heap.Push(val)
	if heap.Size() > k {
		heap.Pop()
	}
	top, _ := heap.Peek()
	return top.(int)
}

func BinHeapConstructor(k int, nums []int) KthLargest {
	heap := heap{*binaryheap.NewWithIntComparator()}
	for _, num := range nums {
		heap.kth(num, k)
	}
	return KthLargest{
		heap,
		k,
	}
}

func (this *KthLargest) Add(val int) int {
	return this.heap.kth(val, this.k)
}
