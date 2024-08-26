package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day648 {
    static class Node {
        public int val;
        public List<Node> children;

        public Node() {
        }

        public Node(int _val) {
            val = _val;
        }

        public Node(int _val, List<Node> _children) {
            val = _val;
            children = _children;
        }
    };

    // https://leetcode.com/problems/n-ary-tree-postorder-traversal/description/?envType=daily-question&envId=2024-08-26
    static class Solution1 {
        public List<Integer> postorder(Node root) {
            List<Integer> result = new ArrayList<>();
            if (root == null)
                return result;

            Stack<Node> nodeStack = new Stack<>();
            Stack<Node> reverseStack = new Stack<>();
            nodeStack.push(root);

            while (!nodeStack.isEmpty()) {
                Node currentNode = nodeStack.pop();
                reverseStack.push(currentNode);
                for (Node child : currentNode.children) {
                    nodeStack.push(child);
                }
            }

            while (!reverseStack.isEmpty()) {
                Node currentNode = reverseStack.pop();
                result.add(currentNode.val);
            }
            return result;
        }
    }

    static class Solution2 {
        static class NodeVisitPair {
            Node node;
            boolean isVisited;

            NodeVisitPair(Node node, boolean isVisited) {
                this.node = node;
                this.isVisited = isVisited;
            }
        }

        public List<Integer> postorder(Node root) {
            List<Integer> result = new ArrayList<>();
            if (root == null)
                return result;
            Stack<NodeVisitPair> nodeStack = new Stack<>();
            nodeStack.push(new NodeVisitPair(root, false));

            while (!nodeStack.isEmpty()) {
                NodeVisitPair currentPair = nodeStack.pop();
                if (currentPair.isVisited) {
                    result.add(currentPair.node.val);
                } else {
                    currentPair.isVisited = true;
                    nodeStack.push(currentPair);

                    List<Node> children = currentPair.node.children;
                    for (int i = children.size() - 1; i >= 0; i--) {
                        nodeStack.push(new NodeVisitPair(children.get(i), false));
                    }
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/binary-tree-postorder-traversal/description/?envType=daily-question&envId=2024-08-25
    static class Solution4 {
        public List<Integer> postorderTraversal(TreeNode root) {
            List<Integer> result = new ArrayList<>();
            if (root == null)
                return result;
            TreeNode previousNode = null;
            Stack<TreeNode> traversalStack = new Stack<>();

            while (root != null || !traversalStack.isEmpty()) {
                if (root != null) {
                    traversalStack.push(root);
                    root = root.left;
                } else {
                    root = traversalStack.peek();

                    if (root.right == null || root.right == previousNode) {
                        result.add(root.val);
                        traversalStack.pop();
                        previousNode = root;
                        root = null;
                    } else {
                        root = root.right;
                    }
                }
            }
            return result;
        }
    }

    static class Solution5 {
        public List<Integer> postorderTraversal(TreeNode root) {
            List<Integer> result = new ArrayList<>();
            if (root == null) {
                return result;
            }

            TreeNode dummyNode = new TreeNode(-1);
            TreeNode predecessor = null;
            dummyNode.left = root;
            root = dummyNode;

            while (root != null) {
                if (root.left != null) {
                    predecessor = root.left;

                    while (predecessor.right != null && predecessor.right != root) {
                        predecessor = predecessor.right;
                    }

                    if (predecessor.right == null) {
                        predecessor.right = root;
                        root = root.left;
                    } else {
                        TreeNode node = predecessor;
                        reverseSubtreeLinks(root.left, predecessor);
                        while (node != root.left) {
                            result.add(node.val);
                            node = node.right;
                        }
                        result.add(node.val);
                        reverseSubtreeLinks(predecessor, root.left);
                        predecessor.right = null;
                        root = root.right;
                    }
                } else {
                    root = root.right;
                }
            }
            return result;
        }

        void reverseSubtreeLinks(TreeNode startNode, TreeNode endNode) {
            if (startNode == endNode) {
                return;
            }

            TreeNode prev = null;
            TreeNode current = startNode;
            TreeNode next = null;

            while (current != endNode) {
                next = current.right;
                current.right = prev;
                prev = current;
                current = next;
            }
            current.right = prev;
        }
    }
}
