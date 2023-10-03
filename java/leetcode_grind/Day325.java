package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;

public class Day325 {
    // https://leetcode.com/problems/number-of-good-pairs/description
    static class Solution1 {
        public int numIdenticalPairs(int[] nums) {
            var m = new HashMap<Integer, Integer>();
            var ans = 0;
            for (var n : nums) {
                ans += m.getOrDefault(n, 0);
                m.put(n, m.getOrDefault(n, 0) + 1);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/maximum-depth-of-n-ary-tree/description/
    // Definition for a Node.
    class Node {
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

    class Solution2 {
        public int maxDepth(Node root) {
            class Pair {
                Node node;
                int depth;

                Pair(Node n, int d) {
                    this.node = n;
                    this.depth = d;
                }
            }
            var ans = 0;
            if (root == null) {
                return ans;
            }
            var stack = new LinkedList<Pair>();
            stack.push(new Pair(root, 1));
            while (stack.size() > 0) {
                var pop = stack.pop();
                ans = Math.max(ans, pop.depth);
                for (var c : pop.node.children) {
                    if (c != null) {
                        stack.push(new Pair(c, pop.depth + 1));
                    }
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/encode-n-ary-tree-to-binary-tree/description/
    class Solution3 {
        class Node {
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

        public class TreeNode {
            int val;
            TreeNode left;
            TreeNode right;

            TreeNode(int x) {
                val = x;
            }
        }

        class Codec {
            static class Pair<U, V> {
                U first;
                V second;

                Pair(U f, V s) {
                    first = f;
                    second = s;
                }
            }

            // Encodes an n-ary tree to a binary tree.
            public TreeNode encode(Node root) {
                if (root == null) {
                    return null;
                }
                var newRoot = new TreeNode(root.val);

                var queue = new ArrayDeque<Pair<TreeNode, Node>>();
                var head = new Pair<TreeNode, Node>(newRoot, root);
                queue.add(head);

                while (queue.size() > 0) {
                    var pair = queue.remove();
                    var bNode = pair.first;
                    var nNode = pair.second;

                    TreeNode prevBNode = null;
                    TreeNode headBNode = null;

                    for (var nChild : nNode.children) {
                        var newBNode = new TreeNode(nChild.val);
                        if (prevBNode == null) {
                            headBNode = newBNode;
                        } else {
                            prevBNode.right = newBNode;
                        }
                        prevBNode = newBNode;
                        var nextEntry = new Pair<TreeNode, Node>(newBNode, nChild);
                        queue.add(nextEntry);
                    }

                    bNode.left = headBNode;
                }

                return newRoot;
            }

            // Decodes your binary tree to an n-ary tree.
            public Node decode(TreeNode root) {
                if (root == null) {
                    return null;
                }

                var newRoot = new Node(root.val, new ArrayList<Node>());
                var queue = new ArrayDeque<Pair<Node, TreeNode>>();
                var head = new Pair<Node, TreeNode>(newRoot, root);
                queue.add(head);

                while (queue.size() > 0) {
                    var entry = queue.remove();
                    var nNode = entry.first;
                    var bNode = entry.second;

                    var sibling = bNode.left;

                    while (sibling != null) {
                        var nChild = new Node(sibling.val, new ArrayList<Node>());
                        nNode.children.add(nChild);

                        var nextEntry = new Pair<Node, TreeNode>(nChild, sibling);
                        queue.add(nextEntry);

                        sibling = sibling.right;
                    }
                }

                return newRoot;
            }
        }
    }
}
