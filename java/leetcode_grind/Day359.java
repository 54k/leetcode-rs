package leetcode_grind;

import java.util.Arrays;
import java.util.PriorityQueue;
import java.util.TreeSet;

public class Day359 {
    // https://leetcode.com/problems/seat-reservation-manager/description
    static class SeatManager1 {
        PriorityQueue<Integer> queue = new PriorityQueue<>();
        int marker = 1;

        public SeatManager1(int n) {
        }

        public int reserve() {
            if (!queue.isEmpty()) {
                return queue.poll();
            }
            return marker++;
        }

        public void unreserve(int seatNumber) {
            queue.add(seatNumber);
        }
    }

    static class SeatManager2 {
        int marker = 1;
        TreeSet<Integer> availableSeats = new TreeSet<>();

        public SeatManager2(int n) {

        }

        public int reserve() {
            if (!availableSeats.isEmpty()) {
                int seatNumber = availableSeats.first();
                availableSeats.remove(seatNumber);
                return seatNumber;
            }
            int seatNumber = marker;
            marker++;
            return seatNumber;
        }

        public void unreserve(int seatNumber) {
            availableSeats.add(seatNumber);
        }
    }

    // https://leetcode.com/problems/last-stone-weight-ii/description/
    static class Solution1 {
        public int lastStoneWeightII(int[] stones) {
            var dp = new boolean[1501];
            dp[0] = true;

            var sum = 0;

            for (var stone : stones) {
                sum += stone;
                for (int i = Math.min(1500, sum); i >= stone; i--) {
                    dp[i] |= dp[i - stone];
                }
            }

            for (int i = sum / 2; i >= 0; i--) {
                if (dp[i]) {
                    return sum - i - i;
                }
            }

            return 0;
        }
    }

    // https://leetcode.com/problems/partition-equal-subset-sum/description/
    static class Solution2 {
        public boolean canPartitionTopDown(int[] nums) {
            var totalSum = 0;
            for (var num : nums) {
                totalSum += num;
            }

            if (totalSum % 2 != 0) {
                return false;
            }

            var subsetSum = totalSum / 2;
            var n = nums.length;
            var memo = new Boolean[n + 1][subsetSum + 1];

            var dfs = new Object() {
                boolean apply(int n, int subsetSum) {
                    if (subsetSum == 0) {
                        return true;
                    }
                    if (n == 0 || subsetSum < 0) {
                        return false;
                    }
                    if (memo[n][subsetSum] != null) {
                        return memo[n][subsetSum];
                    }

                    var res = apply(n - 1, subsetSum - nums[n - 1]) || apply(n - 1, subsetSum);
                    memo[n][subsetSum] = res;
                    return res;
                }
            };

            return dfs.apply(n - 1, subsetSum);
        }

        public boolean canPartitionBottomUp(int[] nums) {
            var sum = 0;
            for (var num : nums) {
                sum += num;
            }
            if (sum % 2 == 1) {
                return false;
            }

            var dp = new boolean[nums.length + 1][sum / 2 + 1];
            dp[0][0] = true;

            for (int i = 1; i <= nums.length; i++) {
                for (int j = 0; j <= sum / 2; j++) {
                    dp[i][j] |= dp[i - 1][j];
                    if (j >= nums[i - 1]) {
                        dp[i][j] |= dp[i - 1][j - nums[i - 1]];
                    }
                }
            }

            return dp[nums.length][sum / 2];
        }

        public boolean canPartitionBottomUpOptimized(int[] nums) {
            var sum = Arrays.stream(nums).boxed().reduce(0, Integer::sum);
            if (sum % 2 == 1) {
                return false;
            }
            var dp = new boolean[sum / 2 + 1];
            dp[0] = true;

            for (var num : nums) {
                for (int j = sum / 2; j >= num; j--) {
                    dp[j] |= dp[j - num];
                }
            }

            return dp[sum / 2];
        }
    }
}
