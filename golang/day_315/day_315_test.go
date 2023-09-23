package day315

// https://leetcode.com/problems/shortest-way-to-form-string/description/
func shortestWay(source string, target string) int {
	nextOccurences := make([][]int, len(source))
	for i := 0; i < len(source); i++ {
		nextOccurences[i] = make([]int, 26)
	}

	for j := 0; j < 26; j++ {
		nextOccurences[len(source)-1][j] = -1
	}
	nextOccurences[len(source)-1][source[len(source)-1]-'a'] = len(source) - 1

	for i := len(source) - 2; i >= 0; i-- {
		for j := 0; j < 26; j++ {
			nextOccurences[i][j] = nextOccurences[i+1][j]
		}
		nextOccurences[i][source[i]-'a'] = i
	}

	sourceIterator := 0
	count := 1
	for _, c := range target {
		if nextOccurences[0][c-'a'] == -1 {
			return -1
		}

		if sourceIterator == len(source) || nextOccurences[sourceIterator][c-'a'] == -1 {
			count++
			sourceIterator = 0
		}

		sourceIterator = nextOccurences[sourceIterator][c-'a'] + 1
	}

	return count
}
