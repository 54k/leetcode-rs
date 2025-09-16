package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day1032 {
    // https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/?envType=daily-question&envId=2025-09-16
    static class Solution1 {
        static int lcm(int x, int y) {
            return x / gcd(x, y) * y;
        }

        static int gcd(int x, int y) {
            if (y == 0) {
                return x;
            }
            return gcd(y, x % y);
        }

        public List<Integer> replaceNonCoprimes(int[] nums) {
            Stack<Integer> stack = new Stack<>();
            for (int x : nums) {
                int cur = x;
                while (!stack.isEmpty()) {
                    int gcd = gcd(stack.peek(), cur);
                    if (gcd == 1)
                        break;
                    cur = lcm(stack.peek(), cur);
                    stack.pop();
                }
                stack.push(cur);
            }
            return new ArrayList<>(stack);
        }
    }

    // https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
    static class Solution2 {
        public int[] searchRange(int[] nums, int target) {
            int firstOccurence = this.findBound(nums, target, true);

            if (firstOccurence == -1) {
                return new int[] { -1, -1 };
            }

            int lastOccurence = this.findBound(nums, target, false);
            return new int[] { firstOccurence, lastOccurence };
        }

        int findBound(int[] nums, int target, boolean isFirst) {
            int N = nums.length;
            int begin = 0, end = N - 1;

            while (begin <= end) {
                int mid = (begin + end) / 2;

                if (nums[mid] == target) {
                    if (isFirst) {
                        if (mid == begin || nums[mid - 1] != target) {
                            return mid;
                        }
                        end = mid - 1;
                    } else {
                        if (mid == end || nums[mid + 1] != target) {
                            return mid;
                        }
                        begin = mid + 1;
                    }
                } else if (nums[mid] > target) {
                    end = mid - 1;
                } else {
                    begin = mid + 1;
                }
            }

            return -1;
        }
    }

}
