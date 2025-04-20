package leetcode_grind;

public class Day883 {
    // https://leetcode.com/problems/rabbits-in-forest/description/
    static class Solution1 {
        public int numRabbits(int[] answers) {
            int[] count = new int[1000];
            for (int x : answers)
                count[x]++;
            int ans = 0;
            for (int k = 0; k < 1000; k++) {
                ans += Math.floorMod(-count[k], k + 1) + count[k];
            }
            return ans;
        }
    }
}
