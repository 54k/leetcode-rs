package leetcode_grind;

public class Day784 {
    // https://leetcode.com/problems/construct-k-palindrome-strings/description/?envType=daily-question&envId=2025-01-11
    static class Solution1 {
        public boolean canConstruct(String s, int k) {
            if (s.length() < k)
                return false;
            if (s.length() == k)
                return true;

            int[] freq = new int[26];
            int oddCount = 0;

            for (char chr : s.toCharArray()) {
                freq[chr - 'a']++;
            }

            for (int count : freq) {
                if (count % 2 == 1) {
                    oddCount++;
                }
            }

            return oddCount <= k;
        }
    }

    // https://leetcode.com/problems/random-pick-with-weight/description/
    static class Solution2 {
        private int[] prefixSums;
        private int totalSum;

        public Solution2(int[] w) {
            this.prefixSums = new int[w.length];
            int prefixSum = 0;
            for (int i = 0; i < w.length; i++) {
                prefixSum += w[i];
                this.prefixSums[i] = prefixSum;
            }
            this.totalSum = prefixSum;
        }

        public int pickIndex() {
            double target = this.totalSum * Math.random();
            int i = 0;
            for (; i < this.prefixSums.length; i++) {
                if (target < this.prefixSums[i]) {
                    return i;
                }
            }
            return i - 1; // never happen
        }
    }

    /**
     * Your Solution object will be instantiated and called as such:
     * Solution obj = new Solution(w);
     * int param_1 = obj.pickIndex();
     */
}
