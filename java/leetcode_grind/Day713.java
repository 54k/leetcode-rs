package leetcode_grind;

import java.util.*;

public class Day713 {
    // https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/description/?envType=daily-question&envId=2024-10-30
    static class Solution1 {
        public int minimumMountainRemovals(int[] nums) {
            int N = nums.length;
            int[] lisLength = new int[N];
            int[] ldsLength = new int[N];

            Arrays.fill(lisLength, 1);
            Arrays.fill(ldsLength, 1);

            for (int i = 0; i < N; i++) {
                for (int j = i - 1; j >= 0; j--) {
                    if (nums[i] > nums[j]) {
                        lisLength[i] = Math.max(lisLength[i], lisLength[j] + 1);
                    }
                }
            }

            for (int i = N - 1; i >= 0; i--) {
                for (int j = i + 1; j < N; j++) {
                    if (nums[i] > nums[j]) {
                        ldsLength[i] = Math.max(ldsLength[i], ldsLength[j] + 1);
                    }
                }
            }

            int minRemovals = Integer.MAX_VALUE;
            for (int i = 0; i < N; i++) {
                if (lisLength[i] > 1 && ldsLength[i] > 1) {
                    minRemovals = Math.min(minRemovals, N - lisLength[i] - ldsLength[i] + 1);
                }
            }

            return minRemovals;
        }
    }

    static class Solution2 {
        List<Integer> getLongestIncreasingSubsequenceLength(List<Integer> v) {
            List<Integer> lisLen = new ArrayList<>(Collections.nCopies(v.size(), 1));
            List<Integer> lis = new ArrayList<>();
            lis.add(v.get(0));

            for (int i = 1; i < v.size(); i++) {
                int index = lowerBound(lis, v.get(i));

                if (index == lis.size()) {
                    lis.add(v.get(i));
                } else {
                    lis.set(index, v.get(i));
                }

                lisLen.set(i, lis.size());
            }
            return lisLen;
        }

        int lowerBound(List<Integer> lis, int target) {
            int left = 0, right = lis.size() - 1;
            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (lis.get(mid) >= target) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            return left;
        }

        public int minimumMountainRemovals(int[] nums) {
            int N = nums.length;
            List<Integer> numsList = new ArrayList<>();
            for (int num : nums)
                numsList.add(num);

            List<Integer> lisLength = getLongestIncreasingSubsequenceLength(numsList);
            Collections.reverse(numsList);
            List<Integer> ldsLength = getLongestIncreasingSubsequenceLength(numsList);
            Collections.reverse(ldsLength);

            int minRemovals = Integer.MAX_VALUE;
            for (int i = 0; i < N; i++) {
                if (lisLength.get(i) > 1 && ldsLength.get(i) > 1) {
                    minRemovals = Math.min(
                            minRemovals,
                            N - lisLength.get(i) - ldsLength.get(i) + 1);
                }
            }
            return minRemovals;
        }
    }

    // https://leetcode.com/problems/paint-house-ii/description/
    static class Solution3 {
        int n;
        int k;
        int[][] costs;
        Map<String, Integer> memo;

        public int minCostII(int[][] costs) {
            if (costs.length == 0)
                return 0;
            k = costs[0].length;
            n = costs.length;
            this.costs = costs;
            this.memo = new HashMap<>();
            int minCost = Integer.MAX_VALUE;
            for (int color = 0; color < k; color++) {
                minCost = Math.min(minCost, memoSolve(0, color));
            }
            return minCost;
        }

        int memoSolve(int houseNumber, int color) {
            if (houseNumber == n - 1) {
                return costs[houseNumber][color];
            }

            if (memo.containsKey(getKey(houseNumber, color))) {
                return memo.get(getKey(houseNumber, color));
            }

            int minRemainingCost = Integer.MAX_VALUE;
            for (int nextColor = 0; nextColor < k; nextColor++) {
                if (color == nextColor) {
                    continue;
                }
                int currentRemainingCost = memoSolve(houseNumber + 1, nextColor);
                minRemainingCost = Math.min(currentRemainingCost, minRemainingCost);
            }
            int totalCost = costs[houseNumber][color] + minRemainingCost;
            memo.put(getKey(houseNumber, color), totalCost);
            return totalCost;
        }

        String getKey(int n, int color) {
            return String.valueOf(n) + " " + String.valueOf(color);
        }
    }

    static class Solution4 {
        public int minCostII(int[][] costs) {
            if (costs.length == 0)
                return 0;
            int k = costs[0].length;
            int n = costs.length;

            for (int house = 1; house < n; house++) {
                for (int color = 0; color < k; color++) {
                    int min = Integer.MAX_VALUE;
                    for (int previousColor = 0; previousColor < k; previousColor++) {
                        if (color == previousColor)
                            continue;
                        min = Math.min(min, costs[house - 1][previousColor]);
                    }
                    costs[house][color] += min;
                }
            }

            int min = Integer.MAX_VALUE;
            for (int c : costs[n - 1]) {
                min = Math.min(min, c);
            }
            return min;
        }
    }

    static class Solution5 {
        public int minCostII(int[][] costs) {
            if (costs.length == 0)
                return 0;
            int k = costs[0].length;
            int n = costs.length;

            int prevMin = -1;
            int prevSecondMin = -1;
            int prevMinColor = -1;
            for (int color = 0; color < k; color++) {
                int cost = costs[0][color];
                if (prevMin == -1 || cost < prevMin) {
                    prevSecondMin = prevMin;
                    prevMinColor = color;
                    prevMin = cost;
                } else if (prevSecondMin == -1 || cost < prevSecondMin) {
                    prevSecondMin = cost;
                }
            }

            for (int house = 1; house < n; house++) {
                int min = -1;
                int secondMin = -1;
                int minColor = -1;
                for (int color = 0; color < k; color++) {
                    int cost = costs[house][color];
                    if (color == prevMinColor) {
                        cost += prevSecondMin;
                    } else {
                        cost += prevMin;
                    }

                    if (min == -1 || cost < min) {
                        secondMin = min;
                        minColor = color;
                        min = cost;
                    } else if (secondMin == -1 || cost < secondMin) {
                        secondMin = cost;
                    }
                }
                prevMin = min;
                prevSecondMin = secondMin;
                prevMinColor = minColor;
            }

            return prevMin;
        }
    }
}
