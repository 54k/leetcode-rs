package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class Day1144 {
    public static class TreeNode {
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

    // https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/description/?envType=daily-question&envId=2026-01-06
    static class Solution1 {
        public int maxLevelSum(TreeNode root) {
            int maxSum = Integer.MIN_VALUE;
            int ans = 0;
            int level = 0;

            Queue<TreeNode> q = new LinkedList<>();
            q.offer(root);

            while (!q.isEmpty()) {
                level++;
                int sumAtCurrentLevel = 0;
                for (int sz = q.size(); sz > 0; --sz) {
                    TreeNode node = q.poll();
                    sumAtCurrentLevel += node.val;

                    if (node.left != null) {
                        q.offer(node.left);
                    }

                    if (node.right != null) {
                        q.offer(node.right);
                    }
                }

                if (maxSum < sumAtCurrentLevel) {
                    maxSum = sumAtCurrentLevel;
                    ans = level;
                }
            }

            return ans;
        }
    }

    static class Solution2 {
        public void dfs(TreeNode node, int level, List<Integer> sumOfNodesAtLevel) {
            if (node == null) {
                return;
            }

            if (sumOfNodesAtLevel.size() == level) {
                sumOfNodesAtLevel.add(node.val);
            } else {
                sumOfNodesAtLevel.set(level, sumOfNodesAtLevel.get(level) + node.val);
            }

            dfs(node.left, level + 1, sumOfNodesAtLevel);
            dfs(node.right, level + 1, sumOfNodesAtLevel);
        }

        public int maxLevelSum(TreeNode root) {
            List<Integer> sumOfNodesAtLevel = new ArrayList<>();
            dfs(root, 0, sumOfNodesAtLevel);

            int maxSum = Integer.MIN_VALUE;
            int ans = 0;
            for (int i = 0; i < sumOfNodesAtLevel.size(); i++) {
                if (maxSum < sumOfNodesAtLevel.get(i)) {
                    maxSum = sumOfNodesAtLevel.get(i);
                    ans = i + 1;
                }
            }
            return ans;
        }
    }

}
