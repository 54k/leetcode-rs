import java.util.Random;

// https://en.wikipedia.org/wiki/Treap
// https://ru.algorithmica.org/cs/tree-structures/treap/
public class TreapExample {
    static class Node {
        int key;
        int priority = new Random().nextInt();
        Node left, right;

        Node(int k) {
            this.key = k;
        }
    }

    Node root;

    void insertExample(int x) {
        Node[] leftRight = split(root, x);

        Node left = leftRight[0];
        Node right = leftRight[1];

        Node t = new Node(x);
        root = merge(left, merge(t, right));
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
    /**
     * Splits treap for left and right subtrees where left subtree keys <= x
     * @param p
     * @param x
     * @return
     */
    Node[] split(Node p, int x) {
        if (p == null) {
            return new Node[] { null, null };
        }

        if (p.key <= x) {
            Node[] leftRight = split(p.right, x);
            Node left = leftRight[0];
            Node right = leftRight[1];
            p.right = left;
            return new Node[] { p, right };
        }

        Node[] leftRight = split(p.left, x);
        Node left = leftRight[0];
        Node right = leftRight[1];
        p.left = right;
        return new Node[] { left, p };
    }
}
