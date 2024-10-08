package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day335 {
    // https://leetcode.com/problems/min-cost-climbing-stairs/description
    static class Solution1 {
        public int minCostClimbingStairs(int[] cost) {
            var a = cost[0];
            var b = cost[1];
            for (var i = 2; i < cost.length; i++) {
                var c = Math.min(a + cost[i], b + cost[i]);
                a = b;
                b = c;
            }
            return Math.min(a, b);
        }
    }

    static class TreeNode {
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

    // https://leetcode.com/problems/binary-tree-preorder-traversal/description/
    static class Solution2 {
        public List<Integer> preorderTraversalIterative(TreeNode root) {
            var stack = new ArrayDeque<TreeNode>();
            var ans = new ArrayList<Integer>();
            if (root == null) {
                return ans;
            }
            stack.push(root);
            while (stack.size() > 0) {
                var pop = stack.pop();
                ans.add(pop.val);
                if (pop.right != null) {
                    stack.push(pop.right);
                }
                if (pop.left != null) {
                    stack.push(pop.left);
                }
            }
            return ans;
        }

        public List<Integer> preorderTraversalMorris(TreeNode root) {
            var ans = new ArrayList<Integer>();
            TreeNode curr = root;
            TreeNode last = null;
            while (curr != null) {
                if (curr.left == null) {
                    ans.add(curr.val);
                    curr = curr.right;
                } else {
                    last = curr.left;
                    while (last.right != null && last.right != curr) {
                        last = last.right;
                    }

                    if (last.right == null) {
                        ans.add(curr.val);
                        last.right = curr;
                        curr = curr.left;
                    } else {
                        last.right = null;
                        curr = curr.right;
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/binary-tree-inorder-traversal/description/
    static class Solution3 {
        public List<Integer> inorderTraversalIterative(TreeNode root) {
            var ans = new ArrayList<Integer>();
            var stack = new ArrayDeque<TreeNode>();
            var curr = root;
            while (stack.size() > 0 || curr != null) {
                while (curr != null) {
                    stack.push(curr);
                    curr = curr.left;
                }
                curr = stack.pop();
                ans.add(curr.val);
                curr = curr.right;
            }
            return ans;
        }

        public List<Integer> inorderTraversalMorris(TreeNode root) {
            var ans = new ArrayList<Integer>();
            TreeNode last;
            var curr = root;
            while (curr != null) {
                if (curr.left == null) {
                    ans.add(curr.val);
                    curr = curr.right;
                } else {
                    last = curr.left;
                    while (last.right != null && last.right != curr) {
                        last = last.right;
                    }

                    if (last.right == null) {
                        last.right = curr;
                        curr = curr.left;
                    } else {
                        ans.add(curr.val);
                        last.right = null;
                        curr = curr.right;
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together/description/
    static class Solution4 {
        public int minSwapsDeque(int[] data) {
            var ones = Arrays.stream(data).sum();
            var current = 0;
            var ans = Integer.MAX_VALUE;

            var deq = new ArrayDeque<Integer>();
            for (var num : data) {
                deq.addLast(num);
                current += num;
                if (deq.size() > ones) {
                    current -= deq.pollFirst();
                }
                ans = Math.min(ans, ones - current);
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/description/
    class Solution5 {
        public int secondsToRemoveOccurrences(String s) {
            var zeros = 0;
            var seconds = 0;
            for (var i = 0; i < s.length(); i++) {
                zeros += s.charAt(i) == '0' ? 1 : 0;
                if (s.charAt(i) == '1' && zeros > 0) {
                    seconds = Math.max(seconds + 1, zeros);
                }
            }
            return seconds;
        }
    }
}
