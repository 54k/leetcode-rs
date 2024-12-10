package leetcode_grind;

import java.util.HashMap;

public class Day762 {
    // https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/description/
    static class Solution1 {
        public int maximumLength(String s) {
            HashMap<String, Integer> count = new HashMap<>();
            for (int start = 0; start < s.length(); start++) {
                StringBuilder currString = new StringBuilder();

                for (int end = start; end < s.length(); end++) {
                    if (currString.length() == 0 || currString.charAt(currString.length() - 1) == s.charAt(end)) {
                        currString.append(s.charAt(end));
                        count.put(currString.toString(), count.getOrDefault(currString.toString(), 0) + 1);
                    } else {
                        break;
                    }
                }
            }

            int ans = 0;
            for (String str : count.keySet()) {
                if (count.get(str) >= 3 && str.length() > ans)
                    ans = str.length();
            }
            if (ans == 0)
                return -1;
            return ans;
        }
    }

    // https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-ii/description/
    static class Solution2 {
        public int maximumLength(String s) {
            int[][] frequency = new int[26][s.length() + 1];
            char previousCharacter = s.charAt(0);
            int substringLength = 1;

            frequency[s.charAt(0) - 'a'][1] = 1;
            int ans = -1;

            for (int charIdx = 1; charIdx < s.length(); charIdx++) {
                char currentCharacter = s.charAt(charIdx);

                if (currentCharacter == previousCharacter) {
                    substringLength++;
                    frequency[currentCharacter - 'a'][substringLength] += 1;
                } else {
                    previousCharacter = currentCharacter;
                    substringLength = 1;
                    frequency[currentCharacter - 'a'][1] += 1;
                }
            }

            for (int charIdx = 0; charIdx < 26; charIdx++) {
                for (int len = s.length() - 1; len >= 1; len--) {
                    frequency[charIdx][len] += frequency[charIdx][len + 1];
                    if (frequency[charIdx][len] >= 3) {
                        ans = Math.max(ans, len);
                        break;
                    }
                }
            }

            return ans;
        }
    }

    static class Solution3 {
        int min(int a, int b, int c) {
            return a < Math.min(b, c) ? a : Math.min(b, c);
        }

        public int maximumLength(String s) {
            int substringLength = 0, ans = -1;
            char previousCharacter = '\0';
            int[][] substringLengths = new int[26][3];

            for (int charIdx = 0; charIdx < 26; charIdx++) {
                for (int lenIdx = 0; lenIdx < 3; lenIdx++) {
                    substringLengths[charIdx][lenIdx] = -1;
                }
            }

            for (char character : s.toCharArray()) {
                if (character == previousCharacter) {
                    substringLength++;
                } else {
                    substringLength = 1;
                    previousCharacter = character;
                }

                int index = character - 'a';
                int minLength = min(substringLengths[index][0], substringLengths[index][1], substringLengths[index][2]);

                if (substringLength > minLength) {
                    if (substringLengths[index][0] == minLength) {
                        substringLengths[index][0] = substringLength;
                    } else if (substringLengths[index][1] == minLength) {
                        substringLengths[index][1] = substringLength;
                    } else {
                        substringLengths[index][2] = substringLength;
                    }
                }
            }

            for (int charIdx = 0; charIdx < 26; charIdx++) {
                ans = Math.max(ans,
                        min(substringLengths[charIdx][0], substringLengths[charIdx][1], substringLengths[charIdx][2]));
            }

            return ans;
        }
    }

}
