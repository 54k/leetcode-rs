package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Stack;

public class Day347 {
    static class Solution1 {
        // https://leetcode.com/problems/k-th-symbol-in-grammar/description
        public int kthGrammarTreeBinSearch(int n, int k) {
            var dfs = new Object() {
                int apply(int n, int k, int rootVal) {
                    if (n == 1) {
                        return rootVal;
                    }
                    var totalNodes = (int) Math.pow(2, n - 1);
                    if (k <= totalNodes / 2) {
                        var nextRootVal = rootVal == 0 ? 0 : 1;
                        return apply(n - 1, k, nextRootVal);
                    } else {
                        var nextRootVal = rootVal == 0 ? 1 : 0;
                        return apply(n - 1, k - (totalNodes / 2), nextRootVal);
                    }
                }
            };
            return dfs.apply(n, k, 0);
        }
    }

    // https://leetcode.com/problems/minimum-sum-of-mountain-triplets-i/
    static class Solution2 {
        public int minimumSum(int[] nums) {
            var ans = Integer.MAX_VALUE;
            for (int i = 0; i < nums.length - 2; i++) {
                for (int j = i + 1; j < nums.length - 1; j++) {
                    for (int k = j + 1; k < nums.length; k++) {
                        if (nums[i] < nums[j] && nums[j] > nums[k]) {
                            var sum = nums[i] + nums[k] + nums[j];
                            ans = Math.min(sum, ans);
                        }
                    }
                }
            }
            if (ans == Integer.MAX_VALUE) {
                return -1;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/minimum-sum-of-mountain-triplets-ii/
    static class Solution3 {
        public int minimumSum1Wrong(int[] nums) {
            var left = new Stack<Integer>();
            var leftMin = new int[nums.length];
            Arrays.fill(leftMin, -1);
            for (int i = nums.length - 1; i >= 0; i--) {
                while (!left.isEmpty() && nums[left.peek()] > nums[i]) {
                    leftMin[left.pop()] = i;
                }
                left.push(i);
            }

            // System.out.println(Arrays.toString(leftMin));

            var right = new Stack<Integer>();
            var rightMin = new int[nums.length];
            Arrays.fill(rightMin, nums.length);
            for (int i = 0; i < rightMin.length; i++) {
                if (!right.isEmpty() && nums[right.peek()] > nums[i]) {
                    rightMin[right.pop()] = i;
                }
                right.push(i);
            }

            // System.out.println(Arrays.toString(rightMin));

            var ans = Integer.MAX_VALUE;
            for (int i = 0; i < nums.length; i++) {
                if (leftMin[i] > -1 && rightMin[i] < nums.length) {
                    var sum = nums[leftMin[i]] + nums[rightMin[i]] + nums[i];
                    // System.out.println(sum);
                    ans = Math.max(ans, sum);
                }
            }
            return ans == Integer.MAX_VALUE ? -1 : ans;
        }

        public int minimumSum2(int[] nums) {
            var n = nums.length;
            var curr = nums[0];

            var leftMin = new int[n];
            for (int i = 0; i < n; i++) {
                curr = Math.min(nums[i], curr);
                leftMin[i] = curr;
            }

            // System.out.println(Arrays.toString(leftMin));

            curr = nums[n - 1];
            var rightMin = new int[n];
            for (int i = n - 1; i >= 0; i--) {
                curr = Math.min(nums[i], curr);
                rightMin[i] = curr;
            }

            // System.out.println(Arrays.toString(rightMin));

            var ans = Integer.MAX_VALUE;
            for (int i = 0; i < n; i++) {
                if (leftMin[i] != nums[i] && rightMin[i] != nums[i]) {
                    var sum = leftMin[i] + rightMin[i] + nums[i];
                    // System.out.println(sum);
                    ans = Math.min(ans, sum);
                }
            }
            return ans == Integer.MAX_VALUE ? -1 : ans;
        }
    }

    // https://leetcode.com/problems/minimum-number-of-groups-to-create-a-valid-assignment/description/
    static class Solution4 {
        public int minGroupsForValidAssignment(int[] nums) {
            var freq = new HashMap<Integer, Integer>();
            for (var num : nums) {
                freq.put(num, freq.getOrDefault(num, 0) + 1);
            }

            var minFreq = Integer.MAX_VALUE;
            for (var f : freq.values()) {
                minFreq = Math.min(minFreq, f);
            }

            var groupCount = Integer.MAX_VALUE;
            for (int x = 1; x <= minFreq; x++) {
                var total = 0;
                var possible = true;
                for (var f : freq.values()) {
                    // To get the minimum number of groups needed for a number having frequency f to be assigned to groups of size x or x + 1, let a = f / (x + 1) and b = f % (x + 1).
                    // If b == 0, then we can simply create a groups of size x + 1.
                    // If x - b <= a, we can have a - (x - b) groups of size x + 1 and x - b + 1 groups of size x. So, in total, we have a + 1 groups.
                    // Otherwise, it's impossible.
                    var a = f / (x + 1);
                    var b = f % (x + 1);
                    if (b == 0) {
                        total += a;
                    } else if (x - b <= a) {
                        total += a + 1;
                    } else {
                        possible = false;
                        break;
                    }
                }
                if (possible) {
                    groupCount = Math.min(groupCount, total);
                }
            }
            return groupCount;
        }
    }
}
