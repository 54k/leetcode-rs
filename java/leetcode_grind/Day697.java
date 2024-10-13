package leetcode_grind;

import java.util.*;

public class Day697 {
    // https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/description/?envType=daily-question&envId=2024-10-13
    static class Solution1 {
        public int[] smallestRange(List<List<Integer>> nums) {
            int k = nums.size();
            int[] indices = new int[k];
            int[] range = new int[] { 0, Integer.MAX_VALUE };
            while (true) {
                int curMin = Integer.MAX_VALUE, curMax = Integer.MIN_VALUE, minListIndex = 0;

                for (int i = 0; i < k; i++) {
                    int currentElement = nums.get(i).get(indices[i]);

                    if (currentElement < curMin) {
                        curMin = currentElement;
                        minListIndex = i;
                    }

                    if (currentElement > curMax) {
                        curMax = currentElement;
                    }
                }

                if (curMax - curMin < range[1] - range[0]) {
                    range[0] = curMin;
                    range[1] = curMax;
                }

                if (++indices[minListIndex] == nums.get(minListIndex).size())
                    break;
            }
            return range;
        }
    }

    static class Solution2 {
        public int[] smallestRange(List<List<Integer>> nums) {
            PriorityQueue<int[]> pq = new PriorityQueue<>(
                    Comparator.comparingInt(a -> a[0]));

            int maxVal = Integer.MIN_VALUE, rangeStart = 0, rangeEnd = Integer.MAX_VALUE;
            for (int i = 0; i < nums.size(); i++) {
                pq.offer(new int[] { nums.get(i).get(0), i, 0 });
                maxVal = Math.max(maxVal, nums.get(i).get(0));
            }

            while (pq.size() == nums.size()) {
                int[] data = pq.poll();
                int minVal = data[0], row = data[1], col = data[2];

                if (maxVal - minVal < rangeEnd - rangeStart) {
                    rangeStart = minVal;
                    rangeEnd = maxVal;
                }

                if (col + 1 < nums.get(row).size()) {
                    int nextVal = nums.get(row).get(col + 1);
                    pq.offer(new int[] { nextVal, row, col + 1 });
                    maxVal = Math.max(maxVal, nextVal);
                }
            }

            return new int[] { rangeStart, rangeEnd };
        }
    }

    static class Solution3 {
        public int[] smallestRange(List<List<Integer>> nums) {
            List<int[]> merged = new ArrayList<>();
            for (int i = 0; i < nums.size(); i++) {
                for (int num : nums.get(i)) {
                    merged.add(new int[] { num, i });
                }
            }
            merged.sort(Comparator.comparingInt(a -> a[0]));
            Map<Integer, Integer> freq = new HashMap<>();
            int left = 0, count = 0;
            int rangeStart = 0, rangeEnd = Integer.MAX_VALUE;

            for (int right = 0; right < merged.size(); right++) {
                freq.put(merged.get(right)[1], freq.getOrDefault(merged.get(right)[1], 0) + 1);
                if (freq.get(merged.get(right)[1]) == 1)
                    count++;

                while (count == nums.size()) {
                    int curRange = merged.get(right)[0] - merged.get(left)[0];
                    if (curRange < rangeEnd - rangeStart) {
                        rangeStart = merged.get(left)[0];
                        rangeEnd = merged.get(right)[0];
                    }

                    freq.put(merged.get(left)[1], freq.get(merged.get(left)[1]) - 1);
                    if (freq.get(merged.get(left)[1]) == 0)
                        count--;
                    left++;
                }
            }

            return new int[] { rangeStart, rangeEnd };
        }
    }
}
