package day264

// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
func letterCombinations(digits string) []string {
	mapping := map[rune][]rune{}
	mapping['2'] = []rune("abc")
	mapping['3'] = []rune("def")
	mapping['4'] = []rune("ghi")
	mapping['5'] = []rune("jkl")
	mapping['6'] = []rune("mno")
	mapping['7'] = []rune("pqrs")
	mapping['8'] = []rune("tuv")
	mapping['9'] = []rune("wxyz")

	ans := []string{}
	cur := []rune{}

	var rec func(int)
	rec = func(i int) {
		if i == len(digits) {
			dst := ""
			if string(cur) == dst {
				return
			}
			for _, ch := range cur {
				dst += string(ch)
			}
			ans = append(ans, dst)
			return
		}

		for _, ch := range mapping[rune(digits[i])] {
			cur = append(cur, ch)
			rec(i + 1)
			cur = cur[:len(cur)-1]
		}

	}

	rec(0)

	return ans
}
