package day339

import (
	"fmt"
	"math"
	"strconv"
	"testing"
	"unicode"
)

// https://leetcode.com/problems/basic-calculator/description/
func calculateForward(s string) int {
	stack := []int{}
	sign := 1
	operand := 0
	result := 0

	for _, ch := range s {
		if unicode.IsDigit(ch) {
			operand = operand*10 + int(ch-'0')
		} else if ch == '+' {
			result += operand * sign
			sign = 1
			operand = 0
		} else if ch == '-' {
			result += operand * sign
			sign = -1
			operand = 0
		} else if ch == '(' {
			stack = append(stack, result)
			stack = append(stack, sign)

			sign = 1
			operand = 0
			result = 0
		} else if ch == ')' {
			result += operand * sign

			result *= stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			result += stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			operand = 0
		}
	}

	return result + (sign * operand)
}

func calculateBackward(s string) int {
	stack := []string{}

	operand := 0
	n := 0

	evaluate := func() int {
		if len(stack) == 0 {
			stack = append(stack, "0")
		}
		if _, err := strconv.Atoi(stack[len(stack)-1]); err != nil {
			stack = append(stack, "0")
		}

		res, _ := strconv.Atoi(stack[len(stack)-1])
		stack = stack[:len(stack)-1]

		for len(stack) > 0 && stack[len(stack)-1] != ")" {
			sign := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			if sign == "+" {
				num, _ := strconv.Atoi(stack[len(stack)-1])
				stack = stack[:len(stack)-1]
				res += num
			} else {
				num, _ := strconv.Atoi(stack[len(stack)-1])
				stack = stack[:len(stack)-1]
				res -= num
			}
		}

		return res
	}

	for i := len(s) - 1; i >= 0; i-- {
		ch := rune(s[i])

		if unicode.IsDigit(ch) {
			operand = int(math.Pow10(n))*int(ch-'0') + operand
			n++
		} else if ch != ' ' {
			if n != 0 {
				stack = append(stack, strconv.Itoa(operand))
				n = 0
				operand = 0
			}

			if ch == '(' {
				res := evaluate()
				stack = stack[:len(stack)-1]
				stack = append(stack, strconv.Itoa(res))
			} else {
				stack = append(stack, string(ch))
			}
		}
	}

	if n != 0 {
		stack = append(stack, strconv.Itoa(operand))
	}

	return evaluate()
}

func TestCalcBackward(t *testing.T) {
	res := calculateBackward("(1+(4+5+2)-3)+(6+8)")
	fmt.Println(res)

	res = calculateBackward("2147483647")
	fmt.Println(res)
}
