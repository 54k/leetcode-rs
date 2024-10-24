package data_structures_examples;

import java.util.*;

class TreeHashPerveev {
    // https://leetcode.com/problems/flip-equivalent-binary-trees/description/
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

    static class Solution {
        public boolean flipEquiv(TreeNode root1, TreeNode root2) {
            var hm = new HashMap<List<Integer>, Integer>();
            var dfs = new Object() {
                Integer apply(TreeNode node) {
                    if (node == null) {
                        return -1;
                    }
                    var hash = new ArrayList<Integer>();
                    hash.add(node.val);
                    hash.add(apply(node.left));
                    hash.add(apply(node.right));
                    Collections.sort(hash);
                    return hm.computeIfAbsent(hash, $ -> hm.size());
                }
            };
            return dfs.apply(root1) == dfs.apply(root2);
        }
    }
}