package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashSet;
import java.util.List;

public class Day420 {
    // https://leetcode.com/problems/maximum-profit-in-job-scheduling/description
    static class Solution1 {
        int[] memo = new int[50001];

        public int jobScheduling(int[] startTime, int[] endTime, int[] profit) {
            List<List<Integer>> jobs = new ArrayList<>();
            Arrays.fill(memo, -1);

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

            return findMaxProfit(jobs, startTime, length, 0);
        }

        int findMaxProfit(List<List<Integer>> jobs, int[] startTime, int n, int position) {
            if (position == n) {
                return 0;
            }

            if (memo[position] != -1) {
                return memo[position];
            }

            int nextIndex = findNextJob(startTime, jobs.get(position).get(1));

            int maxProfit = Math.max(
                    findMaxProfit(jobs, startTime, n, position + 1),
                    jobs.get(position).get(2) + findMaxProfit(jobs, startTime, n, nextIndex));

            return memo[position] = maxProfit;
        }

        int findNextJob(int[] startTime, int lastEndingTime) {
            int start = 0;
            int end = startTime.length - 1;
            int nextIndex = startTime.length;

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
    }

    // https://leetcode.com/problems/longest-consecutive-sequence/description/
    static class Solution2 {
        public int longestConsecutive(int[] nums) {
            var set = new HashSet<Integer>();
            for (var n : nums) {
                set.add(n);
            }

            var longestStreak = 0;

            for (int num : set) {
                if (!set.contains(num - 1)) {
                    int currentNum = num;
                    int currentStreak = 1;

                    while (set.contains(currentNum + 1)) {
                        currentNum++;
                        currentStreak++;
                    }

                    longestStreak = Math.max(longestStreak, currentStreak);
                }
            }

            return longestStreak;
        }
    }

    // https://leetcode.com/problems/flip-game-ii/description/
    static class Solution3 {
        public boolean canWin(String currentState) {
            var states = currentState.toCharArray();
            var rec = new Object() {
                boolean apply() {
                    for (int i = 1; i < states.length; i++) {
                        if (states[i] == '+' && states[i] == states[i - 1]) {
                            states[i - 1] = '-';
                            states[i] = '-';

                            var wins = !apply();

                            states[i - 1] = '+';
                            states[i] = '+';

                            if (wins) {
                                return true;
                            }
                        }
                    }
                    return false;
                }
            };
            return rec.apply();
        }
    }
}
