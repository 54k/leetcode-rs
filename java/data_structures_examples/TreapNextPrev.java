package data_structures_examples;

import java.util.Random;

public class TreapNextPrev {
    // https://leetcode.com/problems/contains-duplicate-iii/description/
    static class Solution1 {
        static final Random rand = new Random();

        static class Node {
            int v, p;
            Node left, right;
            int sz;

            Node(int val) {
                v = val;
                p = rand.nextInt();
                sz = 1;
            }
        }

        void update(Node node) {
            if (node == null)
                return;
            node.sz = 1;
            if (node.left != null) {
                node.sz += node.left.sz;
            }
            if (node.right != null) {
                node.sz += node.right.sz;
            }
        }

        Node[] split(Node root, int x) {
            if (root == null) {
                return new Node[] { null, null };
            }
            if (root.v <= x) {
                var split = split(root.right, x);
                root.right = split[0];
                update(root);
                return new Node[] { root, split[1] };
            }
            var split = split(root.left, x);
            root.left = split[1];
            update(root);
            return new Node[] { split[0], root };
        }

        Node merge(Node r1, Node r2) {
            if (r1 == null)
                return r2;
            if (r2 == null)
                return r1;

            if (r1.p < r2.p) {
                r1.right = merge(r1.right, r2);
                update(r1);
                return r1;
            }
            r2.left = merge(r1, r2.left);
            update(r2);
            return r2;
        }

        Node insert(Node root, int x) {
            Node[] s1 = split(root, x);
            Node[] s2 = split(s1[0], x - 1);
            Node node = new Node(x);
            return merge(s2[0], merge(merge(s2[1], node), s1[1]));
        }

        Node remove(Node root, int x) {
            Node[] s1 = split(root, x);
            Node[] s2 = split(s1[0], x - 1);
            return merge(s2[0], s1[1]);
        }

        Node prev(Node root, int x) {
            Node succ = null;
            while (root != null) {
                if (root.v > x) {
                    root = root.left;
                } else {
                    succ = root;
                    root = root.right;
                }
            }
            return succ;
        }

        Node next(Node root, int x) {
            Node succ = null;
            while (root != null) {
                if (root.v <= x) {
                    root = root.right;
                } else {
                    succ = root;
                    root = root.left;
                }
            }
            return succ;
        }

        public boolean containsNearbyAlmostDuplicate(int[] nums, int k, int t) {
            Node root = null;
            for (int i = 0; i < nums.length; i++) {
                Node nxt = next(root, nums[i]);
                if (nxt != null && nxt.v <= nums[i] + t)
                    return true;

                Node prv = prev(root, nums[i]);
                if (prv != null && nums[i] <= prv.v + t)
                    return true;

                root = insert(root, nums[i]);
                if (root != null && root.sz > k) {
                    root = remove(root, nums[i - k]);
                }
            }
            return false;
        }
    }
}
