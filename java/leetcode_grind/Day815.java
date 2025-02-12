package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;
import java.util.Stack;

public class Day815 {
    // https://leetcode.com/problems/remove-all-occurrences-of-a-substring/description/?envType=daily-question&envId=2025-02-11
    static class Solution1 {
        public String removeOccurrences(String s, String part) {
            while (s.contains(part)) {
                int partStartIndex = s.indexOf(part);
                s = s.substring(0, partStartIndex) + s.substring(partStartIndex + part.length());
            }
            return s;
        }
    }

    static class Solution2 {
        public String removeOccurrences(String s, String part) {
            Stack<Character> stk = new Stack<>();
            int strLength = s.length();
            int partLength = part.length();

            for (int index = 0; index < strLength; index++) {
                stk.push(s.charAt(index));

                if (stk.size() >= partLength && checkMatch(stk, part, partLength)) {
                    for (int j = 0; j < partLength; j++) {
                        stk.pop();
                    }
                }
            }

            StringBuilder result = new StringBuilder();
            while (!stk.isEmpty()) {
                result.append(stk.pop());
            }
            return result.reverse().toString();
        }

        boolean checkMatch(Stack<Character> stk, String part, int partLength) {
            Stack<Character> temp = new Stack<>();
            temp.addAll(stk);

            for (int partIndex = partLength - 1; partIndex >= 0; partIndex--) {
                if (temp.isEmpty() || temp.peek() != part.charAt(partIndex)) {
                    return false;
                }
                temp.pop();
            }
            return true;
        }
    }

    static class Solution3 {
        public String removeOccurrences(String s, String part) {
            int[] kmpLPS = computeLongestPrefixSuffix(part);
            Stack<Character> charStack = new Stack<>();
            int[] patternIndexes = new int[s.length() + 1];

            for (int strIndex = 0, patternIndex = 0; strIndex < s.length(); strIndex++) {
                char currentChar = s.charAt(strIndex);
                charStack.push(currentChar);

                if (currentChar == part.charAt(patternIndex)) {
                    patternIndexes[charStack.size()] = ++patternIndex;

                    if (patternIndex == part.length()) {
                        int remainingLength = part.length();
                        while (remainingLength != 0) {
                            charStack.pop();
                            remainingLength--;
                        }

                        patternIndex = charStack.isEmpty() ? 0 : patternIndexes[charStack.size()];
                    }
                } else {
                    if (patternIndex != 0) {
                        strIndex--;
                        patternIndex = kmpLPS[patternIndex - 1];
                        charStack.pop();
                    } else {
                        patternIndexes[charStack.size()] = 0;
                    }
                }
            }

            StringBuilder result = new StringBuilder();
            while (!charStack.isEmpty()) {
                result.append(charStack.pop());
            }
            return result.reverse().toString();
        }

        int[] computeLongestPrefixSuffix(String pattern) {
            int[] lps = new int[pattern.length()];

            for (int current = 1, prefixLength = 0; current < pattern.length();) {
                if (pattern.charAt(current) == pattern.charAt(prefixLength)) {
                    lps[current] = ++prefixLength;
                    current++;
                } else if (prefixLength != 0) {
                    prefixLength = lps[prefixLength - 1];
                } else {
                    lps[current] = 0;
                    current++;
                }
            }
            return lps;
        }
    }

    // https://leetcode.com/problems/count-beautiful-substrings-ii/description/
    static class Solution {
        public long beautifulSubstrings(String s, int k) {
            int n = s.length(), v = 0, l;
            for (l = 1; l * l % (k * 4) > 0; l++)
                ;
            Set<Character> vowels = new HashSet<>(Arrays.asList('a', 'e', 'i', 'o', 'u'));
            HashMap<Integer, Integer>[] seen = new HashMap[l];
            for (int i = 0; i < l; i++) {
                seen[i] = new HashMap<>();
            }
            seen[l - 1].put(0, 1);
            long res = 0;
            for (int i = 0; i < n; i++) {
                v += vowels.contains(s.charAt(i)) ? 1 : -1;
                int c = seen[i % l].getOrDefault(v, 0);
                res += c;
                seen[i % l].put(v, c + 1);
            }
            return res;
        }
    }
}
