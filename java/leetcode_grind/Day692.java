package leetcode_grind;

import java.util.Arrays;
import java.util.Stack;

public class Day692 {
    // https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/?envType=daily-question&envId=2024-10-08
    static class Solution1 {
        public int minSwaps(String s) {
            Stack<Character> stack = new Stack<>();
            int unbalanced = 0;
            for (int i = 0; i < s.length(); i++) {
                char ch = s.charAt(i);
                if (ch == '[')
                    stack.push(ch);
                else {
                    if (!stack.isEmpty())
                        stack.pop();
                    else
                        unbalanced++;
                }
            }
            return (unbalanced + 1) / 2;
        }
    }

    // https://leetcode.com/problems/maximum-coins-heroes-can-collect/description/?envType=weekly-question&envId=2024-10-08
    static class Solution2 {
        public long[] maximumCoins(int[] heroes, int[] monsters, int[] coins) {
            long[] ans = new long[heroes.length];
            int[][] monsterAndCoin = new int[monsters.length][2];
            for (int i = 0; i < monsters.length; i++) {
                monsterAndCoin[i][0] = monsters[i];
                monsterAndCoin[i][1] = coins[i];
            }
            Arrays.sort(monsterAndCoin, (a, b) -> a[0] - b[0]);

            long[] coinSum = new long[coins.length];
            long prefixSum = 0;
            for (int i = 0; i < monsterAndCoin.length; i++) {
                prefixSum += monsterAndCoin[i][1];
                coinSum[i] = prefixSum;
            }

            for (int i = 0; i < heroes.length; i++) {
                ans[i] = findTotalCoins(monsterAndCoin, heroes[i], coinSum);
            }
            return ans;
        }

        long findTotalCoins(int[][] monsterAndCoin, int heroPower, long[] coinSum) {
            int l = 0;
            int r = monsterAndCoin.length - 1;
            while (l <= r) {
                int mid = (l + r) / 2;
                if (monsterAndCoin[mid][0] > heroPower) {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }
            if (l == 0 && monsterAndCoin[l][0] > heroPower) {
                return 0;
            }
            return coinSum[r];
        }
    }

}
