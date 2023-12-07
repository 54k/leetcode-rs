package leetcode_grind;

public class Day390 {
    // https://leetcode.com/problems/count-of-smaller-numbers-after-self/
    static class Solution {
        public List<Integer> countSmaller(int[] nums) {
            int n = nums.length;
            int[] result = new int[n];
            int[] indices = new int[n];
            for (int i = 0; i < n; i++) {
                indices[i] = i;
            }

            mergeSort(indices, 0, n, result, nums);
            List<Integer> resultToReturn = new ArrayList<>(n);
            for (int i : result) {
                resultToReturn.add(i);
            }
            return resultToReturn;
        }

        void mergeSort(int[] indices, int left, int right, int[] result, int[] nums) {
            if (right - left <= 1) {
                return;
            }
            int mid = (left + right) / 2;
            mergeSort(indices, left, mid, result, nums);
            mergeSort(indices, mid, right, result, nums);
            merge(indices, left, right, mid, result, nums);
        }

        void merge(int[] indices, int left, int right, int mid, int[] result, int[] nums) {
            int i = left;
            int j = mid;
            List<Integer> temp = new ArrayList<>(right - left);

            while (i < mid && j < right) {
                if (nums[indices[i]] <= nums[indices[j]]) {
                    result[indices[i]] += j - mid;
                    temp.add(indices[i]);
                    i++;
                } else {
                    temp.add(indices[j]);
                    j++;
                }
            }

            while (i < mid) {
                result[indices[i]] += j - mid;
                temp.add(indices[i]);
                i++;
            }

            while (j < right) {
                temp.add(indices[j]);
                j++;
            }

            for (int k = left; k < right; k++) {
                indices[k] = temp.get(k - left);
            }
        }
    }
}
