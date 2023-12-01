package leetcode_grind;

public class Day384 {
    static class Node {
        public int val;
        public List<Node> children;

        public Node() {
            children = new ArrayList<Node>();
        }

        public Node(int _val) {
            val = _val;
            children = new ArrayList<Node>();
        }

        public Node(int _val, ArrayList<Node> _children) {
            val = _val;
            children = _children;
        }
    };

    // https://leetcode.com/problems/diameter-of-n-ary-tree/description/
    static class Solution1 {
        int longestPath = 0;
        Node r;

        public int diameter(Node root) {
            if (r == null) {
                r = root;
            }

            if (root.children.size() == 0) {
                return 0;
            }

            int firstLongestPath = 0;
            int secondLongestPath = 0;
            for (var ch : root.children) {
                int path = diameter(ch) + 1;
                if (path > firstLongestPath) {
                    secondLongestPath = firstLongestPath;
                    firstLongestPath = path;
                } else if (path > secondLongestPath) {
                    secondLongestPath = path;
                }
            }

            longestPath = Math.max(longestPath, firstLongestPath + secondLongestPath);

            return r == root ? Math.max(longestPath, firstLongestPath) : firstLongestPath;
        }
    }

    static class Solution2 {
        int diameter = 0;

        int maxDepth(Node node, int currDepth) {
            if (node.children.size() == 0) {
                return currDepth;
            }

            int maxDepth1 = currDepth, maxDepth2 = 0;
            for (Node child : node.children) {
                int depth = maxDepth(child, currDepth + 1);
                if (depth > maxDepth1) {
                    maxDepth2 = maxDepth1;
                    maxDepth1 = depth;
                } else if (depth > maxDepth2) {
                    maxDepth2 = depth;
                }

                int distance = maxDepth1 + maxDepth2 - 2 * currDepth;
                this.diameter = Math.max(this.diameter, distance);
            }

            return maxDepth1;
        }

        public int diameter(Node root) {
            this.diameter = 0;
            maxDepth(root, 0);
            return diameter;
        }
    }
}
