package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day405 {

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

    static class Solution1 {
        public String getDirections(TreeNode root, int startValue, int destValue) {
            var lca = new Object() {
                TreeNode apply(TreeNode root, int v, int u) {
                    if (root == null || root.val == v || root.val == u) {
                        return root;
                    }

                    var left = apply(root.left, v, u);
                    var right = apply(root.right, v, u);

                    if (left != null && right != null) {
                        return root;
                    } else if (left != null) {
                        return left;
                    } else {
                        return right;
                    }
                }
            };

            var path = new Object() {
                boolean apply(TreeNode root, int val, List<String> path) {
                    if (root == null) {
                        return false;
                    }

                    if (root.val == val) {
                        return true;
                    }

                    path.add("L");
                    if (apply(root.left, val, path))
                        return true;
                    path.remove(path.size() - 1);

                    path.add("R");
                    if (apply(root.right, val, path))
                        return true;
                    path.remove(path.size() - 1);

                    return false;
                }
            };

            var ca = lca.apply(root, startValue, destValue);
            var pathV = new ArrayList<String>();
            path.apply(ca, startValue, pathV);

            var pathD = new ArrayList<String>();
            path.apply(ca, destValue, pathD);

            var ans = "";
            for (int i = 0; i < pathV.size(); ++i) {
                ans += "U";
            }
            for (var e : pathD) {
                ans += e;
            }
            return ans;
        }
    }

}
