package data_structures_examples;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

// lol find a better problem for this
public class CentroidDecompose {
    // https://leetcode.com/problems/binary-tree-maximum-path-sum/description
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

    static class Solution {

        Set<TreeNode> removed = new HashSet<>();
        Map<TreeNode, Integer> size = new HashMap<>();
        Map<TreeNode, List<TreeNode>> adj = new HashMap<>();

        int n = 0;
        int ans = Integer.MIN_VALUE;

        int countSize(TreeNode root) {
            if (root == null) {
                return 0;
            }

            n++;
            int sz = 1;

            adj.putIfAbsent(root, new ArrayList<>());

            if (root.left != null) {
                adj.putIfAbsent(root.left, new ArrayList<>());
                adj.get(root.left).add(root);
                adj.get(root).add(root.left);
                sz += countSize(root.left);
            }

            if (root.right != null) {
                adj.putIfAbsent(root.right, new ArrayList<>());
                adj.get(root.right).add(root);
                adj.get(root).add(root.right);
                sz += countSize(root.right);
            }

            size.put(root, sz);
            return sz;
        }

        TreeNode findCentroid(TreeNode root) {
            if (root == null) {
                return root;
            }

            if (root.left != null && !removed.contains(root.left) && size.get(root.left) > n / 2) {
                return findCentroid(root.left);
            }
            if (root.right != null && !removed.contains(root.right) && size.get(root.right) > n / 2) {
                return findCentroid(root.right);
            }

            return root;
        }

        void decompose(TreeNode root) {
            TreeNode c = findCentroid(root);
            if (c == null) {
                return;
            }

            calc(c);
            removed.add(c);

            for (var u : adj.get(c)) {
                if (!removed.contains(u)) {
                    decompose(u);
                }
            }
        }

        void calc(TreeNode root) {
            var dfs = new Object() {
                int apply(TreeNode v, TreeNode p) {
                    if (v == null || removed.contains(v)) {
                        return 0;
                    }

                    var max1 = 0;
                    var max2 = 0;

                    for (var u : adj.get(v)) {
                        if (u != p) {
                            var res = apply(u, v);
                            if (res > max1) {
                                max2 = max1;
                                max1 = res;
                            } else if (res > max2) {
                                max2 = res;
                            }
                        }
                    }

                    ans = Math.max(ans, v.val + Math.max(max1, 0) + Math.max(max2, 0));
                    return v.val + Math.max(Math.max(max1, 0), Math.max(max2, 0));
                }
            };

            dfs.apply(root, null);
        }

        public int maxPathSum(TreeNode root) {
            countSize(root);
            decompose(root);
            return ans;
        }
    }
}
