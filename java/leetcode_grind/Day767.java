package leetcode_grind;

import java.util.PriorityQueue;
import java.util.Stack;

public class Day767 {
    // https://leetcode.com/problems/next-greater-element-ii/description/
    static class Solution1 {
        public int[] nextGreaterElements(int[] nums) {
            int[] res = new int[nums.length];
            Stack<Integer> stack = new Stack<>();
            for (int i = 2 * nums.length - 1; i >= 0; i--) {
                while (!stack.empty() && nums[stack.peek()] <= nums[i % nums.length]) {
                    stack.pop();
                }
                res[i % nums.length] = stack.empty() ? -1 : nums[stack.peek()];
                stack.push(i % nums.length);
            }
            return res;
        }
    }

    // https://leetcode.com/problems/maximal-range-that-each-element-is-maximum-in-it/description/
    static class Solution2 {
        public int[] maximumLengthOfRanges(int[] nums) {
            int n = nums.length;
            int[] left = new int[n];
            int[] right = new int[n];
            Stack<Integer> idxStack = new Stack<>();

            for (int currIdx = 0; currIdx < n; currIdx++) {
                while (!idxStack.empty() && nums[idxStack.peek()] < nums[currIdx]) {
                    idxStack.pop();
                }
                left[currIdx] = idxStack.empty() ? -1 : idxStack.peek();
                idxStack.push(currIdx);
            }

            idxStack.clear();

            for (int currIdx = n - 1; currIdx >= 0; currIdx--) {
                while (!idxStack.empty() && nums[idxStack.peek()] < nums[currIdx]) {
                    idxStack.pop();
                }
                right[currIdx] = idxStack.empty() ? n : idxStack.peek();
                idxStack.push(currIdx);
            }

            int[] ans = new int[n];
            for (int currIdx = 0; currIdx < n; currIdx++) {
                ans[currIdx] = right[currIdx] - left[currIdx] - 1;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/maximum-average-pass-ratio/description/
    static class Solution3 {
        public double maxAverageRatio(int[][] classes, int extraStudents) {
            PriorityQueue<double[]> maxHeap = new PriorityQueue<>((a, b) -> Double.compare(b[0], a[0]));

            for (int[] singleClass : classes) {
                int passes = singleClass[0];
                int totalStudents = singleClass[1];
                double gain = calculateGain(passes, totalStudents);
                maxHeap.offer(new double[] { gain, passes, totalStudents });
            }

            while (extraStudents-- > 0) {
                double[] current = maxHeap.poll();
                int passes = (int) current[1];
                int totalStudents = (int) current[2];
                maxHeap.offer(new double[] {
                        calculateGain(passes + 1, totalStudents + 1),
                        passes + 1,
                        totalStudents + 1,
                });
            }

            double totalPassRatio = 0;
            while (!maxHeap.isEmpty()) {
                double[] current = maxHeap.poll();
                int passes = (int) current[1];
                int totalStudents = (int) current[2];
                totalPassRatio += (double) passes / totalStudents;
            }
            return totalPassRatio / classes.length;
        }

        double calculateGain(int passes, int totalStudents) {
            return ((double) (passes + 1) / (totalStudents + 1) - (double) passes / totalStudents);
        }
    }
}
