package leetcode_grind;

import java.util.Objects;

public class Day345 {
    // https://leetcode.com/problems/power-of-two/description/
    static class Solution1 {
        public boolean isPowerOfTwo1(int n) {
            if (n == 0)
                return false;
            long x = (long) n;
            return (x & (-x)) == x;
        }

        public boolean isPowerOfTwo2(int n) {
            if (n == 0)
                return false;
            long x = (long) n;
            return (x & (x - 1)) == 0;
        }
    }

    // https://leetcode.com/problems/power-of-three/description/
    static class Solution2 {
        public boolean isPowerOfThreeConvertToBase3(int n) {
            if (n == 0)
                return false;
            var base3 = "";
            while (n > 0) {
                base3 = Objects.toString(n % 3) + base3;
                n /= 3;
            }
            return Objects.toString(base3).matches("^10*$");
        }

        public boolean isPowerOfThree(int n) {
            return n > 0 && 1162261467 % n == 0;
        }
    }

    // https://leetcode.com/problems/power-of-four/description/
    static class Solution3 {
        public boolean isPowerOfFour(int num) {
            return (num > 0) && (num & (num - 1)) == 0 && (num & 0xaaaaaaaa) == 0;
        }
    }
}
