class TreeNode {
    val;
    left;
    right;

    constructor(val, left, right) {
        this.val = val;
        this.left = left ?? null;
        this.right = right ?? null;
    }
}

const distance_k = (root, target, k) => {
    const parents = new Map();
    const dfs = (root, parent) => {
        if (root !== null) {
            parents.set(root, parent);
            dfs(root.left, root);
            dfs(root.right, root);
        }
    }
    dfs(root, null);

    const visited = new Set();
    visited.add(target);
    visited.add(null);

    const queue = [];
    queue.push(null);
    queue.push(target);

    let dist = 0;
    while (queue.length > 0) {
        let node = queue.shift();
        if (node === null) {
            if (dist === k) {
                return queue.map(x => x.val);
            }
            queue.push(null);
            dist += 1;
        } else {
            if (!visited.has(node.left)) {
                visited.add(node.left);
                queue.push(node.left);
            }
            if (!visited.has(node.right)) {
                visited.add(node.right);
                queue.push(node.right);
            }
            const par = parents.get(node);
            if (!visited.has(par)) {
                visited.add(par);
                queue.push(par);
            }
        }
    }
    return [];
}

const test_distant_k = () => {
    const target = new TreeNode(5,
        new TreeNode(6),
        new TreeNode(2,
            new TreeNode(7),
            new TreeNode(4)
        )
    );
    const root = new TreeNode(3,
        target,
        new TreeNode(1, new TreeNode(0), new TreeNode(8))
    );
    const nodes = distance_k(root, target, 2);
    console.log(JSON.stringify(nodes));
}

test_distant_k();