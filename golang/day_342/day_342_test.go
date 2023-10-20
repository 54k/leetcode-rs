package day342

import (
	"unicode"
)

// https://leetcode.com/problems/basic-calculator-iii/description/
func calculate(s string) int {
	var solve func(string, *int) int
	solve = func(s string, i *int) int {
		stack := []int{}
		current := 0
		operator := '+'

		eval := func(op rune, x, y int) int {
			switch op {
			case '+':
				return x
			case '-':
				return -x
			case '*':
				return x * y
			default:
				return x / y
			}
		}

		for *i < len(s) {
			ch := rune(s[*i])
			if unicode.IsNumber(ch) {
				current = current*10 + int(ch-'0')
			} else if ch == '(' {
				*i = *i + 1
				current = solve(s, i)
			} else {
				if operator == '*' || operator == '/' {
					x := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					stack = append(stack, eval(operator, x, current))
				} else {
					stack = append(stack, eval(operator, current, 0))
				}

				if ch == ')' {
					break
				}

				operator = ch
				current = 0
			}

			*i = *i + 1
		}

		result := 0
		for _, n := range stack {
			result += n
		}
		return result
	}

	i := 0
	return solve(s+"@", &i)
}

// https://leetcode.com/problems/flatten-nested-list-iterator/description/

// This is the interface that allows for creating nested lists.
// You should not implement it, or speculate about its implementation
type NestedInteger struct {
}

// Return true if this NestedInteger holds a single integer, rather than a nested list.
func (this NestedInteger) IsInteger() bool { return false }

// Return the single integer that this NestedInteger holds, if it holds a single integer
// The result is undefined if this NestedInteger holds a nested list
// So before calling this method, you should have a check
func (this NestedInteger) GetInteger() int { return 0 }

// Set this NestedInteger to hold a single integer.
func (n *NestedInteger) SetInteger(value int) {}

// Set this NestedInteger to hold a nested list and adds a nested integer to it.
func (this *NestedInteger) Add(elem NestedInteger) {}

// Return the nested list that this NestedInteger holds, if it holds a nested list
// The list length is zero if this NestedInteger holds a single integer
// You can access NestedInteger's List element directly if you want to modify it
func (this NestedInteger) GetList() []*NestedInteger { return []*NestedInteger{} }

type NestedIterator struct {
	integers []int
	position int
}

func Constructor(nestedList []*NestedInteger) *NestedIterator {
	integers := []int{}

	var flatten func(list []*NestedInteger)
	flatten = func(list []*NestedInteger) {
		for _, li := range list {
			if li.IsInteger() {
				integers = append(integers, li.GetInteger())
			} else {
				flatten(li.GetList())
			}
		}
	}

	flatten(nestedList)
	return &NestedIterator{integers, 0}
}

func (this *NestedIterator) Next() int {
	if this.position >= len(this.integers) {
		return -1
	}
	res := this.integers[this.position]
	this.position++
	return res
}

func (this *NestedIterator) HasNext() bool {
	return this.position < len(this.integers)
}
