package leetcode_grind;

public class Day1111 {
    // https://leetcode.com/problems/count-collisions-on-a-road/description/?envType=daily-question&envId=2025-12-04
    static class Solution1 {
        public int countCollisions(String directions) {
            int res = 0;
            int flag = -1;
            for (char c : directions.toCharArray()) {
                if (c == 'L') {
                    if (flag >= 0) {
                        res += flag + 1;
                        flag = 0;
                    }
                } else if (c == 'S') {
                    if (flag > 0) {
                        res += flag;
                    }
                    flag = 0;
                } else {
                    if (flag >= 0) {
                        flag++;
                    } else {
                        flag = 1;
                    }
                }
            }
            return res;
        }
    }

    static class Solution2 {
        public int countCollisions(String directions) {
            int n = directions.length();
            int l = 0;
            int r = n - 1;
            while (l < n && directions.charAt(l) == 'L') {
                l++;
            }
            while (r >= l && directions.charAt(r) == 'R') {
                r--;
            }

            int res = 0;
            for (int i = l; i <= r; i++) {
                if (directions.charAt(i) != 'S') {
                    res++;
                }
            }
            return res;
        }
    }
}
