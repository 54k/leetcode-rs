package leetcode_grind;

public class Day958 {
    // https://leetcode.com/problems/find-the-k-th-character-in-string-game-ii/description/?envType=daily-question&envId=2025-07-04
    static class Solution1 {
        public char kthCharacter(long k, int[] operations) {
            int ans = 0;
            int t;
            while (k != 1) {
                t = 63 - Long.numberOfLeadingZeros(k);
                if ((1L << t) == k) {
                    t--;
                }
                k = k - (1L << t);
                if (operations[t] != 0) {
                    ans++;
                }
            }
            return (char) ('a' + (ans % 26));
        }
    }

    static class Solution2 {
        public char kthCharacter(long k, int[] operations) {
            int ans = 0;
            k--;
            for (int i = 63 - Long.numberOfLeadingZeros(k); i >= 0; i--) {
                if (((k >> i) & 1) == 1) {
                    ans += operations[i];
                }
            }
            return (char) ('a' + (ans % 26));
        }
    }

}
