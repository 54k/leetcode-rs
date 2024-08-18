package leetcode_grind;

import java.util.HashSet;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.TreeSet;

public class Day640 {
    // https://leetcode.com/problems/ugly-number-ii/description/?envType=daily-question&envId=2024-08-18
    static class Solution1 {
        public int nthUglyNumber(int n) {
            TreeSet<Long> uglyNumbersSet = new TreeSet<>();
            uglyNumbersSet.add(1L);
            Long currentUgly = 1L;
            for (int i = 0; i < n; i++) {
                currentUgly = uglyNumbersSet.pollFirst();
                uglyNumbersSet.add(currentUgly * 2);
                uglyNumbersSet.add(currentUgly * 3);
                uglyNumbersSet.add(currentUgly * 5);
            }
            return currentUgly.intValue();
        }
    }

    static class Solution2 {
        public int nthUglyNumber(int n) {
            PriorityQueue<Long> minHeap = new PriorityQueue<>();
            Set<Long> seenNumbers = new HashSet<>();
            int[] primeFactors = { 2, 3, 5 };
            minHeap.offer(1L);
            seenNumbers.add(1L);
            long currentUgly = 1L;
            for (int i = 0; i < n; i++) {
                currentUgly = minHeap.poll();

                for (int prime : primeFactors) {
                    long nextUgly = currentUgly * prime;
                    if (seenNumbers.add(nextUgly)) {
                        minHeap.offer(nextUgly);
                    }
                }
            }
            return (int) currentUgly;
        }
    }
}
