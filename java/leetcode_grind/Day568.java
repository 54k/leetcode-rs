package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day568 {
    // https://leetcode.com/problems/find-common-characters/description
    static class Solution1 {
        public List<String> commonChars(String[] words) {
            int wordsSize = words.length;
            int[] commonCharacterCounts = new int[26];
            int[] currentCharacterCounts = new int[26];
            List<String> result = new ArrayList<>();

            for (char ch : words[0].toCharArray()) {
                commonCharacterCounts[ch - 'a']++;
            }

            for (int i = 1; i < wordsSize; i++) {
                Arrays.fill(currentCharacterCounts, 0);

                for (char ch : words[i].toCharArray()) {
                    currentCharacterCounts[ch - 'a']++;
                }

                for (int letter = 0; letter < 26; letter++) {
                    commonCharacterCounts[letter] = Math.min(
                            commonCharacterCounts[letter],
                            currentCharacterCounts[letter]);
                }
            }

            for (int letter = 0; letter < 26; letter++) {
                for (int commonCount = 0; commonCount < commonCharacterCounts[letter]; commonCount++) {
                    result.add(String.valueOf((char) (letter + 'a')));
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/longest-common-subsequence-between-sorted-arrays/description
    static class Solution2 {
        public List<Integer> longestCommonSubsequence(int[][] arrays) {
            Map<Integer, Integer> frequencies = new HashMap<>();
            List<Integer> longestCommonSubseq = new ArrayList<>();

            for (int[] array : arrays) {
                for (int num : array) {
                    frequencies.put(num, frequencies.getOrDefault(num, 0) + 1);
                    if (frequencies.get(num) == arrays.length) {
                        longestCommonSubseq.add(num);
                    }
                }
            }
            return longestCommonSubseq;
        }
    }

    static class Solution3 {
        public List<Integer> longestCommonSubsequence(int[][] arrays) {
            List<Integer> longestCommonSubseq = new ArrayList<>();
            for (int num : arrays[0]) {
                longestCommonSubseq.add(num);
            }

            for (int i = 1; i < arrays.length; i++) {
                if (longestCommonSubseq.isEmpty()) {
                    return longestCommonSubseq;
                }
                longestCommonSubseq = longestSeq(longestCommonSubseq, arrays[i]);
            }
            return longestCommonSubseq;
        }

        List<Integer> longestSeq(List<Integer> nums1, int[] nums2) {
            List<Integer> longestCommonSeq = new ArrayList<>();
            int first = 0;
            int second = 0;
            while (first < nums1.size() && second < nums2.length) {
                if (nums1.get(first) < nums2[second]) {
                    first++;
                } else if (nums1.get(first) > nums2[second]) {
                    second++;
                } else {
                    longestCommonSeq.add(nums1.get(first));
                    first++;
                    second++;
                }
            }
            return longestCommonSeq;
        }
    }

    static class Solution3 {
        public List<Integer> longestCommonSubsequence(int[][] arrays) {
            int[] shortestArray = arrays[0];
            for (int[] array : arrays) {
                if (array.length < shortestArray.length) {
                    shortestArray = array;
                }
            }

            List<Integer> longestCommonSubseq = new ArrayList<>();
            for (int num : shortestArray) {
                longestCommonSubseq.add(num);
            }

            for (int[] array : arrays) {
                if (longestCommonSubseq.isEmpty()) {
                    return longestCommonSubseq;
                }

                List<Integer> uncommon = new ArrayList<>();
                for (Integer num : longestCommonSubseq) {
                    if (!binarySearch(num, array))
                        uncommon.add(num);
                }
                for (Integer num : uncommon) {
                    longestCommonSubseq.remove(num);
                }
            }
            return longestCommonSubseq;
        }

        boolean binarySearch(int target, int[] nums) {
            int left = 0;
            int right = nums.length - 1;
            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (nums[mid] > target) {
                    right = mid - 1;
                } else if (nums[mid] < target) {
                    left = mid + 1;
                } else {
                    return true;
                }
            }
            return false;
        }
    }
}
