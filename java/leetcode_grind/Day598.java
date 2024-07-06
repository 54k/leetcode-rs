package leetcode_grind;

public class Day598 {
    // https://leetcode.com/problems/pass-the-pillow/description/?envType=daily-question&envId=2024-07-06
    static class Solution1 {
        public int passThePillow(int n, int time) {
            int i = 1;
            int dir = 1;
            while (time > 0) {
                if ((dir == 1 && i == n) || (dir == -1 && i == 1)) {
                    dir = -dir;
                }
                i += dir;
                time--;
            }
            return i;
        }
    }
}
