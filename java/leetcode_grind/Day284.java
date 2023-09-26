package leetcode_grind;
abstract class Node {
    public abstract int evaluate();
    // define your fields here
};

class OpNode extends Node {
    String op;
    Node left;
    Node right;

    OpNode(String op, Node left, Node right) {
        this.op = op;
        this.left = left;
        this.right = right;
    } 

    public int evaluate() {
        switch(op) {
            case "+":
                return left.evaluate() + right.evaluate();
            case "-":
                return right.evaluate() - left.evaluate();
            case "*":
                return left.evaluate() * right.evaluate();
            case "/":
                return right.evaluate() / left.evaluate();
        }
        throw new Error("Unknown op");
    }
}

class NumNode extends Node {
    int num;
    NumNode(int num) {
        this.num = num;
    }

    public int evaluate() {
        return num;
    }
}

class TreeBuilder {
    Node buildTree(String[] postfix) {
        var stack = new Stack<Node>();
        for (var s : postfix) {
            if (Character.isDigit(s.charAt(0))) {
                stack.push(new NumNode(Integer.valueOf(s)));
            } else {
                var left = stack.pop();
                var right = stack.pop();
                stack.push(new OpNode(s, left, right));
            }
        }
        return stack.pop();
    }
};
