package data_structures_examples;

import java.util.Random;

// https://en.wikipedia.org/wiki/Treap
// https://ru.algorithmica.org/cs/tree-structures/treap/
public class Treap {
    static class Split {
        Node left, right;

        Split(Node left, Node right) {
            this.left = left;
            this.right = right;
        }
    }

    static class Node {
        int key;
        int priority = new Random().nextInt();
        Node left, right;

        Node(int k) {
            key = k;
        }
    }

    Node merge(Node left, Node right) {
        if (left == null) {
            return right;
        }
        if (right == null) {
            return left;
        }

        if (left.priority > right.priority) {
            left.right = merge(left.right, right);
            return left;
        }

        right.left = merge(left, right.left);
        return right;
    }

    Split split(Node p, int x) {
        if (p == null) {
            return new Split(null, null);
        }

        if (p.key <= x) {
            Split leftRight = split(p.right, x);
            p.right = leftRight.left;
            return new Split(p, leftRight.right);
        }

        Split leftRight = split(p.left, x);
        p.left = leftRight.right;
        return new Split(leftRight.left, p);
    }
}
