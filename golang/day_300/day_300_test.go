package day300

// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/
func minSwaps(s string) int {
	bal := 0
	for _, v := range s {
		if v == '[' {
			bal++
		} else if bal > 0 {
			bal--
		}
	}
	return (bal + 1) / 2
}

// https://leetcode.com/problems/max-chunks-to-make-sorted/description/
func maxChunksToSorted(arr []int) int {
	stack := []int{}
	for _, n := range arr {
		maxi := n
		for len(stack) > 0 && stack[len(stack)-1] > n {
			top := stack[len(stack)-1]
			if top > maxi {
				maxi = top
			}
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, maxi)
	}
	return len(stack)
}

// https://leetcode.com/problems/ternary-expression-parser/description/
func parseTernary(expression string) string {
	for len(expression) > 1 {
		qMarkIdx := len(expression) - 1
		for expression[qMarkIdx] != '?' {
			qMarkIdx--
		}
		var ch byte
		if expression[qMarkIdx-1] == 'T' {
			ch = expression[qMarkIdx+1]
		} else {
			ch = expression[qMarkIdx+3]
		}

		first := expression[:qMarkIdx-1]
		second := string(ch)
		third := expression[qMarkIdx+4:]
		expression = first + second + third
	}
	return expression
}
