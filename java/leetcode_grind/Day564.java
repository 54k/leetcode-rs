package leetcode_grind;

import java.util.HashMap;

public class Day564 {
    // https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/description/
    static class Solution1 {
        public int countTriplets(int[] arr) {
            int[] prefixXOR = new int[arr.length + 1];
            prefixXOR[0] = 0;
            System.arraycopy(arr, 0, prefixXOR, 1, arr.length);
            int size = prefixXOR.length;
            int count = 0;

            for (int i = 1; i < size; i++)
                prefixXOR[i] ^= prefixXOR[i - 1];

            HashMap<Integer, Integer> countMap = new HashMap<>();
            HashMap<Integer, Integer> totalMap = new HashMap<>();

            for (int i = 0; i < size; i++) {
                count += countMap.getOrDefault(prefixXOR[i], 0) * (i - 1) - totalMap.getOrDefault(prefixXOR[i], 0);

                totalMap.put(
                        prefixXOR[i],
                        totalMap.getOrDefault(prefixXOR[i], 0) + i);
                countMap.put(
                        prefixXOR[i],
                        countMap.getOrDefault(prefixXOR[i], 0) + 1);
            }
            return count;
        }
    }
}
