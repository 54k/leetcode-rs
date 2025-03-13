package leetcode_grind;

public class Day845 {
    // https://leetcode.com/problems/zero-array-transformation-ii/description/?envType=daily-question&envId=2025-03-13
    static class Solution1 {
        public int minZeroArray(int[] nums, int[][] queries) {
            int left = 0, right = queries.length;
            if (!currentIndexZero(nums, queries, right))
                return -1;

            while (left <= right) {
                int middle = left + (right - left) / 2;
                if (currentIndexZero(nums, queries, middle)) {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            }

            return left;
        }

        boolean currentIndexZero(int[] nums, int[][] queries, int k) {
            int n = nums.length, sum = 0;
            int[] differenceArray = new int[n + 1];
            for (int queryIndex = 0; queryIndex < k; queryIndex++) {
                int left = queries[queryIndex][0], right = queries[queryIndex][1], val = queries[queryIndex][2];
                differenceArray[left] += val;
                differenceArray[right + 1] -= val;
            }

            for (int numIndex = 0; numIndex < n; numIndex++) {
                sum += differenceArray[numIndex];
                if (sum < nums[numIndex])
                    return false;
            }
            return true;
        }
    }

    static class Solution2 {
        public int minZeroArray(int[] nums, int[][] queries) {
            int n = nums.length;
            int[] diffArray = new int[n + 1];
            int sum = 0;
            int k = 0;

            for (int i = 0; i < n; i++) {
                while (sum + diffArray[i] < nums[i]) {
                    k++;
                    if (k > queries.length) {
                        return -1;
                    }
                    int left = queries[k - 1][0], right = queries[k - 1][1], val = queries[k - 1][2];

                    if (right >= i) {
                        diffArray[Math.max(i, left)] += val;
                        diffArray[right + 1] -= val;
                    }
                }
                sum += diffArray[i];
            }

            return k;
        }
    }

}
