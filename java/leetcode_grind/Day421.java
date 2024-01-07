package leetcode_grind;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

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

    // https://leetcode.com/problems/maximum-profit-in-job-scheduling/description/
    static class Solution2 {
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
}
