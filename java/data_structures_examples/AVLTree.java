package data_structures_examples;

class AVLTree {
    public static class Node {
        int val;
        int h;
        Node left, right;

        Node(int v) {
            val = v;
        }

        Node(int v, Node l, Node r) {
            val = v;
            left = l;
            right = r;
        }
    }

    Node root;

    void insert(int x) {
        root = insert(root, x);
    }

    Node insert(Node v, int x) {
        if (v == null) {
            return new Node(x);
        }
        if (v.val < x) {
            v.right = insert(v.right, x);
        } else if (v.val > x) {
            v.left = insert(v.left, x);
        }

        balance(v);
        return v;
    }

    void balance(Node v) {
        int diff = v.left.h - v.right.h;
        if (Math.abs(diff) <= 1) {
            v.h = Math.max(v.left != null ? v.left.h : 0, v.right != null ? v.right.h : 0) + 1;
        } else if (diff == -2) {
            if (v.right.left.h - v.right.right.h >= 0) {
                rotateLeft(v);
            } else {
                bigRotateLeft(v);
            }
        } else if (diff == 2) {
            if (v.left.left.h - v.left.right.h >= 0) {
                rotateRight(v);
            } else {
                bigRotateRight(v);
            }
        }
    }

    void rotateLeft(Node v) {
        Node w = v.right;
        v.right = w.left;
        w.left = v;

        v.h = Math.max(v.left.h, v.right.h) + 1;
        w.h = Math.max(w.left.h, v.right.h) + 1;
    }

    void rotateRight(Node v) {
        Node w = v.left;
        v.left = w.right;
        w.right = v;

        v.h = Math.max(v.left.h, v.right.h) + 1;
        w.h = Math.max(w.left.h, w.right.h) + 1;
    }

    void bigRotateLeft(Node v) {
        rotateRight(v.right);
        rotateLeft(v);
    }

    void bigRotateRight(Node v) {
        rotateLeft(v.left);
        rotateRight(v);
    }
}

class Main {
    public static void main(String[] args) {
        AVLTree tree = new AVLTree();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        System.out.println(tree);
    }
}