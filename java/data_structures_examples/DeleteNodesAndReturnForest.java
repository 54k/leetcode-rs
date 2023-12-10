package data_structures_examples;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

public class DeleteNodesAndReturnForest {
    static public class TreeNode {
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

    // https://leetcode.com/problems/delete-nodes-and-return-forest/
    static class Solution {
        public List<TreeNode> delNodes1(TreeNode root, int[] to_delete) {
            var toDelete = new HashSet<Integer>();
            for (var d : to_delete) {
                toDelete.add(d);
            }

            var ans = new ArrayList<TreeNode>();
            var dfs = new Object() {
                TreeNode apply(TreeNode root, TreeNode parent) {
                    if (root == null) {
                        return root;
                    }

                    root.left = apply(root.left, root);
                    root.right = apply(root.right, root);

                    if (toDelete.contains(root.val)) {
                        if (root.left != null) {
                            ans.add(root.left);
                        }
                        if (root.right != null) {
                            ans.add(root.right);
                        }
                        return null;
                    } else if (root == parent) {
                        ans.add(root);
                    }

                    return root;
                }
            };

            dfs.apply(root, root);
            return ans;
        }

        public List<TreeNode> delNodes2(TreeNode root, int[] to_delete) {
            var toDelete = new HashSet<Integer>();
            for (var d : to_delete) {
                toDelete.add(d);
            }

            var ans = new ArrayList<TreeNode>();
            var dfs = new Object() {
                TreeNode apply(TreeNode root, boolean isRoot) {
                    if (root == null) {
                        return root;
                    }

                    var deleted = toDelete.contains(root.val);
                    if (isRoot && !deleted)
                        ans.add(root);
                    root.left = apply(root.left, deleted);
                    root.right = apply(root.right, deleted);

                    return deleted ? null : root;
                }
            };

            dfs.apply(root, true);
            return ans;
        }
    }
}
