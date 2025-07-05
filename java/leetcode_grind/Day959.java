package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day959 {
    // https://leetcode.com/problems/find-lucky-integer-in-an-array/description/?envType=daily-question&envId=2025-07-05
    static class Solution1 {
        public int findLucky(int[] arr) {
            Map<Integer, Integer> counts = new HashMap<>();
            for (Integer num : arr) {
                counts.put(num, counts.getOrDefault(num, 0) + 1);
            }
            int largestLuckyNumber = -1;
            for (Map.Entry<Integer, Integer> entry : counts.entrySet()) {
                if (entry.getKey().equals(entry.getValue())) {
                    largestLuckyNumber = Math.max(largestLuckyNumber, entry.getKey());
                }
            }
            return largestLuckyNumber;
        }
    }
}
