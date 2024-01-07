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
}
