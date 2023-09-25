package day317

// https://leetcode.com/problems/find-the-difference/description
func findTheDifference(s string, t string) byte {
	tMap := map[rune]int{}
	for _, ch := range t {
		tMap[ch]++
	}
	for _, ch := range s {
		tMap[ch]--
		if tMap[ch] == 0 {
			delete(tMap, ch)
		}
	}
	for k, _ := range tMap {
		return byte(k)
	}
	panic("oops")
}
