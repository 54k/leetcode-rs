// https://leetcode.com/problems/construct-quad-tree/description/
// https://leetcode.com/problems/construct-quad-tree/editorial/

function Node(val, isLeaf, topLeft, topRight, bottomLeft, bottomRight) {
    this.val = val;
    this.isLeaf = isLeaf;
    this.topLeft = topLeft;
    this.topRight = topRight;
    this.bottomLeft = bottomLeft;
    this.bottomRight = bottomRight;
};

/**
 * @param {number[][]} grid
 * @return {Node}
 */
const construct = function (grid) {
    const n = grid.length;
    const is_same = (x, y, length) => {
        for (let i = x; i < x + length; i++) {
            for (let j = y; j < y + length; j++) {
                if (grid[i][j] !== grid[x][y]) {
                    return false;
                }
            }
        }
        return true;
    };
    const build_tree = (x, y, length) => {
        if (is_same(x, y, length)) {
            return new Node(grid[x][y], true);
        } else {
            const root = new Node(grid[x][y], false);
            const half_length = (length / 2) >> 0;
            root.topLeft = build_tree(x, y, half_length, half_length);
            root.topRight = build_tree(x, y + half_length, half_length);
            root.bottomLeft = build_tree(x + half_length, y, half_length);
            root.bottomRight = build_tree(x + half_length, y + half_length, half_length);
            return root;
        }
    };
    return build_tree(0, 0, n);
};

const construct_optimized = function (grid) {
    const build_tree = (x, y, length) => {
        if (length === 1) {
            return new Node(grid[x][y], true);
        }
        const half_length = (length / 2) >> 0;
        const topLeft = build_tree(x, y, half_length, half_length);
        const topRight = build_tree(x, y + half_length, half_length);
        const bottomLeft = build_tree(x + half_length, y, half_length);
        const bottomRight = build_tree(x + half_length, y + half_length, half_length);
        if (topLeft.isLeaf && topRight.isLeaf && bottomLeft.isLeaf && bottomRight.isLeaf &&
            topLeft.val == topRight.val && topRight.val == bottomLeft.val && bottomLeft.val == bottomRight.val) {
            return new Node(topLeft.val, true);
        }
        return new Node(grid[x][y], false, topLeft, topRight, bottomLeft, bottomRight);
    };
    return build_tree(0, 0, grid.length);
};