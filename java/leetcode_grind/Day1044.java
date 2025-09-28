package leetcode_grind;

import java.util.Arrays;

public class Day1044 {
    // https://leetcode.com/problems/largest-perimeter-triangle/description/?envType=daily-question&envId=2025-09-28
    static class Solution1 {
        public int largestPerimeter(int[] A) {
            Arrays.sort(A);
            for (int i = A.length - 3; i >= 0; i--) {
                if (A[i] + A[i + 1] > A[i + 2]) {
                    return A[i] + A[i + 1] + A[i + 2];
                }
            }
            return 0;
        }
    }

    // https://leetcode.com/problems/coin-change/description/
    static class Solution2 {
        public int coinChange(int[] coins, int amount) {
            int[] dp = new int[amount + 1];
            Arrays.fill(dp, amount + 1);

            dp[0] = 0;
            for (int i = 1; i <= amount; i++) {
                for (int coin : coins) {
                    if (i - coin < 0)
                        continue;
                    dp[i] = Math.min(dp[i], dp[i - coin] + 1);
                }
            }

            return dp[amount] == (amount + 1) ? -1 : dp[amount];
        }
    }

    // https://leetcode.com/problems/divide-chocolate/description/
    static class Solution3 {
        public int maximizeSweetness(int[] sweetness, int k) {
            int numberOfPeople = k + 1;
            int left = Arrays.stream(sweetness).min().getAsInt();
            int right = Arrays.stream(sweetness).sum() / numberOfPeople;

            while (left < right) {
                int mid = (left + right + 1) / 2;
                int curSweetness = 0;
                int peopleWithChocolate = 0;

                for (int s : sweetness) {
                    curSweetness += s;
                    if (curSweetness >= mid) {
                        peopleWithChocolate += 1;
                        curSweetness = 0;
                    }
                }

                if (peopleWithChocolate >= numberOfPeople) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            return left;
        }
    }
}
