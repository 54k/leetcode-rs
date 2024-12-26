package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Arrays;
import java.util.Deque;
import java.util.TreeSet;

public class Day768 {
    // https://leetcode.com/problems/target-sum/description/
    static class Solution1 {
        int totalWays = 0;

        public int findTargetSumWays(int[] nums, int target) {
            calculateWays(nums, 0, 0, target);
            return totalWays;
        }

        void calculateWays(int[] nums, int currentIndex, int currentSum, int target) {
            if (currentIndex == nums.length) {
                if (currentSum == target)
                    totalWays++;
            } else {
                calculateWays(nums, currentIndex + 1, currentSum + nums[currentIndex], target);
                calculateWays(nums, currentIndex + 1, currentSum - nums[currentIndex], target);
            }
        }
    }

    static class Solution2 {
        int totalSum = 0;

        public int findTargetSumWays(int[] nums, int target) {
            totalSum = Arrays.stream(nums).sum();
            int[][] memo = new int[nums.length][2 * totalSum + 1];
            for (int[] row : memo) {
                Arrays.fill(row, Integer.MIN_VALUE);
            }
            return calculateWays(nums, 0, 0, target, memo);
        }

        int calculateWays(int[] nums, int currentIndex, int currentSum, int target, int[][] memo) {
            if (currentIndex == nums.length) {
                if (currentSum == target)
                    return 1;
                else
                    return 0;
            } else {
                if (memo[currentIndex][currentSum + totalSum] != Integer.MIN_VALUE) {
                    return memo[currentIndex][currentSum + totalSum];
                }

                int add = calculateWays(nums, currentIndex + 1, currentSum + nums[currentIndex], target, memo);
                int substract = calculateWays(nums, currentIndex + 1, currentSum - nums[currentIndex], target, memo);
                memo[currentIndex][currentSum + totalSum] = add + substract;
            }
            return memo[currentIndex][currentSum + totalSum];
        }
    }

    static class Solution3 {
        public int findTargetSumWays(int[] nums, int target) {
            int totalSum = Arrays.stream(nums).sum();
            int[][] dp = new int[nums.length][2 * totalSum + 1];
            dp[0][nums[0] + totalSum] = 1;
            dp[0][-nums[0] + totalSum] += 1;

            for (int index = 1; index < nums.length; index++) {
                for (int sum = -totalSum; sum <= totalSum; sum++) {
                    if (dp[index - 1][sum + totalSum] > 0) {
                        dp[index][sum + nums[index] + totalSum] += dp[index - 1][sum + totalSum];
                        dp[index][sum - nums[index] + totalSum] += dp[index - 1][sum + totalSum];
                    }
                }
            }

            return Math.abs(target) > totalSum ? 0 : dp[nums.length - 1][target + totalSum];
        }
    }

    // https://leetcode.com/problems/k-empty-slots/description/
    static class Solution4 {
        public int kEmptySlots(int[] flowers, int k) {
            TreeSet<Integer> active = new TreeSet<>();
            int day = 0;
            for (int flower : flowers) {
                day++;
                active.add(flower);
                Integer lower = active.lower(flower);
                Integer higher = active.higher(flower);

                if (lower != null && flower - lower - 1 == k || higher != null && higher - flower - 1 == k) {
                    return day;
                }
            }
            return -1;
        }
    }

    static class Solution5 {
        public int kEmptySlots(int[] flowers, int k) {
            int[] days = new int[flowers.length];
            for (int i = 0; i < days.length; i++) {
                days[flowers[i] - 1] = i + 1;
            }

            MinQueue<Integer> window = new MinQueue<>();
            int ans = days.length;

            for (int i = 0; i < days.length; i++) {
                int day = days[i];
                window.addLast(day);
                if (k <= i && i < days.length - 1) {
                    window.pollFirst();
                    if (k == 0 || days[i - k] < window.min() && days[i + 1] < window.min()) {
                        ans = Math.min(ans, Math.max(days[i - k], days[i + 1]));
                    }
                }
            }

            return ans < days.length ? ans : -1;
        }

        static class MinQueue<E extends Comparable<E>> extends ArrayDeque<E> {
            Deque<E> mins;

            public MinQueue() {
                mins = new ArrayDeque<E>();
            }

            @Override
            public void addLast(E x) {
                super.addLast(x);
                while (mins.peekLast() != null && x.compareTo(mins.peekLast()) < 0) {
                    mins.pollLast();
                }
                mins.addLast(x);
            }

            public E pollFirst() {
                E x = super.pollFirst();
                if (x == mins.peekFirst()) {
                    mins.pollFirst();
                }
                return x;
            }

            public E min() {
                return mins.peekFirst();
            }
        }
    }

    static class Solution6 {
        public int kEmptySlots(int[] flowers, int k) {
            int[] days = new int[flowers.length];
            for (int i = 0; i < days.length; i++) {
                days[flowers[i] - 1] = i + 1;
            }
            int ans = Integer.MAX_VALUE;
            int left = 0, right = k + 1;

            search: while (right < days.length) {
                for (int i = left + 1; i < right; i++) {
                    if (days[i] < days[left] || days[i] < days[right]) {
                        left = i;
                        right = i + k + 1;
                        continue search;
                    }
                }
                ans = Math.min(ans, Math.max(days[left], days[right]));
                left = right;
                right = left + k + 1;
            }
            return ans < Integer.MAX_VALUE ? ans : -1;
        }
    }
}
