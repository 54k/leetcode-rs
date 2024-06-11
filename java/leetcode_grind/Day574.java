package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day574 {
    // https://leetcode.com/problems/relative-sort-array/description
    static class Solution1 {
        public int[] relativeSortArray(int[] arr1, int[] arr2) {
            List<Integer> result = new ArrayList<>();
            for (int i = 0; i < arr2.length; i++) {
                for (int j = 0; j < arr1.length; j++) {
                    if (arr1[j] == arr2[i]) {
                        result.add(arr1[j]);
                        arr1[j] = -1;
                    }
                }
            }

            Arrays.sort(arr1);
            for (int i = 0; i < arr1.length; i++) {
                if (arr1[i] != -1) {
                    result.add(arr1[i]);
                }
            }
            return result.stream().mapToInt(Integer::intValue).toArray();
        }
    }

    // https://leetcode.com/problems/relative-sort-array/description/
    static class Solution2 {
        public int[] relativeSortArray(int[] arr1, int[] arr2) {
            Map<Integer, Integer> countMap = new HashMap<>();
            List<Integer> remaining = new ArrayList<>();
            List<Integer> result = new ArrayList<>();

            for (int value : arr2) {
                countMap.put(value, 0);
            }

            for (int value : arr1) {
                if (countMap.containsKey(value)) {
                    countMap.put(value, countMap.get(value) + 1);
                } else {
                    remaining.add(value);
                }
            }

            Collections.sort(remaining);

            for (int value : arr2) {
                for (int j = 0; j < countMap.get(value); j++) {
                    result.add(value);
                }
            }

            result.addAll(remaining);

            return result.stream().mapToInt(Integer::intValue).toArray();
        }
    }

    // https://leetcode.com/problems/relative-sort-array/description/
    static class Solution3 {
        public int[] relativeSortArray(int[] arr1, int[] arr2) {
            int maxElement = Arrays.stream(arr1).max().orElse(0);
            int[] count = new int[maxElement + 1];
            for (int element : arr1) {
                count[element]++;
            }

            List<Integer> result = new ArrayList<>();
            for (int value : arr2) {
                while (count[value] > 0) {
                    result.add(value);
                    count[value]--;
                }
            }

            for (int num = 0; num <= maxElement; num++) {
                while (count[num] > 0) {
                    result.add(num);
                    count[num]--;
                }
            }

            return result.stream().mapToInt(Integer::intValue).toArray();
        }
    }
}
