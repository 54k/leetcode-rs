package leetcode_grind;

public class Day607 {
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

    // https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/description/?envType=daily-question&envId=2024-07-16
    static class Solution1 {
        public String getDirections(TreeNode root, int startValue, int destValue) {
            TreeNode lowestCommonAncestor = findLowestCommonAncestor(root, startValue, destValue);
            StringBuilder pathToStart = new StringBuilder();
            StringBuilder pathToDest = new StringBuilder();

            findPath(lowestCommonAncestor, startValue, pathToStart);
            findPath(lowestCommonAncestor, destValue, pathToDest);

            StringBuilder directions = new StringBuilder();

            directions.append("U".repeat(pathToStart.length()));
            directions.append(pathToDest);
            return directions.toString();
        }

        TreeNode findLowestCommonAncestor(TreeNode node, int value1, int value2) {
            if (node == null)
                return null;
            if (node.val == value1 || node.val == value2)
                return node;
            TreeNode leftLCA = findLowestCommonAncestor(node.left, value1, value2);
            TreeNode rightLCA = findLowestCommonAncestor(node.right, value1, value2);
            if (leftLCA == null)
                return rightLCA;
            else if (rightLCA == null)
                return leftLCA;
            else
                return node;
        }

        boolean findPath(TreeNode node, int targetValue, StringBuilder path) {
            if (node == null)
                return false;
            if (node.val == targetValue)
                return true;
            path.append("L");
            if (findPath(node.left, targetValue, path)) {
                return true;
            }
            path.setLength(path.length() - 1);
            path.append("R");
            if (findPath(node.right, targetValue, path)) {
                return true;
            }
            path.setLength(path.length() - 1);
            return false;
        }
    }

}
