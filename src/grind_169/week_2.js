/**
 * // Definition for a Node.
 */
function Node(val, neighbors) {
    this.val = val === undefined ? 0 : val;
    this.neighbors = neighbors === undefined ? [] : neighbors;
};

/**
 * @param {Node} node
 * @return {Node}
 */

// https://leetcode.com/problems/clone-graph/
var cloneGraph = function (node) {
    let map = {};
    const go = (node) => {
        if (!node) {
            return node;
        }
        if (!map[node.val]) {
            map[node.val] = new Node(node.val);
            map[node.val].neighbors = node.neighbors.map((neighbor) => go(neighbor));
        }
        return map[node.val];
    }
    return go(node);
};