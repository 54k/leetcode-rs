package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day422 {
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

    // https://leetcode.com/problems/range-sum-of-bst/description
    static class Solution1 {
        public int rangeSumBST(TreeNode root, int low, int high) {
            int ans = 0;
            Stack<TreeNode> stack = new Stack<>();
            stack.push(root);

            while (!stack.isEmpty()) {
                TreeNode node = stack.pop();
                if (node != null) {
                    if (low <= node.val && node.val <= high) {
                        ans += node.val;
                    }
                    if (low < node.val) {
                        stack.push(node.left);
                    }
                    if (high > node.val) {
                        stack.push(node.right);
                    }
                }
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/subtree-of-another-tree/description/
    static class Solution2 {
        boolean isSame(TreeNode root, TreeNode subRoot) {
            if (root == null && subRoot == null) {
                return true;
            } else if (root == null && subRoot != null) {
                return false;
            } else if (root != null && subRoot == null) {
                return false;
            }
            return root.val == subRoot.val && isSame(root.left, subRoot.left) && isSame(root.right, subRoot.right);
        }

        public boolean isSubtree(TreeNode root, TreeNode subRoot) {
            if (root == null) {
                return subRoot == null;
            }

            if (root.val == subRoot.val) {
                return isSame(root, subRoot) || isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
            }

            return isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
        }
    }

    // https://leetcode.com/problems/find-leaves-of-binary-tree/
    static class Solution3 {
        public List<List<Integer>> findLeaves(TreeNode root) {
            var leaves = new ArrayList<List<Integer>>();
            var dfs = new Object() {
                int apply(TreeNode root) {
                    if (root == null) {
                        return -1;
                    }

                    int l = apply(root.left);
                    int r = apply(root.right);
                    int d = 1 + Math.max(l, r);

                    if (d == leaves.size()) {
                        leaves.add(new ArrayList<>());
                    }

                    leaves.get(d).add(root.val);
                    return d;
                }
            };

            dfs.apply(root);
            return leaves;
        }
    }

    // https://leetcode.com/problems/read-n-characters-given-read4-ii-call-multiple-times/
    public static class Reader4 {
        int read4(char[] buf) {
            return 0;
        }

    }

    public static class Solution4 extends Reader4 {
        int bufPtr = 0;
        int bufCnt = 0;
        char[] buf4 = new char[4];

        /**
         * @param buf Destination buffer
         * @param n   Number of characters to read
         * @return The number of actual characters read
         */
        public int read(char[] buf, int n) {
            int ptr = 0;
            while (ptr < n) {
                if (bufPtr == 0) {
                    bufCnt = read4(buf4);
                }
                if (bufCnt == 0)
                    break;

                while (ptr < n && bufPtr < bufCnt) {
                    buf[ptr++] = buf4[bufPtr++];
                }
                if (bufPtr >= bufCnt)
                    bufPtr = 0;
            }
            return ptr;
        }
    }
}
