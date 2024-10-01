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

}
