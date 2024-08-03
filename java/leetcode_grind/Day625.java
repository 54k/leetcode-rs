package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class Day625 {
    // https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/description/?envType=daily-question&envId=2024-08-03
    static class Solution1 {
        public boolean canBeEqual(int[] target, int[] arr) {
            Arrays.sort(target);
            Arrays.sort(arr);
            for (int i = 0; i < arr.length; i++) {
                if (target[i] != arr[i]) {
                    return false;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/longest-repeating-substring/description/?envType=weekly-question&envId=2024-08-01
    static class Solution2 {
        public int longestRepeatingSubstring(String s) {
            Set<String> seenSubstrings = new HashSet<>();
            int maxLength = s.length() - 1;
            for (int start = 0; start <= s.length(); start++) {
                int end = start;
                if (end + maxLength > s.length()) {
                    if (--maxLength == 0)
                        break;
                    start = -1;
                    seenSubstrings.clear();
                    continue;
                }

                String currentSubstring = s.substring(end, end + maxLength);
                if (!seenSubstrings.add(currentSubstring)) {
                    return maxLength;
                }
            }
            return maxLength;
        }
    }

    static class Solution3 {
        public int longestRepeatingSubstring(String s) {
            int length = s.length(), maxLength = 0;
            Set<String> seenSubstrings = new HashSet<>();

            for (int start = 0; start < length; start++) {
                int end = start;
                if (end + maxLength >= length) {
                    return maxLength;
                }
                String currentSubstring = s.substring(end, end + maxLength + 1);
                if (!seenSubstrings.add(currentSubstring)) {
                    start = -1;
                    seenSubstrings.clear();
                    maxLength++;
                }
            }
            return maxLength;
        }
    }

    static class Solution4 {
        public int longestRepeatingSubstring(String s) {
            char[] characters = s.toCharArray();
            int start = 1, end = characters.length - 1;

            while (start <= end) {
                int mid = (start + end) / 2;
                if (hasRepeatingSubstring(characters, mid)) {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
            return end;
        }

        boolean hasRepeatingSubstring(char[] characters, int length) {
            Set<String> seenSubstrings = new HashSet<>();
            for (int i = 0; i <= characters.length - length; i++) {
                String substring = new String(characters, i, length);
                if (!seenSubstrings.add(substring)) {
                    return true;
                }
            }
            return false;
        }
    }

    static class Solution5 {
        public int longestRepeatingSubstring(String s) {
            int length = s.length();
            String[] suffixes = new String[length];
            for (int i = 0; i < length; i++) {
                suffixes[i] = s.substring(i);
            }
            msdRadixSort(suffixes);
            int maxLength = 0;
            for (int i = 1; i < length; i++) {
                int j = 0;
                while (j < Math.min(suffixes[i].length(), suffixes[i - 1].length()) &&
                        suffixes[i].charAt(j) == suffixes[i - 1].charAt(j)) {
                    j++;
                }
                maxLength = Math.max(maxLength, j);
            }
            return maxLength;
        }

        void msdRadixSort(String[] input) {
            sort(input, 0, input.length - 1, 0, new String[input.length]);
        }

        void sort(String[] input, int lo, int hi, int depth, String[] aux) {
            if (lo >= hi)
                return;
            int[] count = new int[128];
            for (int i = lo; i <= hi; i++) {
                count[charAt(input[i], depth) + 1]++;
            }
            for (int i = 1; i < 28; i++) {
                count[i] += count[i - 1];
            }
            for (int i = lo; i <= hi; i++) {
                aux[count[charAt(input[i], depth)]++] = input[i];
            }
            for (int i = lo; i <= hi; i++) {
                input[i] = aux[i - lo];
            }
            for (int i = 0; i < 27; i++) {
                sort(input, lo + count[i], lo + count[i + 1] - 1, depth + 1, aux);
            }
        }

        int charAt(String s, int index) {
            if (index >= s.length())
                return 0;
            return s.charAt(index) - 'a' + 1;
        }
    }
}
