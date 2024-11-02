package leetcode_grind;

import java.util.*;

public class Day716 {
    // https://leetcode.com/problems/number-of-same-end-substrings/description/?envType=weekly-question&envId=2024-11-01
    static class Solution1 {
        public int[] sameEndSubstringCount(String s, int[][] queries) {
            Map<Character, List<Integer>> charPositionMap = new HashMap<>();
            for (int i = 0; i < s.length(); i++) {
                char c = s.charAt(i);
                charPositionMap.putIfAbsent(c, new ArrayList<>());
                charPositionMap.get(c).add(i);
            }

            int numQueries = queries.length;
            int[] result = new int[numQueries];

            for (int i = 0; i < numQueries; i++) {
                int leftIndex = queries[i][0];
                int rightIndex = queries[i][1];
                int countSameEndSubstrings = 0;

                for (char c : charPositionMap.keySet()) {
                    List<Integer> positions = charPositionMap.get(c);

                    int leftBound = findFirstGE(positions, leftIndex);
                    int rightBound = findFirstGT(positions, rightIndex);
                    int numOccurences = rightBound - leftBound;

                    countSameEndSubstrings += (numOccurences * (numOccurences + 1)) / 2;
                }

                result[i] = countSameEndSubstrings;
            }

            return result;
        }

        int findFirstGE(List<Integer> arr, int target) {
            int left = 0;
            int right = arr.size();
            while (left < right) {
                int mid = left + (right - left) / 2;
                if (arr.get(mid) < target) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }

        int findFirstGT(List<Integer> arr, int target) {
            int left = 0;
            int right = arr.size();
            while (left < right) {
                int mid = left + (right - left) / 2;
                if (arr.get(mid) <= target) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }
    }

    static class Solution2 {
        public int[] sameEndSubstringCount(String s, int[][] queries) {
            int n = s.length();
            int[][] charFreqPrefixSum = new int[26][n];
            for (int i = 0; i < n; i++) {
                charFreqPrefixSum[s.charAt(i) - 'a'][i]++;
            }

            for (int i = 0; i < 26; i++) {
                for (int j = 1; j < n; j++) {
                    charFreqPrefixSum[i][j] += charFreqPrefixSum[i][j - 1];
                }
            }

            int[] results = new int[queries.length];

            for (int i = 0; i < queries.length; i++) {
                int leftIndex = queries[i][0];
                int rightIndex = queries[i][1];
                int countSameEndSubstrings = 0;

                for (int charIndex = 0; charIndex < 26; charIndex++) {
                    int leftFreq = (leftIndex == 0) ? 0 : charFreqPrefixSum[charIndex][leftIndex - 1];
                    int rightFreq = charFreqPrefixSum[charIndex][rightIndex];
                    int frequencyInRange = rightFreq - leftFreq;

                    countSameEndSubstrings += (frequencyInRange * (frequencyInRange + 1)) / 2;
                }

                results[i] = countSameEndSubstrings;
            }

            return results;
        }
    }
}
