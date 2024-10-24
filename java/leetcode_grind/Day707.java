package leetcode_grind;

import java.util.Stack;

public class Day707 {
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

    // https://leetcode.com/problems/flip-equivalent-binary-trees/description/
    static class Solution1 {
        public boolean flipEquiv(TreeNode root1, TreeNode root2) {
            var dfs = new Object() {
                boolean apply(TreeNode n1, TreeNode n2) {
                    if (n1 == null && n2 == null) {
                        return true;
                    }
                    if (n1 == null || n2 == null) {
                        return false;
                    }
                    if (n1.val != n2.val) {
                        return false;
                    }
                    boolean noSwap = apply(n1.left, n2.left) && apply(n1.right, n2.right);
                    boolean swap = apply(n1.left, n2.right) && apply(n1.right, n2.left);
                    return noSwap || swap;
                }
            };
            return dfs.apply(root1, root2);
        }
    }

    static class Solution2 {
        boolean checkNodeValues(TreeNode node1, TreeNode node2) {
            if (node1 == null && node2 == null)
                return true;
            if (node1 != null && node2 != null && node1.val == node2.val) {
                return true;
            }
            return false;
        }

        public boolean flipEquiv(TreeNode root1, TreeNode root2) {
            Stack<TreeNode[]> nodePairStack = new Stack<>();
            nodePairStack.push(new TreeNode[] { root1, root2 });
            while (!nodePairStack.isEmpty()) {
                TreeNode[] current = nodePairStack.pop();
                TreeNode node1 = current[0];
                TreeNode node2 = current[1];

                if (node1 == null && node2 == null)
                    continue;
                if (node1 == null || node2 == null)
                    return false;
                if (node1.val != node2.val)
                    return false;

                if (checkNodeValues(node1.left, node2.left) && checkNodeValues(node1.right, node2.right)) {
                    nodePairStack.push(new TreeNode[] { node1.left, node2.left });
                    nodePairStack.push(new TreeNode[] { node1.right, node2.right });
                } else if (checkNodeValues(node1.left, node2.right) && checkNodeValues(node1.right, node2.left)) {
                    nodePairStack.push(new TreeNode[] { node1.left, node2.right });
                    nodePairStack.push(new TreeNode[] { node1.right, node2.left });
                } else {
                    return false;
                }
            }
            return true;
        }
    }

    static class Solution3 {
        void findCanonicalForm(TreeNode root) {
            if (root == null)
                return;
            findCanonicalForm(root.left);
            findCanonicalForm(root.right);

            if (root.right == null)
                return;

            if (root.left == null) {
                root.left = root.right;
                root.right = null;
                return;
            }

            TreeNode left = root.left;
            TreeNode right = root.right;
            if (left.val > right.val) {
                root.left = right;
                root.right = left;
            }
        }

        boolean areEquivalent(TreeNode root1, TreeNode root2) {
            if (root1 == null && root2 == null)
                return true;
            if (root1 == null || root2 == null)
                return false;
            if (root1.val != root2.val)
                return false;
            return (areEquivalent(root1.left, root2.left) &&
                    areEquivalent(root1.right, root2.right));
        }

        public boolean flipEquiv(TreeNode root1, TreeNode root2) {
            findCanonicalForm(root1);
            findCanonicalForm(root2);
            return areEquivalent(root1, root2);
        }
    }
}
