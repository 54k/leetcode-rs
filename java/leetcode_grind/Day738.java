package leetcode_grind;

public class Day738 {
    // https://leetcode.com/problems/maximum-matrix-sum/description/?envType=daily-question&envId=2024-11-24
    static class Solution1 {
        public long maxMatrixSum(int[][] matrix) {
            long totalSum = 0;
            int minAbsVal = Integer.MAX_VALUE;
            int negativeCount = 0;

            for (int[] row : matrix) {
                for (int val : row) {
                    totalSum += Math.abs(val);
                    if (val < 0) {
                        negativeCount++;
                    }
                    minAbsVal = Math.min(minAbsVal, Math.abs(val));
                }
            }

            if (negativeCount % 2 != 0) {
                totalSum -= 2 * minAbsVal;
            }

            return totalSum;
        }
    }

    // https://leetcode.com/problems/optimal-division/description/
    static class Solution2 {
        static class T {
            float max_val, min_val;
            String min_str, max_str;
        }

        public String optimalDivision(int[] nums) {
            T t = optimal(nums, 0, nums.length - 1, "");
            return t.max_str;
        }

        T optimal(int[] nums, int start, int end, String res) {
            T t = new T();

            if (start == end) {
                t.max_val = nums[start];
                t.min_val = nums[start];
                t.min_str = "" + nums[start];
                t.max_str = "" + nums[start];
                return t;
            }

            t.min_val = Float.MAX_VALUE;
            t.max_val = Float.MIN_VALUE;

            for (int i = start; i < end; i++) {
                T left = optimal(nums, start, i, "");
                T right = optimal(nums, i + 1, end, "");

                if (t.min_val > left.min_val / right.max_val) {
                    t.min_val = left.min_val / right.max_val;
                    t.min_str = left.min_str + "/" + (i + 1 != end ? "(" : "") + right.max_str
                            + (i + 1 != end ? ")" : "");
                }

                if (t.max_val < left.max_val / right.min_val) {
                    t.max_val = left.max_val / right.min_val;
                    t.max_str = left.max_str + "/" + (i + 1 != end ? "(" : "") + right.min_str
                            + (i + 1 != end ? ")" : "");
                }
            }
            return t;
        }
    }

    static class Solution3 {
        static class T {
            float max_val, min_val;
            String min_str, max_str;
        }

        public String optimalDivision(int[] nums) {
            T[][] memo = new T[nums.length][nums.length];
            T t = optimal(nums, 0, nums.length - 1, "", memo);
            return t.max_str;
        }

        T optimal(int[] nums, int start, int end, String res, T[][] memo) {
            if (memo[start][end] != null) {
                return memo[start][end];
            }

            T t = new T();

            if (start == end) {
                t.max_val = nums[start];
                t.min_val = nums[start];
                t.min_str = "" + nums[start];
                t.max_str = "" + nums[start];
                return t;
            }

            t.min_val = Float.MAX_VALUE;
            t.max_val = Float.MIN_VALUE;

            for (int i = start; i < end; i++) {
                T left = optimal(nums, start, i, "", memo);
                T right = optimal(nums, i + 1, end, "", memo);

                if (t.min_val > left.min_val / right.max_val) {
                    t.min_val = left.min_val / right.max_val;
                    t.min_str = left.min_str + "/" + (i + 1 != end ? "(" : "") + right.max_str
                            + (i + 1 != end ? ")" : "");
                }

                if (t.max_val < left.max_val / right.min_val) {
                    t.max_val = left.max_val / right.min_val;
                    t.max_str = left.max_str + "/" + (i + 1 != end ? "(" : "") + right.min_str
                            + (i + 1 != end ? ")" : "");
                }
            }
            memo[start][end] = t;
            return t;
        }
    }
}
