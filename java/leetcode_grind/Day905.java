package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Set;

public class Day905 {
    // https://leetcode.com/problems/finding-3-digit-even-numbers/description/?envType=daily-question&envId=2025-05-12
    static class Solution1 {
        public int[] findEvenNumbers(int[] digits) {
            Set<Integer> nums = new HashSet<>();
            int n = digits.length;
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n; j++) {
                    for (int k = 0; k < n; k++) {
                        if (i == j || j == k || i == k) {
                            continue;
                        }
                        int num = digits[i] * 100 + digits[j] * 10 + digits[k];
                        if (num >= 100 && num % 2 == 0) {
                            nums.add(num);
                        }
                    }
                }
            }

            List<Integer> res = new ArrayList<>(nums);
            Collections.sort(res);
            int[] result = new int[res.size()];
            for (int i = 0; i < res.size(); i++) {
                result[i] = res.get(i);
            }
            return result;
        }
    }

    // https://leetcode.com/problems/find-median-from-data-stream/description/
    static class MedianFinder {
        PriorityQueue<Integer> lo = new PriorityQueue<>(Collections.reverseOrder());
        PriorityQueue<Integer> hi = new PriorityQueue<>();

        public MedianFinder() {

        }

        public void addNum(int num) {
            lo.add(num);
            hi.add(lo.poll());

            if (hi.size() > lo.size()) {
                lo.add(hi.poll());
            }
        }

        public double findMedian() {
            if (lo.size() > hi.size()) {
                return (double) lo.peek();
            }
            return 1.0 * (lo.peek() + hi.peek()) / 2.0;
        }
    }

    /**
     * Your MedianFinder object will be instantiated and called as such:
     * MedianFinder obj = new MedianFinder();
     * obj.addNum(num);
     * double param_2 = obj.findMedian();
     */

}
