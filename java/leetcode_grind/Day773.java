package leetcode_grind;

import java.util.*;

public class Day773 {
    // https://leetcode.com/problems/minimum-cost-for-tickets/description/?envType=daily-question&envId=2024-12-31
    static class Solution1 {
        HashSet<Integer> isTravelNeeded = new HashSet<>();

        int solve(int[] dp, int[] days, int[] costs, int currDay) {
            if (currDay > days[days.length - 1]) {
                return 0;
            }

            if (!isTravelNeeded.contains(currDay)) {
                return solve(dp, days, costs, currDay + 1);
            }

            if (dp[currDay] != -1) {
                return dp[currDay];
            }

            int oneDay = costs[0] + solve(dp, days, costs, currDay + 1);
            int sevenDay = costs[1] + solve(dp, days, costs, currDay + 7);
            int thirtyDay = costs[2] + solve(dp, days, costs, currDay + 30);

            return dp[currDay] = Math.min(oneDay, Math.min(sevenDay, thirtyDay));
        }

        public int mincostTickets(int[] days, int[] costs) {
            int lastDay = days[days.length - 1];
            int[] dp = new int[lastDay + 1];
            Arrays.fill(dp, -1);
            for (int day : days) {
                isTravelNeeded.add(day);
            }
            return solve(dp, days, costs, 1);
        }
    }

    static class Solution2 {
        public int mincostTickets(int[] days, int[] costs) {
            int lastDay = days[days.length - 1];
            int[] dp = new int[lastDay + 1];
            Arrays.fill(dp, 0);

            int i = 0;
            for (int day = 1; day <= lastDay; day++) {
                if (day < days[i]) {
                    dp[day] = dp[day - 1];
                } else {
                    i++;
                    dp[day] = Math.min(dp[day - 1] + costs[0], Math.min(dp[Math.max(0, day - 7)] + costs[1],
                            dp[Math.max(0, day - 30)] + costs[2]));
                }
            }
            return dp[lastDay];
        }
    }
}
