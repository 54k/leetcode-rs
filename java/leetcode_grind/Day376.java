package leetcode_grind;

import java.util.HashMap;

public class Day376 {
    // https://leetcode.com/problems/arithmetic-slices/
    static class Solution1 {
        public int numberOfArithmeticSlices1(int[] nums) {
            var rec = new Object() {
                int sum = 0;

                int apply(int i) {
                    if (i < 2) {
                        return 0;
                    }
                    var ans = 0;
                    if ((nums[i] - nums[i - 1]) == (nums[i - 1] - nums[i - 2])) {
                        ans = 1 + apply(i - 1);
                        sum += ans;
                    } else {
                        apply(i - 1);
                    }
                    return ans;
                }
            };

            rec.apply(nums.length - 1);
            return rec.sum;
        }

        public int numberOfArithmeticSlices2(int[] nums) {
            var dp = new int[nums.length];
            var sum = 0;

            for (int i = 2; i < dp.length; i++) {
                if (nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2]) {
                    dp[i] = 1 + dp[i - 1];
                    sum += dp[i];
                }
            }
            return sum;
        }
    }

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

    // https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
    static class Solution2 {
        public TreeNode buildTree(int[] preorder, int[] inorder) {
            var map = new HashMap<Integer, Integer>();
            for (int i = 0; i < inorder.length; i++) {
                map.put(inorder[i], i);
            }
            var idx = new int[1];
            var build = new Object() {
                TreeNode apply(int left, int right) {
                    if (left > right) {
                        return null;
                    }

                    var val = preorder[idx[0]++];
                    TreeNode root = new TreeNode(val);
                    root.left = apply(left, map.get(val) - 1);
                    root.right = apply(map.get(val) + 1, right);
                    return root;
                }
            };
            return build.apply(0, preorder.length - 1);
        }
    }

    // https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/
    static class Solution3 {
        public TreeNode buildTree(int[] inorder, int[] postorder) {
            var idx = new int[1];
            idx[0] = postorder.length - 1;

            var map = new HashMap<Integer, Integer>();
            for (int i = 0; i < inorder.length; i++) {
                map.put(inorder[i], i);
            }

            var build = new Object() {
                TreeNode apply(int left, int right) {
                    if (left > right) {
                        return null;
                    }
                    var val = postorder[idx[0]--];
                    var root = new TreeNode(val);
                    root.right = apply(map.get(val) + 1, right);
                    root.left = apply(left, map.get(val) - 1);
                    return root;
                }
            };

            return build.apply(0, postorder.length - 1);
        }
    }

    // https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/description/
    static class Solution4 {
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
}
