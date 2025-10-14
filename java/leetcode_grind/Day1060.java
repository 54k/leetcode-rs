package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;

public class Day1060 {
    // https://leetcode.com/problems/adjacent-increasing-subarrays-detection-i/description/?envType=daily-question&envId=2025-10-14
    static class Solution1 {
        public boolean hasIncreasingSubarrays(List<Integer> nums, int k) {
            int n = nums.size();
            int inc = 1, prevInc = 0, maxLen = 0;
            for (int i = 1; i < n; i++) {
                if (nums.get(i) > nums.get(i - 1))
                    inc++;
                else {
                    prevInc = inc;
                    inc = 1;
                }
                maxLen = Math.max(maxLen, Math.max(inc >> 1, Math.min(prevInc, inc)));
                if (maxLen >= k)
                    return true;
            }
            return false;
        }
    }

    // https://leetcode.com/problems/logger-rate-limiter/description/
    static class Logger {
        Map<String, Integer> map = new HashMap<>();

        public boolean shouldPrintMessage(int timestamp, String message) {
            if (map.getOrDefault(message, timestamp) <= timestamp) {
                map.put(message, timestamp + 10);
                return true;
            }
            return false;
        }
    }

    // https://leetcode.com/problems/insert-delete-getrandom-o1/description/
    static class RandomizedSet {
        Map<Integer, Integer> dict;
        List<Integer> list;
        Random rand = new Random();

        public RandomizedSet() {
            dict = new HashMap<>();
            list = new ArrayList<>();
        }

        public boolean insert(int val) {
            if (dict.containsKey(val))
                return false;

            dict.put(val, list.size());
            list.add(list.size(), val);
            return true;
        }

        public boolean remove(int val) {
            if (!dict.containsKey(val))
                return false;

            int lastElement = list.get(list.size() - 1);

            int idx = dict.get(val);
            list.set(idx, lastElement);
            dict.put(lastElement, idx);

            list.remove(list.size() - 1);
            dict.remove(val);
            return true;
        }

        public int getRandom() {
            return list.get(rand.nextInt(list.size()));
        }
    }
}
