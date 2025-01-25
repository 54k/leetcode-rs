package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedList;

public class Day798 {
    // https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/description/?envType=daily-question&envId=2025-01-25
    static class Solution1 {
        public int[] lexicographicallySmallestArray(int[] nums, int limit) {
            int[] numsSorted = new int[nums.length];
            for (int i = 0; i < nums.length; i++) {
                numsSorted[i] = nums[i];
            }
            Arrays.sort(numsSorted);

            int currGroup = 0;
            HashMap<Integer, Integer> numToGroup = new HashMap<>();
            numToGroup.put(numsSorted[0], currGroup);

            HashMap<Integer, LinkedList<Integer>> groupToList = new HashMap<>();
            groupToList.put(currGroup, new LinkedList<Integer>(Arrays.asList(numsSorted[0])));

            for (int i = 1; i < nums.length; i++) {
                if (Math.abs(numsSorted[i] - numsSorted[i - 1]) > limit) {
                    currGroup++;
                }

                numToGroup.put(numsSorted[i], currGroup);

                if (!groupToList.containsKey(currGroup)) {
                    groupToList.put(currGroup, new LinkedList<>());
                }

                groupToList.get(currGroup).add(numsSorted[i]);
            }

            for (int i = 0; i < nums.length; i++) {
                int num = nums[i];
                int group = numToGroup.get(num);
                nums[i] = groupToList.get(group).pop();
            }

            return nums;
        }
    }

    // https://leetcode.com/problems/largest-bst-subtree/description/
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

    static class Solution2 {
        boolean isValidBST(TreeNode root) {
            if (root == null) {
                return true;
            }

            int leftMax = findMax(root.left);
            if (leftMax >= root.val) {
                return false;
            }

            int rightMin = findMin(root.right);
            if (rightMin <= root.val) {
                return false;
            }

            if (isValidBST(root.left) && isValidBST(root.right)) {
                return true;
            }
            return false;
        }

        int findMax(TreeNode root) {
            if (root == null) {
                return Integer.MIN_VALUE;
            }
            return Math.max(Math.max(root.val, findMax(root.left)), findMax(root.right));
        }

        int findMin(TreeNode root) {
            if (root == null) {
                return Integer.MAX_VALUE;
            }
            return Math.min(Math.min(root.val, findMin(root.left)), findMin(root.right));
        }

        int countNodes(TreeNode root) {
            if (root == null) {
                return 0;
            }
            return 1 + countNodes(root.left) + countNodes(root.right);
        }

        public int largestBSTSubtree(TreeNode root) {
            if (root == null) {
                return 0;
            }

            if (isValidBST(root)) {
                return countNodes(root);
            }
            return Math.max(largestBSTSubtree(root.left), largestBSTSubtree(root.right));
        }
    }
}
