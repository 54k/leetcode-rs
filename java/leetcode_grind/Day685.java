package leetcode_grind;

import java.util.*;

public class Day685 {
    // https://leetcode.com/problems/rank-transform-of-an-array/description/?envType=daily-question&envId=2024-10-02
    static class Solution1 {
        public int[] arrayRankTransform(int[] arr) {
            Map<Integer, Integer> numToRank = new HashMap<>();
            int[] sortedArr = Arrays.copyOf(arr, arr.length);
            Arrays.sort(sortedArr);
            int rank = 1;
            for (int i = 0; i < sortedArr.length; i++) {
                if (i > 0 && sortedArr[i] > sortedArr[i - 1]) {
                    rank++;
                }
                numToRank.put(sortedArr[i], rank);
            }
            for (int i = 0; i < arr.length; i++) {
                arr[i] = numToRank.get(arr[i]);
            }
            return arr;
        }
    }

    static class Solution2 {
        public int[] arrayRankTransform(int[] arr) {
            var numToRank = new HashMap<Integer, Integer>();
            var nums = new TreeSet<Integer>();
            for (int num : arr) {
                nums.add(num);
            }
            int rank = 1;
            for (int num : nums) {
                numToRank.put(num, rank);
                rank++;
            }
            for (int i = 0; i < arr.length; i++) {
                arr[i] = numToRank.get(arr[i]);
            }
            return arr;
        }
    }

    static class Solution3 {
        public int[] arrayRankTransform(int[] arr) {
            TreeMap<Integer, List<Integer>> numToIndices = new TreeMap<>();
            for (int i = 0; i < arr.length; i++) {
                if (!numToIndices.containsKey(arr[i])) {
                    numToIndices.put(arr[i], new ArrayList<>());
                }
                numToIndices.get(arr[i]).add(i);
            }
            int rank = 1;
            for (int num : numToIndices.keySet()) {
                for (int index : numToIndices.get(num)) {
                    arr[index] = rank;
                }
                rank++;
            }
            return arr;
        }
    }
}
