import java.util.*;

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode(int x) {
        val = x;
    }
}

// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree-iv/description
public class Day283 {
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode[] nodes) {
        Set<TreeNode> children = new HashSet<>();
        for (var n : nodes) {
            children.add(n);
        }
        return find(root, children);
    }

    TreeNode find(TreeNode root, Set<TreeNode> nodes) {
        if (nodes.contains(root) || root == null) {
            return root;
        }

        var left = find(root.left, nodes);
        var right = find(root.right, nodes);

        if (left == null) {
            return right;
        } else if (right == null) {
            return left;
        }

        return root;
    }
}
