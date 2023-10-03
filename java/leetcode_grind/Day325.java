package leetcode_grind;

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
}
