package leetcode_grind;

import java.util.*;

public class Day717 {
    // https://leetcode.com/problems/rotate-string/description/?envType=daily-question&envId=2024-11-03
    static class Solution1 {
        public boolean rotateString(String s, String goal) {
            if (s.length() != goal.length())
                return false;
            int length = s.length();
            char[] sChars = s.toCharArray();

            for (int rotationCount = 0; rotationCount < length; rotationCount++) {
                sChars = rotateOnce(sChars);
                if (new String(sChars).equals(goal))
                    return true;
            }
            return false;
        }

        char[] rotateOnce(char[] arr) {
            char firstChar = arr[0];
            System.arraycopy(arr, 1, arr, 0, arr.length - 1);
            arr[arr.length - 1] = firstChar;
            return arr;
        }
    }

    static class Solution2 {
        public boolean rotateString(String s, String goal) {
            if (s.length() != goal.length())
                return false;
            String doubledString = s + s;
            return kmpSearch(doubledString, goal);
        }

        boolean kmpSearch(String text, String pattern) {
            // int[] lps = computeLPS(pattern + '$' + text);
            // for (var n : lps) {
            // if (n == pattern.length()) return true;
            // }
            int[] lps = computeLPS(pattern);
            int textIndex = 0, patternIndex = 0;
            int textLength = text.length(), patternLength = pattern.length();

            while (textIndex < textLength) {
                if (text.charAt(textIndex) == pattern.charAt(patternIndex)) {
                    textIndex++;
                    patternIndex++;
                    if (patternIndex == patternLength)
                        return true;
                } else if (patternIndex > 0) {
                    patternIndex = lps[patternIndex - 1];
                } else {
                    textIndex++;
                }
            }
            return false;
        }

        int[] computeLPS(String pattern) {
            int patternLength = pattern.length();
            int[] lps = new int[patternLength];
            int length = 0, index = 1;

            while (index < patternLength) {
                if (pattern.charAt(index) == pattern.charAt(length)) {
                    length++;
                    lps[index++] = length;
                } else if (length > 0) {
                    length = lps[length - 1];
                } else {
                    lps[index++] = 0;
                }
            }

            return lps;
        }
    }

    // https://leetcode.com/problems/output-contest-matches/description/?envType=problem-list-v2&envId=recursion
    static class Solution3 {
        public String findContestMatch(int n) {
            String[] team = new String[n];
            for (int i = 1; i <= n; i++) {
                team[i - 1] = "" + i;
            }

            for (; n > 1; n /= 2) {
                for (int i = 0; i < n / 2; ++i) {
                    team[i] = "(" + team[i] + "," + team[n - 1 - i] + ")";
                }
            }

            return team[0];
        }
    }

    static class SQL {
        Map<String, Map<Integer, List<String>>> records = new HashMap<>();
        Map<String, Integer> ids = new HashMap<>();

        public SQL(List<String> names, List<Integer> columns) {
            for (int i = 0; i < names.size(); i++) {
                records.put(names.get(i), new HashMap<Integer, List<String>>());
                ids.put(names.get(i), 0);
            }
        }

        public void insertRow(String name, List<String> row) {
            var rec = records.get(name);
            var id = ids.get(name) + 1;
            ids.put(name, id);
            rec.put(id, row);
        }

        public void deleteRow(String name, int rowId) {
            var rec = records.get(name);
            rec.remove(rowId);
        }

        public String selectCell(String name, int rowId, int columnId) {
            var rec = records.get(name);
            return rec.getOrDefault(rowId, new ArrayList<>()).stream().skip(columnId - 1).findFirst().orElse("");
        }
    }

    /**
     * Your SQL object will be instantiated and called as such:
     * SQL obj = new SQL(names, columns);
     * obj.insertRow(name,row);
     * obj.deleteRow(name,rowId);
     * String param_3 = obj.selectCell(name,rowId,columnId);
     */
}
