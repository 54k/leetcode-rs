package main

// https://leetcode.com/problems/gray-code/description/
func grayCodeRec(n int) []int {
	result := []int{}
	var rec func(int)
	rec = func(n int) {
		if n == 0 {
			result = append(result, 0)
			return
		}

		rec(n - 1)
		curSeqLen := len(result)
		mask := 1 << (n - 1)
		for i := curSeqLen - 1; i >= 0; i-- {
			result = append(result, result[i]|mask)
		}
	}
	rec(n)
	return result
}
