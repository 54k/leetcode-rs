package leetcode_grind;

import java.util.PriorityQueue;

public class Day577 {
    // https://leetcode.com/problems/maximum-product-after-k-increments/description/
    static class Solution1 {
        public int maximumProduct(int[] nums, int k) {
            var pq = new PriorityQueue<Integer>();
            var mod = (int) 1e9 + 7;
            for (var n : nums) {
                pq.add(n);
            }
            while (k-- > 0) {
                pq.add(pq.remove() + 1);
            }
            long ans = 1;
            while (!pq.isEmpty()) {
                ans *= pq.remove() % mod;
                ans %= mod;
            }
            return (int) ans;
        }
    }

    // https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/description/
    static class Solution2 {
        public long numberOfSubstrings(String s) {
            long answer = 0;
            long[] prefixCount = new long[26];
            for (int i = 0; i < s.length(); i++) {
                prefixCount[s.charAt(i) - 'a']++;
                answer += prefixCount[s.charAt(i) - 'a'];
            }
            return answer;
        }
    }

    // https://leetcode.com/problems/minimum-increment-to-make-array-unique/description
    static class Solution3 {
        public int minIncrementForUnique(int[] nums) {
            int n = nums.length;
            int max = 0;
            int minIncrements = 0;
            for (int val : nums) {
                max = Math.max(max, val);
            }
            int[] frequencyCount = new int[n + max];
            for (int val : nums) {
                frequencyCount[val]++;
            }
            for (int i = 0; i < frequencyCount.length; i++) {
                if (frequencyCount[i] <= 1)
                    continue;

                int duplicates = frequencyCount[i] - 1;
                frequencyCount[i + 1] += duplicates;
                frequencyCount[i] = 1;
                minIncrements += duplicates;
            }
            return minIncrements;
        }
    }
}