package leetcode_grind;

import java.util.*;

public class Day660 {
    public class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

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

    // https://leetcode.com/problems/linked-list-in-binary-tree/description/?envType=daily-question&envId=2024-09-07
    static class Solution1 {
        public boolean isSubPath(ListNode head, TreeNode root) {
            if (root == null)
                return false;
            boolean result = dfs(root, head);
            result |= isSubPath(head, root.left);
            result |= isSubPath(head, root.right);
            return result;
        }

        boolean dfs(TreeNode node, ListNode head) {
            if (head == null)
                return true;
            if (node == null)
                return false;
            boolean result = false;
            if (node.val == head.val) {
                result |= dfs(node.right, head.next) || dfs(node.left, head.next);
            }
            return result;
        }
    }

    static class Solution2 {
        public boolean isSubPath(ListNode head, TreeNode root) {
            if (root == null)
                return false;
            return checkPath(root, head);
        }

        boolean checkPath(TreeNode node, ListNode head) {
            if (node == null)
                return false;
            if (dfs(node, head))
                return true;
            return checkPath(node.left, head) || checkPath(node.right, head);
        }

        boolean dfs(TreeNode node, ListNode head) {
            if (head == null)
                return true;
            if (node == null)
                return false;
            if (node.val != head.val)
                return false;
            return dfs(node.left, head.next) || dfs(node.right, head.next);
        }
    }

    static class Solution3 {
        public boolean isSubPath(ListNode head, TreeNode root) {
            if (root == null)
                return false;
            Stack<TreeNode> nodes = new Stack<>();
            nodes.push(root);

            while (!nodes.isEmpty()) {
                TreeNode node = nodes.pop();
                if (isMatch(node, head)) {
                    return true;
                }
                if (node.left != null) {
                    nodes.push(node.left);
                }
                if (node.right != null) {
                    nodes.push(node.right);
                }
            }
            return false;
        }

        boolean isMatch(TreeNode node, ListNode lst) {
            Stack<Map.Entry<TreeNode, ListNode>> s = new Stack<>();
            s.push(new HashMap.SimpleEntry<>(node, lst));

            while (!s.isEmpty()) {
                Map.Entry<TreeNode, ListNode> entry = s.pop();
                TreeNode currentNode = entry.getKey();
                ListNode currentList = entry.getValue();

                while (currentNode != null && currentList != null) {
                    if (currentNode.val != currentList.val) {
                        break;
                    }

                    currentList = currentList.next;

                    if (currentList != null) {
                        if (currentNode.left != null) {
                            s.push(
                                    new HashMap.SimpleEntry<>(
                                            currentNode.left,
                                            currentList));
                        }
                        if (currentNode.right != null) {
                            s.push(
                                    new HashMap.SimpleEntry<>(
                                            currentNode.right,
                                            currentList));
                        }
                        break;
                    }
                    if (currentList == null) {
                        return true;
                    }
                }
            }
            return false;
        }
    }

    static class Solution4 {
        public boolean isSubPath(ListNode head, TreeNode root) {
            List<Integer> pattern = new ArrayList<>();
            List<Integer> prefixTable = new ArrayList<>();
            pattern.add(head.val);
            prefixTable.add(0);
            int patternIndex = 0;
            head = head.next;
            while (head != null) {
                while (patternIndex > 0 && head.val != pattern.get(patternIndex)) {
                    patternIndex = prefixTable.get(patternIndex - 1);
                }
                patternIndex += head.val == pattern.get(patternIndex) ? 1 : 0;
                pattern.add(head.val);
                prefixTable.add(patternIndex);
                head = head.next;
            }
            return searchInTree(root, 0, pattern, prefixTable);
        }

        boolean searchInTree(
                TreeNode node,
                int patternIndex,
                List<Integer> pattern,
                List<Integer> prefixTable) {
            if (node == null)
                return false;
            while (patternIndex > 0 && node.val != pattern.get(patternIndex)) {
                patternIndex = prefixTable.get(patternIndex - 1);
            }
            patternIndex += node.val == pattern.get(patternIndex) ? 1 : 0;

            if (patternIndex == pattern.size())
                return true;

            return (searchInTree(node.left, patternIndex, pattern, prefixTable) ||
                    searchInTree(node.right, patternIndex, pattern, prefixTable));
        }
    }
}
