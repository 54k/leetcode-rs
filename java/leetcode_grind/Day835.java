package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.NoSuchElementException;

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

    // https://leetcode.com/problems/flatten-2d-vector/description/
    static class Vector2D_1 {
        List<Integer> nums = new ArrayList<>();
        int position = 0;

        public Vector2D_1(int[][] vec) {
            for (int[] innerVector : vec) {
                for (int num : innerVector) {
                    nums.add(num);
                }
            }
        }

        public int next() {
            if (!hasNext())
                throw new NoSuchElementException();
            int result = nums.get(position);
            position++;
            return result;
        }

        public boolean hasNext() {
            return position < nums.size();
        }
    }

    static class Vector2D_2 {
        int[][] vector;
        int inner = 0;
        int outer = 0;

        public Vector2D_2(int[][] vec) {
            vector = vec;
        }

        void advanceToNext() {
            while (outer < vector.length && inner == vector[outer].length) {
                inner = 0;
                outer++;
            }
        }

        public int next() {
            if (!hasNext())
                throw new NoSuchElementException();
            return vector[outer][inner++];
        }

        public boolean hasNext() {
            advanceToNext();
            return outer < vector.length;
        }
    }

}
