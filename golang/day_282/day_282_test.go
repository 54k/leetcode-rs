package day282

// https://leetcode.com/problems/repeated-substring-pattern/description/
func repeatedSubstringPattern(s string) bool {
	if len(s) == 1 {
		return false
	}
	isOk := func(pat string) bool {
		s2 := ""
		for i := 0; i < len(s)/len(pat); i++ {
			s2 += pat
		}
		return s == s2
	}

	for i := 1; i <= len(s)/2; i++ {
		if len(s)%i == 0 && isOk(s[:i]) {
			return true
		}
	}
	return false
}
