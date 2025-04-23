package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day886 {
    static class Solution1 {
        public int countLargestGroup(int n) {
            Map<Integer, Integer> map = new HashMap<>();
            int maxValue = 0;
            for (int i = 1; i <= n; ++i) {
                int key = 0, i0 = i;
                while (i0 != 0) {
                    key += i0 % 10;
                    i0 /= 10;
                }
                map.put(key, map.getOrDefault(key, 0) + 1);
                maxValue = Math.max(maxValue, map.get(key));
            }

            int count = 0;
            for (Map.Entry<Integer, Integer> kvpair : map.entrySet()) {
                if (kvpair.getValue() == maxValue) {
                    ++count;
                }
            }
            return count;
        }
    }
}