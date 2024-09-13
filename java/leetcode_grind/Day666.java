package leetcode_grind;

import java.util.*;

public class Day666 {
    // https://leetcode.com/problems/xor-queries-of-a-subarray/description/?envType=daily-question&envId=2024-09-13
    static class Solution1 {
        public int[] xorQueries(int[] arr, int[][] queries) {
            var res = new int[queries.length];
            var qi = 0;
            for (var q : queries) {
                var xor = 0;
                for (int i = q[0]; i <= q[1]; i++) {
                    xor ^= arr[i];
                }
                res[qi++] = xor;
            }
            return res;
        }
    }

    static class Solution2 {
        public int[] xorQueries(int[] arr, int[][] queries) {
            List<Integer> result = new ArrayList<>();

            for (int i = 1; i < arr.length; i++) {
                arr[i] ^= arr[i - 1];
            }

            for (int[] q : queries) {
                if (q[0] > 0) {
                    result.add(arr[q[0] - 1] ^ arr[q[1]]);
                } else {
                    result.add(arr[q[1]]);
                }
            }

            return result.stream().mapToInt(i -> i).toArray();
        }
    }
}
