package data_structures_examples;

import java.util.Random;

public class QuickSort {
    // https://leetcode.com/problems/sort-an-array/description/
    static class Solution {
        void quickSort(int[] nums, int l, int r) {
            if (l >= r) {
                return;
            }

            int mid = partition2(nums, l, r);
            quickSort(nums, l, mid - 1);
            quickSort(nums, mid + 1, r);
        }

        // lomuto partition
        int partition1(int[] nums, int l, int r) {
            int pivot = new Random().nextInt(nums.length) % (r - l) + l;
            swap(nums, pivot, r);
            pivot = r;

            int j = l;
            for (int i = l; i <= r; ++i) {
                if (nums[i] < nums[pivot]) {
                    swap(nums, i, j);
                    j++;
                }
            }
            swap(nums, j, r);
            return j;
        }

        // hoare partition
        int partition2(int[] nums, int l, int r) {
            int p = r;
            while (l < r) {
                while (l < r && nums[l] <= nums[p]) {
                    l++;
                }
                while (l < r && nums[r] >= nums[p]) {
                    r--;
                }
                swap(nums, l, r);
            }

            swap(nums, r, p);
            return r;
        }

        void swap(int[] nums, int i, int j) {
            int t = nums[i];
            nums[i] = nums[j];
            nums[j] = t;
        }

        public int[] sortArray(int[] nums) {
            quickSort(nums, 0, nums.length - 1);
            return nums;
        }
    }
}
