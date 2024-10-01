package leetcode_grind;

import java.util.*;

public class Day684 {
    // https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/description/?envType=daily-question&envId=2024-10-01
    static class Solution1 {
        public boolean canArrange(int[] arr, int k) {
            Map<Integer, Integer> remainderCount = new HashMap<>();
            for (int i : arr) {
                int rem = ((i % k) + k) % k;
                remainderCount.put(rem, remainderCount.getOrDefault(rem, 0) + 1);
            }

            for (int i : arr) {
                int rem = ((i % k) + k) % k;
                if (rem == 0) {
                    if (remainderCount.get(rem) % 2 == 1)
                        return false;
                } else if (!Objects.equals(remainderCount.get(rem), remainderCount.get(k - rem))) {
                    return false;
                }
            }
            return true;
        }
    }

    static class Solution2 {
        static class Comparator implements java.util.Comparator<Integer> {
            int k;

            Comparator(int k) {
                this.k = k;
            }

            @Override
            public int compare(Integer i, Integer j) {
                return ((k + (i % k)) % k) - ((k + (j % k)) % k);
            }
        }

        public boolean canArrange(int[] arr, int k) {
            Integer[] array = new Integer[arr.length];
            for (int i = 0; i < arr.length; i++) {
                array[i] = arr[i];
            }
            Arrays.sort(array, new Comparator(k));

            int start = 0, end = array.length - 1;
            while (start < end) {
                if (array[start] % k != 0)
                    break;
                if (array[start + 1] % k != 0)
                    return false;
                start = start + 2;
            }
            while (start < end) {
                if ((array[start] + array[end]) % k != 0)
                    return false;
                start++;
                end--;
            }
            return true;
        }
    }
}
