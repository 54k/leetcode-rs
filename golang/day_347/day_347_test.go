package day347

// https://leetcode.com/problems/maximum-subarray/description/
func maxSubArray(nums []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	var findBest func(int, int) int
	findBest = func(left, right int) int {
		if left > right {
			return -(1 << 30)
		}

		mid := (right-left)/2 + left
		curr := 0
		leftSum := 0
		for i := mid - 1; i >= left; i-- {
			curr += nums[i]
			leftSum = max(leftSum, curr)
		}
		curr = 0
		rightSum := 0
		for i := mid + 1; i <= right; i++ {
			curr += nums[i]
			rightSum = max(rightSum, curr)
		}

		bothSum := nums[mid] + leftSum + rightSum

		return max(bothSum, max(findBest(left, mid-1), findBest(mid+1, right)))
	}
	return findBest(0, len(nums)-1)
}

// https://leetcode.com/problems/palindrome-partitioning/description/
func partitionBacktrack(s string) [][]string {
	isPalindrome := func(low, hi int) bool {
		for low < hi {
			if s[low] != s[hi] {
				return false
			}
			low++
			hi--
		}
		return true
	}

	result := [][]string{}
	current := []string{}

	var backtrack func(int)
	backtrack = func(start int) {
		if start >= len(s) {
			r := make([]string, len(current))
			copy(r, current)
			result = append(result, r)
		}

		for end := start; end < len(s); end++ {
			if isPalindrome(start, end) {
				current = append(current, s[start:end+1])
				backtrack(end + 1)
				current = current[:len(current)-1]
			}
		}
	}

	backtrack(0)
	return result
}

// https://leetcode.com/problems/longest-palindromic-substring/
func longestPalindromeBruteForce(s string) string {
	check := func(start, end int) bool {
		for start < end {
			if s[start] != s[end] {
				return false
			}
			start++
			end--
		}
		return true
	}

	ans := 0

	lo := -1
	hi := -1

	for start := 0; start < len(s); start++ {
		for end := start; end < len(s); end++ {
			if check(start, end) {
				if end-start+1 > ans {
					lo = start
					hi = end
					ans = end - start + 1
				}
			}
		}
	}

	return s[lo : hi+1]
}

func longestPalindromeDP(s string) string {
	dp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		dp[i] = make([]bool, len(s))
	}

	maxLen := 0
	lo := 0
	hi := 0

	for i := 0; i < len(s); i++ {
		dp[i][i] = true
		if i < len(s)-1 {
			if s[i] == s[i+1] {
				dp[i][i+1] = true
				maxLen = 2
				lo = i
				hi = i + 1
			}
		}
	}

	for diff := 2; diff < len(s); diff++ {
		for start := 0; start < len(s)-diff; start++ {
			end := start + diff
			if dp[start+1][end-1] && s[start] == s[end] {
				dp[start][end] = true
				if maxLen < end-start+1 {
					maxLen = end - start + 1
					lo = start
					hi = end
				}
			}
		}
	}

	return s[lo : hi+1]
}

func longestPalindromeExpandAroundCenter(s string) string {
	expand := func(i, j int) int {
		left := i
		right := j

		for left >= 0 && right < len(s) && s[left] == s[right] {
			left--
			right++
		}

		return right - left - 1
	}
	ans := []int{0, 0}

	for i := 0; i < len(s); i++ {
		oddLength := expand(i, i)
		if oddLength > ans[1]-ans[0]+1 {
			dist := oddLength / 2
			ans[0] = i - dist
			ans[1] = i + dist
		}

		evenLength := expand(i, i+1)
		if evenLength > ans[1]-ans[0]+1 {
			dist := (evenLength / 2) - 1
			ans[0] = i - dist
			ans[1] = i + 1 + dist
		}
	}

	return s[ans[0] : ans[1]+1]
}

// https://leetcode.com/problems/palindrome-permutation/description/
func canPermutePalindrome(s string) bool {
	freq := map[rune]int{}
	for _, ch := range s {
		freq[ch]++
	}

	count := 0
	for _, v := range freq {
		count += v % 2
	}
	return count <= 1
}

// https://leetcode.com/problems/longest-palindrome/description/
func longestPalindrome(s string) int {
	freq := map[rune]int{}
	for _, ch := range s {
		freq[ch]++
	}
	hasOdd := false
	count := 0
	for _, v := range freq {
		if v%2 == 0 {
			count += v
		} else if v > 1 {
			hasOdd = true
			count += v - 1
		} else {
			hasOdd = true
		}
	}
	if hasOdd {
		return count + 1
	}
	return count
}
