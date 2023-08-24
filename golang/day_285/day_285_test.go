package day285

// https://leetcode.com/problems/text-justification/description/
func fullJustify(words []string, maxWidth int) []string {
	getWords := func(i int) []string {
		currLength := 0
		currentLine := []string{}
		for i < len(words) && currLength+len(words[i]) <= maxWidth {
			currLength += len(words[i]) + 1
			currentLine = append(currentLine, words[i])
			i++
		}
		return currentLine
	}

	createLine := func(line []string, wordsIdx int) {
		baseLength := -1
		for _, word := range line {
			baseLength += len(word) + 1
		}
		extraSpaces := maxWidth - baseLength

		if len(line) == 1 || wordsIdx == len(words) {
			for i := 0; i < len(line)-1; i++ {
				line[i] += " "
			}
			for i := 0; i < extraSpaces; i++ {
				line[len(line)-1] += " "
			}
			return
		}

		wordsCount := len(line) - 1
		spacePerWord := extraSpaces / wordsCount
		needsExtraSpaces := extraSpaces % wordsCount

		for i := 0; i < needsExtraSpaces; i++ {
			line[i] += " "
		}

		for i := 0; i < wordsCount; i++ {
			for j := 0; j < spacePerWord; j++ {
				line[i] += " "
			}
		}

		for i := 0; i < len(line)-1; i++ {
			line[i] += " "
		}
	}

	ans := []string{}
	i := 0
	for i < len(words) {
		currentLine := getWords(i)
		i += len(currentLine)
		createLine(currentLine, i)
		strLine := ""
		for _, word := range currentLine {
			strLine += word
		}
		ans = append(ans, strLine)
	}
	return ans
}
