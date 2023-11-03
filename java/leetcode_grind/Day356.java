package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Stack;

public class Day356 {
    // https://leetcode.com/problems/build-an-array-with-stack-operations/description/
    static class Solution1 {
        public List<String> buildArray1(int[] target, int n) {
            var tptr = 0;
            var stack = new ArrayList<String>();
            for (int i = 1; i <= n && tptr < target.length; i++) {
                stack.add("Push");
                if (i != target[tptr]) {
                    stack.add("Pop");
                } else {
                    tptr++;
                }
            }
            return stack;
        }

        public List<String> buildArray2(int[] target, int n) {
            var ans = new ArrayList<String>();
            var i = 1;

            for (var num : target) {
                while (i < num) {
                    ans.add("Push");
                    ans.add("Pop");
                    i++;
                }
                ans.add("Push");
                i++;
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/minimum-cost-for-tickets/description/
    static class Solution2 {
        public int mincostTicketsTopDown(int[] days, int[] costs) {
            var lastDay = days[days.length - 1];
            var memo = new int[lastDay + 1];
            Arrays.fill(memo, -1);
            var isTravelNeeded = new HashSet<Integer>();

            for (var day : days) {
                isTravelNeeded.add(day);
            }

            var solve = new Object() {
                int apply(int currDay) {
                    if (currDay > days[days.length - 1]) {
                        return 0;
                    }

                    if (!isTravelNeeded.contains(currDay)) {
                        return apply(currDay + 1);
                    }

                    if (memo[currDay] != -1) {
                        return memo[currDay];
                    }

                    var oneDay = costs[0] + apply(currDay + 1);
                    var sevenDay = costs[1] + apply(currDay + 7);
                    var thirtyDay = costs[2] + apply(currDay + 30);

                    return memo[currDay] = Math.min(oneDay, Math.min(sevenDay, thirtyDay));
                }
            };

            return solve.apply(1);
        }

        public int mincostTicketsBottomUp(int[] days, int[] costs) {
            var lastDay = days[days.length - 1];
            var dp = new int[lastDay + 1];
            var i = 0;
            for (int day = 1; day <= lastDay; day++) {
                if (day < days[i]) {
                    dp[day] = dp[day - 1];
                } else {
                    i++;
                    dp[day] = Math.min(dp[day - 1] + costs[0],
                            Math.min(dp[Math.max(0, day - 7)] + costs[1],
                                    dp[Math.max(0, day - 30)] + costs[2]));
                }
            }

            return dp[lastDay];
        }
    }

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

    // https://leetcode.com/problems/add-one-row-to-tree/description/
    class Solution3 {
        public TreeNode addOneRow(TreeNode root, int val, int depth) {
            class Node {
                TreeNode node;
                int depth;

                Node(TreeNode n, int d) {
                    node = n;
                    depth = d;
                }
            }

            if (depth == 1) {
                return new TreeNode(val, root, null);
            }

            var stack = new Stack<Node>();
            stack.push(new Node(root, 1));

            while (!stack.isEmpty()) {
                var top = stack.pop();
                if (top.node == null) {
                    continue;
                }

                if (top.depth == depth - 1) {
                    var t = top.node.left;
                    top.node.left = new TreeNode(val, t, null);
                    t = top.node.right;
                    top.node.right = new TreeNode(val, null, t);
                } else {
                    stack.push(new Node(top.node.left, top.depth + 1));
                    stack.push(new Node(top.node.right, top.depth + 1));
                }
            }

            return root;
        }
    }
}
