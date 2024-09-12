package leetcode_grind;

import java.util.*;

public class Day665 {
    // https://leetcode.com/problems/count-the-number-of-consistent-strings/description/?envType=daily-question&envId=2024-09-12
    static class Solution1 {
        public int countConsistentStrings(String allowed, String[] words) {
            int allowedBits = 0;
            for (int i = 0; i < allowed.length(); i++) {
                allowedBits |= 1 << (allowed.charAt(i) - 'a');
            }

            int consistentCount = 0;
            for (String word : words) {
                boolean isConsistent = true;

                for (int i = 0; i < word.length(); i++) {
                    int bit = (allowedBits >> (word.charAt(i) - 'a')) & 1;
                    if (bit == 0) {
                        isConsistent = false;
                    }
                }

                if (isConsistent) {
                    consistentCount++;
                }
            }
            return consistentCount;
        }
    }

    // https://leetcode.com/problems/super-ugly-number/description/?envType=problem-list-v2&envId=dynamic-programming
    static class Solution2 {
        public int nthSuperUglyNumber(int n, int[] primes) {
            var minPQ = new PriorityQueue<long[]>(Comparator.comparingLong(a -> a[0]));
            for (int prime : primes) {
                minPQ.offer(new long[] { prime, 0, prime }); // [value, index in output, prime factor]
            }
            long[] output = new long[n];
            output[0] = 1;
            for (int i = 1; i < n; i++) {
                long[] current = minPQ.poll();
                long value = current[0];
                int idx = (int) current[1];
                long prime = current[2];

                if (value != output[i - 1]) {
                    output[i] = value;
                } else {
                    i--;
                }
                minPQ.offer(new long[] { output[idx + 1] * prime, idx + 1, prime });
            }
            return (int) output[n - 1];
        }
    }
}
