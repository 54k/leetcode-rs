package leetcode_grind;

public class Day1056 {
    // https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/description/?envType=daily-question&envId=2025-10-10
    static class Solution1 {
        public int maximumEnergy(int[] energy, int k) {
            int n = energy.length;
            int ans = Integer.MIN_VALUE;

            for (int i = n - k; i < n; i++) {
                int sum = 0;
                for (int j = i; j >= 0; j -= k) {
                    sum += energy[j];
                    ans = Math.max(ans, sum);
                }
            }
            return ans;
        }
    }
}