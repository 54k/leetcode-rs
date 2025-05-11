package leetcode_grind;

public class Day904 {
    // https://leetcode.com/problems/three-consecutive-odds/submissions/1630957794/?envType=daily-question&envId=2025-05-11
    static class Solution1 {
        public boolean threeConsecutiveOdds(int[] arr) {
            int n = arr.length;
            for (int i = 0; i < n - 2; i++) {
                if ((arr[i] * arr[i + 1] * arr[i + 2]) % 2 == 1) {
                    return true;
                }
            }
            return false;
        }
    }
}
