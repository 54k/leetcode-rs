package leetcode_grind;

public class Day616 {
    // https://leetcode.com/problems/sort-an-array/description/?envType=daily-question&envId=2024-07-25
    static class Solution1 {
        public int[] sortArray(int[] nums) {
            var n = nums.length;
            var ms = new Object() {
                int[] tmp = new int[n];

                void apply(int[] nums, int l, int r) {
                    if (l + 1 == r) {
                        return;
                    }
                    int mid = (l + r) / 2;
                    apply(nums, l, mid);
                    apply(nums, mid, r);
                    merge(nums, l, mid, r);
                }

                void merge(int[] nums, int l, int m, int r) {
                    int k = l;
                    int i = l;
                    int j = m;
                    while (i < m && j < r) {
                        if (nums[i] <= nums[j]) {
                            tmp[k++] = nums[i++];
                        } else {
                            tmp[k++] = nums[j++];
                        }
                    }
                    while (i < m) {
                        tmp[k++] = nums[i++];
                    }
                    while (j < r) {
                        tmp[k++] = nums[j++];
                    }
                    while (l < r) {
                        nums[l] = tmp[l];
                        l++;
                    }
                }
            };
            ms.apply(nums, 0, n);
            return nums;
        }
    }
}
