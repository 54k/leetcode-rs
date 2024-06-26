package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Set;
import java.util.Stack;

public class Day549 {
    // https://leetcode.com/problems/number-of-spaces-cleaning-robot-cleaned/description
    static class Solution1 {
        private final int[] DIRECTIONS = { 0, 1, 0, -1, 0 };

        public int numberOfCleanRooms(int[][] room) {
            int rows = room.length;
            int cols = room[0].length;
            Set<String> visited = new HashSet<>();
            Set<String> cleaned = new HashSet<>();
            return clean(room, rows, cols, 0, 0, 0, visited, cleaned);
        }

        int clean(int[][] room, int rows, int cols, int row, int col, int direction, Set<String> visited,
                Set<String> cleaned) {
            if (visited.contains(row + "," + col + "," + direction)) {
                return cleaned.size();
            }

            visited.add(row + "," + col + "," + direction);
            cleaned.add(row + "," + col);

            int nextRow = row + DIRECTIONS[direction];
            int nextCol = col + DIRECTIONS[direction + 1];

            if (0 <= nextRow && nextRow < rows && 0 <= nextCol && nextCol < cols && room[nextRow][nextCol] == 0) {
                return clean(room, rows, cols, nextRow, nextCol, direction, visited, cleaned);
            }

            return clean(room, rows, cols, row, col, (direction + 1) % 4, visited, cleaned);
        }
    }

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {
        }

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    // https://leetcode.com/problems/evaluate-boolean-binary-tree/
    static class Solution2 {
        public boolean evaluateTree(TreeNode root) {
            Stack<TreeNode> st = new Stack<>();
            st.push(root);
            HashMap<TreeNode, Boolean> evaluated = new HashMap<>();

            while (!st.isEmpty()) {
                TreeNode topNode = st.peek();

                if (topNode.left == null && topNode.right == null) {
                    st.pop();
                    evaluated.put(topNode, topNode.val == 1);
                    continue;
                }

                if (evaluated.containsKey(topNode.left) && evaluated.containsKey(topNode.right)) {
                    st.pop();
                    if (topNode.val == 2) {
                        evaluated.put(topNode, evaluated.get(topNode.left) || evaluated.get(topNode.right));
                    } else {
                        evaluated.put(topNode, evaluated.get(topNode.left) && evaluated.get(topNode.right));
                    }
                } else {
                    st.push(topNode.right);
                    st.push(topNode.left);
                }
            }

            return evaluated.get(root);
        }
    }

    // https://leetcode.com/problems/number-of-spaces-cleaning-robot-cleaned/description
    static class Solution3 {
        int[] DIRECTIONS = { 0, 1, 0, -1, 0 };

        public int numberOfCleanRooms(int[][] room) {
            int rows = room.length;
            int cols = room[0].length;
            int[][] visited = new int[rows][cols];
            int cleaned = 0;

            Queue<int[]> queue = new LinkedList<>();
            queue.offer(new int[] { 0, 0, 0 });

            while (!queue.isEmpty()) {
                int[] current = queue.poll();
                int row = current[0];
                int col = current[1];
                int direction = current[2];

                if (visited[row][col] == 0) {
                    cleaned += 1;
                }

                visited[row][col] |= 1 << direction;

                for (int d = 0; d < 4; d++) {
                    int nextDir = (direction + d) % 4;
                    int nextRow = row + DIRECTIONS[nextDir];
                    int nextCol = col + DIRECTIONS[nextDir + 1];

                    if (0 <= nextRow && nextRow < rows && 0 <= nextCol && nextCol < cols
                            && room[nextRow][nextCol] == 0) {
                        if ((visited[nextRow][nextCol] >> nextDir & 1) == 1) {
                            return cleaned;
                        } else {
                            queue.offer(new int[] { nextRow, nextCol, nextDir });
                            break;
                        }
                    }
                }
            }
            return cleaned;
        }
    }
}
