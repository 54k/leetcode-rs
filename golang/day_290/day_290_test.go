package day290

// https://leetcode.com/problems/minimum-penalty-for-a-shop/description/
func bestClosingTime(customers string) int {
	curPenalty := 0
	minPenalty := 0
	closingTime := 0

	for i, ch := range customers {
		if ch == 'Y' {
			curPenalty -= 1
		} else {
			curPenalty += 1
		}

		if curPenalty < minPenalty {
			minPenalty = curPenalty
			closingTime = i + 1
		}
	}
	return closingTime
}
