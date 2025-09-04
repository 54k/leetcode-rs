package leetcode_grind;

public class Day1020 {
    // https://leetcode.com/problems/find-closest-person/description/?envType=daily-question&envId=2025-09-04
    static class Solution1 {

        public int findClosest(int x, int y, int z) {
            int dxz = Math.abs(x - z);
            int dyz = Math.abs(y - z);
            if (dxz < dyz) {
                return 1;
            } else if (dxz > dyz) {
                return 2;
            } else {
                return 0;
            }
        }
    }
}
