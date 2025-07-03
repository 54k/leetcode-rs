package leetcode_grind;

public class Day957 {
    // https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/description/?envType=daily-question&envId=2025-07-03
    static class Solution1 {
        public char kthCharacter(int k) {
            int ans = 0;
            int t;
            while (k != 1) {
                t = 31 - Integer.numberOfLeadingZeros(k);
                if ((1 << t) == k) {
                    t--;
                }
                k = k - (1 << t);
                ans++;
            }
            return (char) ('a' + ans);
        }
    }
}
