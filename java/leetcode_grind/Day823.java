package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Stack;

public class Day823 {
    // https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/description/?envType=daily-question&envId=2025-02-19
    static class Solution1 {
        List<String> happyStrings = new ArrayList<>();

        public String getHappyString(int n, int k) {
            String currentString = "";
            generateHappyString(n, currentString);
            if (happyStrings.size() < k)
                return "";
            Collections.sort(happyStrings);
            return happyStrings.get(k - 1);
        }

        void generateHappyString(int n, String currentString) {
            if (currentString.length() == n) {
                happyStrings.add(currentString);
                return;
            }

            for (char currentChar = 'a'; currentChar <= 'c'; currentChar++) {
                if (currentString.length() > 0 && currentString.charAt(currentString.length() - 1) == currentChar)
                    continue;
                generateHappyString(n, currentString + currentChar);
            }
        }
    }

    static class Solution2 {
        void generateHappyStrings(int n, int k, StringBuilder currentString, int[] indexInSortedList, String[] result) {
            if (currentString.length() == n) {
                indexInSortedList[0]++;
                if (indexInSortedList[0] == k)
                    result[0] = currentString.toString();
                return;
            }

            for (char currentChar = 'a'; currentChar <= 'c'; currentChar++) {
                if (currentString.length() > 0 && currentString.charAt(currentString.length() - 1) == currentChar)
                    continue;

                currentString.append(currentChar);
                generateHappyStrings(n, k, currentString, indexInSortedList, result);

                if (result[0] != null)
                    return;

                currentString.deleteCharAt(currentString.length() - 1);
            }
        }

        public String getHappyString(int n, int k) {
            StringBuilder currentString = new StringBuilder();
            String[] result = new String[1];
            int[] indexInSortedList = new int[1];
            generateHappyStrings(n, k, currentString, indexInSortedList, result);
            return result[0] == null ? "" : result[0];
        }
    }

    static class Solution3 {
        public String getHappyString(int n, int k) {
            Stack<String> stringsStack = new Stack<>();
            int indexInSortedList = 0;
            stringsStack.push("");

            while (stringsStack.size() > 0) {
                String currentString = stringsStack.pop();
                if (currentString.length() == n) {
                    indexInSortedList++;
                    if (indexInSortedList == k) {
                        return currentString;
                    }
                    continue;
                }

                for (char currentChar = 'c'; currentChar >= 'a'; currentChar--) {
                    if (currentString.length() == 0
                            || currentString.charAt(currentString.length() - 1) != currentChar) {
                        stringsStack.push(currentString + currentChar);
                    }
                }
            }
            return "";
        }
    }
}
