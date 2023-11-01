package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Stack;

public class Day354 {
    public class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    // https://leetcode.com/problems/find-mode-in-binary-search-tree/description
    static class Solution1 {
        public int[] findModeDFSRecursive(TreeNode root) {
            var freq = new HashMap<Integer, Integer>();
            var maxFreq = new int[] { 0 };

            var dfs = new Object() {
                void go(TreeNode root) {
                    if (root == null) {
                        return;
                    }

                    freq.put(root.val, freq.getOrDefault(root.val, 0) + 1);
                    maxFreq[0] = Math.max(freq.get(root.val), maxFreq[0]);

                    go(root.left);
                    go(root.right);
                }
            };

            dfs.go(root);

            List<Integer> result = new ArrayList<Integer>();
            for (var e : freq.entrySet()) {
                if (e.getValue() == maxFreq[0]) {
                    result.add(e.getKey());
                }
            }
            return result.stream().mapToInt(Integer::intValue).toArray();
        }

        public int[] findModeDFSIterative(TreeNode root) {
            var freq = new HashMap<Integer, Integer>();
            var maxFreq = 0;
            var stack = new Stack<TreeNode>();
            stack.push(root);

            while (!stack.isEmpty()) {
                var top = stack.pop();
                if (top == null) {
                    continue;
                }
                freq.put(top.val, freq.getOrDefault(top.val, 0) + 1);
                maxFreq = Math.max(freq.get(top.val), maxFreq);
                stack.push(top.right);
                stack.push(top.left);
            }

            List<Integer> result = new ArrayList<Integer>();
            for (var e : freq.entrySet()) {
                if (e.getValue() == maxFreq) {
                    result.add(e.getKey());
                }
            }
            return result.stream().mapToInt(Integer::intValue).toArray();
        }

        public int[] findModeInorder(TreeNode root) {
            var ans = new ArrayList<Integer>();

            var inorder = new Object() {
                int currNum = 0;
                int currStreak = 0;
                int maxStreak = 0;

                void apply(TreeNode root) {
                    if (root == null) {
                        return;
                    }
                    apply(root.left);

                    if (currNum == root.val) {
                        currStreak++;
                    } else {
                        currStreak = 1;
                    }

                    if (currStreak > maxStreak) {
                        ans.clear();
                        maxStreak = currStreak;
                    }
                    if (currStreak == maxStreak) {
                        ans.add(root.val);
                    }

                    currNum = root.val;

                    apply(root.right);
                }
            };

            inorder.apply(root);
            return ans.stream().mapToInt(Integer::intValue).toArray();
        }

        public int[] findModeMorrisTraversal(TreeNode root) {
            var res = new ArrayList<Integer>();

            var curNum = 0;
            var curStreak = 0;
            var maxStreak = 0;

            var curNode = root;
            while (curNode != null) {
                if (curNode.left == null) {
                    var val = curNode.val;

                    if (curNum == val) {
                        curStreak++;
                    } else {
                        curStreak = 1;
                        curNum = val;
                    }

                    if (curStreak > maxStreak) {
                        res.clear();
                        maxStreak = curStreak;
                    }

                    if (curStreak == maxStreak) {
                        res.add(val);
                    }

                    curNode = curNode.right;
                } else {
                    var next = curNode.left;
                    while (next.right != null) {
                        next = next.right;
                    }

                    next.right = curNode;
                    var tmp = curNode;
                    curNode = curNode.left;
                    tmp.left = null;
                }
            }

            return res.stream().mapToInt(Integer::intValue).toArray();
        }
    }

    // https://leetcode.com/problems/diagonal-traverse-ii/description/
    static class Solution2 {
        public int[] findDiagonalOrder(List<List<Integer>> nums) {
            var ans = new ArrayList<Integer>();

            var queue = new LinkedList<int[]>();
            queue.offer(new int[] { 0, 0 });

            while (!queue.isEmpty()) {
                var top = queue.poll();
                var row = top[0];
                var col = top[1];

                ans.add(nums.get(row).get(col));

                if (col == 0 && row + 1 < nums.size()) {
                    queue.offer(new int[] { row + 1, col });
                }

                if (col + 1 < nums.get(row).size()) {
                    queue.offer(new int[] { row, col + 1 });
                }
            }

            return ans.stream().mapToInt(Integer::intValue).toArray();
        }
    }

    // https://leetcode.com/problems/maximum-subarray-min-product/description/
    static class Solution3 {
        public int maxSumMinProduct(int[] nums) {
            var prefix = new long[nums.length + 1];
            var mono = new Stack<Integer>();

            long ans = 0;
            for (int i = 0; i < nums.length; i++) {
                prefix[i + 1] = prefix[i] + nums[i];
            }

            for (int i = 0; i <= nums.length; i++) {
                while (!mono.isEmpty() && (i == nums.length || nums[mono.peek()] > nums[i])) {
                    var min = mono.pop();
                    var left = mono.isEmpty() ? 0 : mono.peek() + 1;
                    var prod = (prefix[i] - prefix[left]) * nums[min];
                    ans = Math.max(ans, prod);
                }
                if (i < nums.length) {
                    mono.add(i);
                }
            }

            return (int) (ans % 1_000_000_007);
        }
    }

    // https://leetcode.com/problems/edit-distance/description/
    static class Solution4 {
        public int minDistance(String word1, String word2) {
            var word1Len = word1.length();
            var word2Len = word2.length();

            var dp = new int[word1Len + 1][word2Len + 1];
            for (var i = 1; i <= word1Len; i++) {
                dp[i][0] = dp[i - 1][0] + 1;
            }
            for (var i = 1; i <= word2Len; i++) {
                dp[0][i] = dp[0][i - 1] + 1;
            }

            for (var w1 = 1; w1 <= word1Len; w1++) {
                for (var w2 = 1; w2 <= word2Len; w2++) {
                    if (word1.charAt(w1 - 1) == word2.charAt(w2 - 1)) {
                        dp[w1][w2] = dp[w1 - 1][w2 - 1];
                    } else {
                        dp[w1][w2] = Math.min(
                                dp[w1 - 1][w2 - 1],
                                Math.min(dp[w1 - 1][w2], dp[w1][w2 - 1])) + 1;
                    }
                }
            }

            return dp[word1Len][word2Len];
        }
    }
}
