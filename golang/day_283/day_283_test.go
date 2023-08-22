package day283

// https://leetcode.com/problems/excel-sheet-column-title/description/
func convertToTitle(columnNumber int) string {
	ans := ""
	for columnNumber > 0 {
		columnNumber--
		ans += string(columnNumber%26 + 'A')
		columnNumber /= 26
	}
	ansb := []byte(ans)
	for i := 0; i < len(ans)/2; i++ {
		ansb[i], ansb[len(ans)-i-1] = ansb[len(ans)-i-1], ansb[i]
	}
	return string(ansb)
}
