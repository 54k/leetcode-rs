package leetcode_grind;

public class Day827 {
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

    static class Solution1 {
        int pre = 0;
        int post = 0;

        public TreeNode constructFromPrePost(int[] preorder, int[] postorder) {
            var root = new TreeNode(preorder[pre++]);
            if (root.val != postorder[post]) {
                root.left = constructFromPrePost(preorder, postorder);
            }
            if (root.val != postorder[post]) {
                root.right = constructFromPrePost(preorder, postorder);
            }
            post++;
            return root;
        }
    }

    static class Solution2 {
        public TreeNode constructFromPrePost(int[] preorder, int[] postorder) {
            int numOfNodes = preorder.length;
            return constructTree(0, numOfNodes - 1, 0, preorder, postorder);
        }

        TreeNode constructTree(int preStart, int preEnd, int postStart, int[] preorder, int[] postorder) {
            if (preStart > preEnd)
                return null;

            if (preStart == preEnd) {
                return new TreeNode(preorder[preStart]);
            }

            int leftRoot = preorder[preStart + 1];
            int numOfNodesInLeft = 1;
            while (postorder[postStart + numOfNodesInLeft - 1] != leftRoot) {
                numOfNodesInLeft++;
            }

            TreeNode root = new TreeNode(preorder[preStart]);
            root.left = constructTree(
                    preStart + 1,
                    preStart + numOfNodesInLeft,
                    postStart,
                    preorder,
                    postorder);

            root.right = constructTree(
                    preStart + numOfNodesInLeft + 1,
                    preEnd,
                    postStart + numOfNodesInLeft,
                    preorder,
                    postorder);
            return root;
        }
    }
}
