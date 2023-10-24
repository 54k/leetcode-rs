package top_interview_150;

// https://leetcode.com/studyplan/top-interview-150/
public class ArrayAndString {
    // https://leetcode.com/problems/merge-sorted-array/description
    static class Solution1 {
        public void merge3PointersBackward(int[] nums1, int m, int[] nums2, int n) {
            // 3 pointers forward
            int p1 = m - 1;
            int p2 = n - 1;
            for (int p = m + n - 1; p >= 0; p--) {
                if (p2 < 0) {
                    break;
                }
                if (p1 >= 0 && nums1[p1] > nums2[p2]) {
                    nums1[p] = nums1[p1--];
                } else {
                    nums1[p] = nums2[p2--];
                }
            }
        }
    }

    // https://leetcode.com/problems/remove-element/description
    static class Solution2 {
        public int removeElement1(int[] nums, int val) {
            int left = 0;
            for (int right = 0; right < nums.length; right++) {
                if (nums[right] != val) {
                    nums[left++] = nums[right];
                }
            }
            return left++;
        }

        public int removeElement2(int[] nums, int val) {
            int i = 0;
            int n = nums.length;
            while (i < n) {
                if (nums[i] == val) {
                    nums[i] = nums[n - 1];
                    n--;
                } else {
                    i++;
                }
            }
            return i;
        }
    }

    // https://leetcode.com/problems/remove-duplicates-from-sorted-array
    static class Solution {
        public int removeDuplicates1(int[] nums) {
            int left = 0;
            for (int right = 0; right < nums.length; right++) {
                if (nums[left] != nums[right]) {
                    nums[++left] = nums[right];
                }
            }
            return ++left;
        }

        public int removeDuplicates2(int[] nums) {
            int insertIndex = 1;
            for (int i = 1; i < nums.length; i++) {
                if (nums[i - 1] != nums[i]) {
                    nums[insertIndex++] = nums[i];
                }
            }
            return insertIndex;
        }
    }
}
