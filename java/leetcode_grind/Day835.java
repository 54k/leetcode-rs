package leetcode_grind;

import java.util.LinkedList;

public class Day835 {
    // https://leetcode.com/problems/partition-array-according-to-given-pivot/description/?envType=daily-question&envId=2025-03-03
    static class Solution1 {
        public int[] pivotArray(int[] nums, int pivot) {
            LinkedList<Integer> less = new LinkedList<>();
            LinkedList<Integer> equal = new LinkedList<>();
            LinkedList<Integer> greater = new LinkedList<>();

            for (int num : nums) {
                if (num < pivot) {
                    less.add(num);
                } else if (num > pivot) {
                    greater.add(num);
                } else {
                    equal.add(num);
                }
            }

            less.addAll(equal);
            less.addAll(greater);

            int i = 0;
            int[] ans = new int[nums.length];
            for (int num : less) {
                ans[i++] = num;
            }
            return ans;
        }
    }

    static class Solution2 {
        public int[] pivotArray(int[] nums, int pivot) {
            int less = 0;
            int equal = 0;
            for (int num : nums) {
                if (num < pivot)
                    less++;
                else if (num == pivot)
                    equal++;
            }

            int[] ans = new int[nums.length];
            int lessI = 0;
            int equalI = less;
            int greaterI = less + equal;

            for (int i = 0; i < nums.length; i++) {
                int num = nums[i];
                if (num < pivot) {
                    ans[lessI] = num;
                    lessI++;
                } else if (num > pivot) {
                    ans[greaterI] = num;
                    greaterI++;
                } else {
                    ans[equalI] = num;
                    equalI++;
                }
            }
            return ans;
        }
    }

    static class Solution3 {
        public int[] pivotArray(int[] nums, int pivot) {
            int[] ans = new int[nums.length];
            int lessI = 0;
            int greaterI = nums.length - 1;

            for (int i = 0, j = nums.length - 1; i < nums.length; i++, j--) {
                if (nums[i] < pivot) {
                    ans[lessI] = nums[i];
                    lessI++;
                }
                if (nums[j] > pivot) {
                    ans[greaterI] = nums[j];
                    greaterI--;
                }
            }
            while (lessI <= greaterI) {
                ans[lessI] = pivot;
                lessI++;
            }
            return ans;
        }
    }
}
