package leetcode_grind;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day421 {
    // https://leetcode.com/problems/arithmetic-slices-ii-subsequence/description
    static class Solution1 {
        int n;
        int ans;

        void dfs(int dep, int[] nums, List<Long> cur) {
            if (dep == n) {
                if (cur.size() < 3) {
                    return;
                }

                long diff = cur.get(1) - cur.get(0);
                for (int i = 1; i < cur.size(); i++) {
                    if (cur.get(i) - cur.get(i - 1) != diff) {
                        return;
                    }
                }
                ans++;
                return;
            }

            dfs(dep + 1, nums, cur);

            cur.add((long) nums[dep]);
            dfs(dep + 1, nums, cur);
            cur.remove((long) nums[dep]);
        }

        public int numberOfArithmeticSlices(int[] nums) {
            n = nums.length;
            ans = 0;
            List<Long> cur = new ArrayList<Long>();
            dfs(0, nums, cur);
            return (int) ans;
        }
    }

    // https://leetcode.com/problems/arithmetic-slices-ii-subsequence/description
    static class Solution2 {
        public int numberOfArithmeticSlices(int[] nums) {
            int n = nums.length;
            long ans = 0;
            Map<Integer, Integer>[] cnt = new Map[n];

            for (int i = 0; i < n; i++) {
                cnt[i] = new HashMap<>();

                for (int j = 0; j < i; j++) {
                    long delta = (long) nums[i] - (long) nums[j];

                    if (delta < Integer.MIN_VALUE || delta > Integer.MAX_VALUE) {
                        continue;
                    }

                    int diff = (int) delta;
                    int sum = cnt[j].getOrDefault(diff, 0);
                    int origin = cnt[i].getOrDefault(diff, 0);
                    cnt[i].put(diff, origin + sum + 1);
                    ans += sum;
                }
            }
            return (int) ans;
        }
    }

    // https://leetcode.com/problems/maximum-profit-in-job-scheduling/description/
    static class Solution3 {
        int memo[] = new int[50001];

        int findNextJob(int[] startTime, int lastEndingTime) {
            int start = 0, end = startTime.length - 1, nextIndex = startTime.length;
            while (start <= end) {
                int mid = (start + end) / 2;

                if (startTime[mid] >= lastEndingTime) {
                    nextIndex = mid;
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            }
            return nextIndex;
        }

        int findMaxProfit(List<List<Integer>> jobs, int[] startTime) {
            int length = startTime.length;

            for (int position = length - 1; position >= 0; position--) {
                int currProfit = 0;
                int nextIndex = findNextJob(startTime, jobs.get(position).get(1));

                if (nextIndex != length) {
                    currProfit = jobs.get(position).get(2) + memo[nextIndex];
                } else {
                    currProfit = jobs.get(position).get(2);
                }

                if (position == length - 1) {
                    memo[position] = currProfit;
                } else {
                    memo[position] = Math.max(currProfit, memo[position + 1]);
                }
            }

            return memo[0];
        }

        public int jobScheduling(int[] startTime, int[] endTime, int[] profit) {
            List<List<Integer>> jobs = new ArrayList<>();

            int length = profit.length;
            for (int i = 0; i < length; i++) {
                ArrayList<Integer> currJob = new ArrayList<>();
                currJob.add(startTime[i]);
                currJob.add(endTime[i]);
                currJob.add(profit[i]);

                jobs.add(currJob);
            }

            jobs.sort(Comparator.comparingInt(a -> a.get(0)));

            for (int i = 0; i < length; i++) {
                startTime[i] = jobs.get(i).get(0);
            }

            return findMaxProfit(jobs, startTime);
        }
    }

    static class Solution4 {
        static class The_Comparator implements Comparator<ArrayList<Integer>> {
            public int compare(ArrayList<Integer> list1, ArrayList<Integer> list2) {
                return list1.get(0) - list2.get(0);
            }
        }

        int findMaxProfit(List<List<Integer>> jobs) {
            int n = jobs.size(), maxProfit = 0;

            PriorityQueue<ArrayList<Integer>> pq = new PriorityQueue<>(new The_Comparator());

            for (int i = 0; i < n; i++) {
                int start = jobs.get(i).get(0);
                int end = jobs.get(i).get(1);
                int profit = jobs.get(i).get(2);

                while (!pq.isEmpty() && start >= pq.peek().get(0)) {
                    maxProfit = Math.max(maxProfit, pq.peek().get(1));
                    pq.remove();
                }

                ArrayList<Integer> combinedJob = new ArrayList<>();
                combinedJob.add(end);
                combinedJob.add(profit + maxProfit);

                pq.add(combinedJob);
            }

            while (!pq.isEmpty()) {
                maxProfit = Math.max(maxProfit, pq.peek().get(1));
                pq.remove();
            }

            return maxProfit;
        }

        public int jobScheduling(int[] startTime, int[] endTime, int[] profit) {
            List<List<Integer>> jobs = new ArrayList<>();
            for (int i = 0; i < startTime.length; i++) {
                List<Integer> cur = new ArrayList<>();
                cur.add(startTime[i]);
                cur.add(endTime[i]);
                cur.add(profit[i]);
                jobs.add(cur);
            }
            jobs.sort(Comparator.comparingInt(a -> a.get(0)));

            return findMaxProfit(jobs);
        }
    }

    // https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/description/
    static class Solution5 {
        public boolean isValidSerialization(String preorder) {
            int slots = 1;

            int n = preorder.length();
            for (int i = 0; i < n; i++) {
                if (preorder.charAt(i) == ',') {
                    --slots;
                    if (slots < 0)
                        return false;
                    if (preorder.charAt(i - 1) != '#')
                        slots += 2;
                }
            }

            slots = (preorder.charAt(n - 1) == '#') ? slots - 1 : slots + 1;
            return slots == 0;
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/description
    static class Solution6 {
        public int maxProfitDPApproach(int k, int[] prices) {
            int n = prices.length;

            if (n <= 0 || k <= 0) {
                return 0;
            }

            if (k / 2 > n) {
                int res = 0;
                for (int i = 1; i < n; i++) {
                    res += Math.max(0, prices[i] - prices[i - 1]);
                }
                return res;
            }

            int[][][] dp = new int[n][k + 1][2];
            int INF = -1000000000;
            for (int i = 0; i < n; i++) {
                for (int j = 0; j <= k; j++) {
                    dp[i][j][0] = INF;
                    dp[i][j][1] = INF;
                }
            }

            dp[0][0][0] = 0;
            dp[0][1][1] = -prices[0];

            for (int i = 1; i < n; i++) {
                for (int j = 0; j <= k; j++) {
                    dp[i][j][0] = Math.max(dp[i - 1][j][0], dp[i - 1][j][1] + prices[i]);

                    if (j > 0) {
                        dp[i][j][1] = Math.max(dp[i - 1][j][1], dp[i - 1][j - 1][0] - prices[i]);
                    }
                }
            }

            int res = 0;
            for (int j = 0; j <= k; j++) {
                res = Math.max(res, dp[n - 1][j][0]);
            }

            return res;
        }

        public int maxProfitMergeApproach(int k, int[] prices) {
            int n = prices.length;

            if (n <= 0 || k <= 0) {
                return 0;
            }

            ArrayList<int[]> transactions = new ArrayList<>();
            int start = 0;
            int end = 0;

            for (int i = 1; i < n; i++) {
                if (prices[i] >= prices[i - 1]) {
                    end = i;
                } else {
                    if (end > start) {
                        int[] t = { start, end };
                        transactions.add(t);
                    }
                    start = i;
                }
            }

            if (end > start) {
                int[] t = { start, end };
                transactions.add(t);
            }

            while (transactions.size() > k) {
                int delete_index = 0;
                int min_delete_loss = Integer.MAX_VALUE;

                for (int i = 0; i < transactions.size(); i++) {
                    int[] t = transactions.get(i);
                    int profit_loss = prices[t[1]] - prices[t[0]];
                    if (profit_loss < min_delete_loss) {
                        min_delete_loss = profit_loss;
                        delete_index = i;
                    }
                }

                int merge_index = 0;
                int min_merge_loss = Integer.MAX_VALUE;

                for (int i = 1; i < transactions.size(); i++) {
                    int[] t1 = transactions.get(i - 1);
                    int[] t2 = transactions.get(i);
                    int profit_loss = prices[t1[1]] - prices[t2[0]];
                    if (profit_loss < min_merge_loss) {
                        min_merge_loss = profit_loss;
                        merge_index = i;
                    }
                }

                if (min_delete_loss <= min_merge_loss) {
                    transactions.remove(delete_index);
                } else {
                    int[] t1 = transactions.get(merge_index - 1);
                    int[] t2 = transactions.get(merge_index);
                    t1[1] = t2[1];
                    transactions.remove(merge_index);
                }
            }

            int res = 0;
            for (int[] t : transactions) {
                res += prices[t[1]] - prices[t[0]];
            }
            return res;
        }
    }

    // https://leetcode.com/problems/maximum-profit-from-trading-stocks/description/
    static class Solution7 {
        public int maximumProfit2DKnapsack(int[] present, int[] future, int budget) {
            int n = present.length;
            int[][] dp = new int[n + 1][budget + 1];

            for (int i = 1; i <= n; i++) {
                for (int j = 0; j <= budget; j++) {
                    dp[i][j] = dp[i - 1][j];
                    if (j >= present[i - 1]) {
                        int diff = j - present[i - 1];
                        dp[i][j] = Math.max(dp[i][j], dp[i - 1][diff] + future[i - 1]);
                    }
                }
            }

            int ans = 0;
            for (int i = 1; i <= n; i++) {
                for (int b = 0; b <= budget; b++) {
                    ans = Math.max(ans, dp[i][b] - b);
                }
            }
            return ans;
        }

        public int maximumProfit1DKnapsack(int[] present, int[] future, int budget) {
            int n = present.length;
            int[] dp = new int[budget + 1];

            for (int i = 0; i < n; i++) {
                for (int j = budget; j >= present[i]; j--) {
                    dp[j] = Math.max(dp[j], dp[j - present[i]] + future[i] - present[i]);
                }
            }
            return dp[budget];
        }
    }
}
