package leetcode_grind;

import java.util.*;;

public class Day647 {
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

    // https://leetcode.com/problems/binary-tree-postorder-traversal/description/?envType=daily-question&envId=2024-08-25
    static class Solution1 {
        public List<Integer> postorderTraversal(TreeNode root) {
            List<Integer> result = new ArrayList<>();
            Deque<TreeNode> traversalStack = new ArrayDeque<>();
            TreeNode currentNode = root;
            while (currentNode != null || !traversalStack.isEmpty()) {
                if (currentNode != null) {
                    result.add(currentNode.val);
                    traversalStack.push(currentNode);
                    currentNode = currentNode.right;
                } else {
                    currentNode = traversalStack.pop();
                    currentNode = currentNode.left;
                }
            }
            Collections.reverse(result);
            return result;
        }
    }

    static class Solution2 {
        public List<Integer> postorderTraversal(TreeNode root) {
            List<Integer> result = new ArrayList<>();
            if (root == null) {
                return result;
            }

            Stack<TreeNode> mainStack = new Stack<>();
            Stack<TreeNode> pathStack = new Stack<>();
            mainStack.push(root);

            while (!mainStack.isEmpty()) {
                root = mainStack.peek();
                if (!pathStack.isEmpty() && pathStack.peek() == root) {
                    result.add(root.val);
                    mainStack.pop();
                    pathStack.pop();
                } else {
                    pathStack.push(root);
                    if (root.right != null) {
                        mainStack.push(root.right);
                    }
                    if (root.left != null) {
                        mainStack.push(root.left);
                    }
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/all-elements-in-two-binary-search-trees/description/
    static class Solution3 {
        public List<Integer> getAllElements(TreeNode root1, TreeNode root2) {
            var inorder = new Object() {
                void apply(TreeNode r, List<Integer> vals) {
                    if (r == null)
                        return;
                    apply(r.left, vals);
                    vals.add(r.val);
                    apply(r.right, vals);
                }
            };

            var res = new ArrayList<Integer>();
            var t1 = new ArrayList<Integer>();
            inorder.apply(root1, t1);

            var t2 = new ArrayList<Integer>();
            inorder.apply(root2, t2);

            int i = 0, j = 0;
            while (i < t1.size() || j < t2.size()) {
                if (i >= t1.size()) {
                    res.add(t2.get(j++));
                } else if (j >= t2.size()) {
                    res.add(t1.get(i++));
                } else if (t1.get(i) > t2.get(j)) {
                    res.add(t2.get(j++));
                } else if (t2.get(j) >= t1.get(i)) {
                    res.add(t1.get(i++));
                }
            }
            return res;
        }
    }
}
