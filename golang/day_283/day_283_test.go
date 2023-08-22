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

// https://leetcode.com/problems/optimal-partition-of-string/description
func partitionString(s string) int {
	lastSeen := make([]int, 26)
	for i := 0; i < 26; i++ {
		lastSeen[i] = -1
	}
	count := 1
	substringStart := 0
	for i, ch := range s {
		if lastSeen[ch-'a'] >= substringStart {
			count++
			substringStart = i
		}
		lastSeen[ch-'a'] = i
	}
	return count
}

// https://leetcode.com/problems/count-complete-subarrays-in-an-array/description/
func countCompleteSubarrays(nums []int) int {
	uniq := map[int]bool{}
	for _, n := range nums {
		uniq[n] = true
	}
	k := len(uniq)

	ans := 0
	countFreq := map[int]int{}
	start := 0

	for end := 0; end < len(nums); end++ {
		curr := nums[end]
		if countFreq[curr] == 0 {
			k--
		}
		countFreq[curr]++
		for k == 0 {
			prev := nums[start]
			countFreq[prev]--
			if countFreq[prev] == 0 {
				k++
			}
			start++
		}
		ans += start
	}
	return ans
}
