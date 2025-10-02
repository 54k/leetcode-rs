package leetcode_grind;

public class Day1048 {
    // https://leetcode.com/problems/water-bottles-ii/description/?envType=daily-question&envId=2025-10-02
    static class Solution1 {
        public int maxBottlesDrunk(int numBottles, int numExchange) {
            int ans = 0;
            while (numBottles >= numExchange) {
                ans += numExchange;
                numBottles -= numExchange;
                numBottles++;
                numExchange++;
            }
            return ans + numBottles;
        }
    }

    static class Solution2 {
        public int maxBottlesDrunk(int numBottles, int numExchange) {
            int ans = numBottles;
            for (int empty = numBottles; empty >= numExchange; numExchange++) {
                ans++;
                empty -= numExchange - 1;
            }
            return ans;
        }
    }

    static class Solution3 {
        public int maxBottlesDrunk(int numBottles, int numExchange) {
            int t = 0;
            int empty = t * numExchange + (t * (t - 1)) / 2;
            int total = numBottles + t;
            int a = 1;
            int b = 2 * numExchange - 3;
            int c = -2 * numBottles;

            t = (int) Math.ceil(((-b + Math.sqrt(b * b - 4 * a * c)) / (2 * a)));
            return numBottles + t - 1;
        }
    }

}
