package leetcode_grind;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Day324 {
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

    // https://leetcode.com/problems/n-ary-tree-preorder-traversal/description/
    static class Solution1 {
        public List<Integer> postorder(Node root) {
            var answer = new LinkedList<Integer>(null);
            var stack = new LinkedList<Node>();
            if (root == null) {
                return answer;
            }
            stack.push(root);
            while (stack.size() > 0) {
                var n = stack.pop();
                answer.addFirst(n.val);
                for (var ch : n.children) {
                    if (ch != null) {
                        stack.push(ch);
                    }
                }
            }
            return answer.stream().toList();
        }
    }

    // https://leetcode.com/problems/n-ary-tree-postorder-traversal/description/
    static class Solution2 {
        public List<Integer> preorder(Node root) {
            var result = new ArrayList<Integer>();
            var stack = new LinkedList<Node>();
            if (root == null) {
                return result;
            }
            stack.push(root);
            while (stack.size() > 0) {
                var n = stack.pop();
                result.add(n.val);
                for (int i = n.children.size() - 1; i >= 0; i--) {
                    stack.push(n.children.get(i));
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/n-ary-tree-level-order-traversal/description/
    static class Solution3 {
        public List<List<Integer>> levelOrder(Node root) {
            var result = new ArrayList<List<Integer>>();
            var queue = new LinkedList<Node>();
            if (root == null) {
                return result;
            }
            queue.add(root);
            while (queue.size() > 0) {
                result.add(queue.stream().map(n -> n.val).toList());
                var k = queue.size();
                while (k-- > 0) {
                    var n = queue.pollFirst();
                    for (var ch : n.children) {
                        if (ch != null) {
                            queue.addLast(ch);
                        }
                    }
                }
            }
            return result;
        }
    }
}
