package leetcode_grind;

import java.util.Stack;

public class Day604 {
    // https://leetcode.com/problems/maximum-score-from-removing-substrings/description/?envType=daily-question&envId=2024-07-12
    static class Solution1 {
        public int maximumGain(String s, int x, int y) {
            int totalScore = 0;
            String highPriorityPair = x > y ? "ab" : "ba";
            String lowPriorityPair = highPriorityPair.equals("ab") ? "ba" : "ab";

            String stringAfterFirstPass = removeSubstring(s, highPriorityPair);
            int removedPairsCount = (s.length() - stringAfterFirstPass.length()) / 2;

            totalScore += removedPairsCount * Math.max(x, y);

            String stringAfterSecondPass = removeSubstring(stringAfterFirstPass, lowPriorityPair);
            removedPairsCount = (stringAfterFirstPass.length() - stringAfterSecondPass.length()) / 2;

            totalScore += removedPairsCount * Math.min(x, y);
            return totalScore;
        }

        String removeSubstring(String input, String targetPair) {
            Stack<Character> charStack = new Stack<>();
            for (int i = 0; i < input.length(); i++) {
                char currentChar = input.charAt(i);

                if (currentChar == targetPair.charAt(1) && !charStack.isEmpty()
                        && charStack.peek() == targetPair.charAt(0)) {
                    charStack.pop();
                } else {
                    charStack.push(currentChar);
                }
            }

            StringBuilder remainingChars = new StringBuilder();
            while (!charStack.isEmpty()) {
                remainingChars.append(charStack.pop());
            }
            return remainingChars.reverse().toString();
        }
    }

    static class Solution2 {
        public int maximumGain(String s, int x, int y) {
            StringBuilder text = new StringBuilder(s);
            int totalPoints = 0;
            if (x > y) {
                totalPoints += removeSubstring(text, "ab", x);
                totalPoints += removeSubstring(text, "ba", y);
            } else {
                totalPoints += removeSubstring(text, "ba", y);
                totalPoints += removeSubstring(text, "ab", x);
            }
            return totalPoints;
        }

        int removeSubstring(StringBuilder inputString, String targetSubstring, int pointsPerRemoval) {
            int totalPoints = 0;
            int writeIndex = 0;
            for (int readIndex = 0; readIndex < inputString.length(); readIndex++) {
                inputString.setCharAt(writeIndex++, inputString.charAt(readIndex));

                if (writeIndex > 1 && inputString.charAt(writeIndex - 2) == targetSubstring.charAt(0) &&
                        inputString.charAt(writeIndex - 1) == targetSubstring.charAt(1)) {
                    writeIndex -= 2;
                    totalPoints += pointsPerRemoval;
                }
            }
            inputString.setLength(writeIndex);
            return totalPoints;
        }
    }
}
