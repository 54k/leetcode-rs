import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.PriorityQueue;

public class Day308 {
    // https://leetcode.com/problems/path-with-minimum-effort/
    public int minimumEffortPathBruteForceTLE(int[][] heights) {
        var backtrack = new Object() {
            static final int[][] DIRS = new int[][] {
                    { 1, 0 },
                    { -1, 0 },
                    { 0, 1 },
                    { 0, -1 },
            };

            int ans = Integer.MAX_VALUE;
            int row = heights.length;
            int col = heights[0].length;

            List<int[]> nextValidMoves(int x, int y) {
                var moves = new ArrayList<int[]>();
                for (var dir : DIRS) {
                    var nx = x + dir[0];
                    var ny = y + dir[1];

                    if (nx >= 0 && nx < row && ny >= 0 && ny < col) {
                        if (heights[nx][ny] != 0) {
                            moves.add(new int[] { nx, ny });
                        }
                    }
                }
                return moves;
            }

            int dfs(int x, int y, int maxDiff) {
                if (x == row - 1 && y == col - 1) {
                    ans = Math.min(ans, maxDiff);
                    return maxDiff;
                }

                var val = heights[x][y];
                heights[x][y] = 0;

                var ans = Integer.MAX_VALUE;
                for (var move : nextValidMoves(x, y)) {
                    var nx = move[0];
                    var ny = move[1];

                    var curDiff = Math.abs(val - heights[nx][ny]);
                    var res = dfs(nx, ny, Math.max(maxDiff, curDiff));
                    ans = Math.min(ans, res);
                }

                heights[x][y] = val;
                return maxDiff;
            }
        };

        backtrack.dfs(0, 0, 0);
        return backtrack.ans;
    }

    public int minimumEffortPathDijkstra(int[][] heights) {
        @FunctionalInterface
        interface Function2<A, B> {
            boolean apply(A x, B y);
        }

        class Node {
            final int x, y;
            final int weight;

            Node(int x, int y, int weight) {
                this.x = x;
                this.y = y;
                this.weight = weight;
            }
        }

        final var DIRS = new int[][] {
                { -1, 0 },
                { 1, 0 },
                { 0, 1 },
                { 0, -1 },
        };

        var row = heights.length;
        var col = heights[0].length;

        Function2<Integer, Integer> isValidMove = (x, y) -> {
            return x >= 0 && x < heights.length && y >= 0 && y < heights[0].length;
        };

        var visited = new boolean[row][col];
        var diffMatrix = new int[row][col];
        for (var i = 0; i < diffMatrix.length; i++) {
            for (var j = 0; j < diffMatrix[i].length; j++) {
                diffMatrix[i][j] = Integer.MAX_VALUE;
            }
        }
        diffMatrix[0][0] = 0;

        var pq = new PriorityQueue<Node>(new Comparator<Node>() {
            @Override
            public int compare(Node o1, Node o2) {
                return o1.weight - o2.weight;
            }

        });

        pq.add(new Node(0, 0, 0));

        while (!pq.isEmpty()) {
            var cur = pq.poll();
            visited[cur.x][cur.y] = true;

            if (cur.x == row - 1 && cur.y == col - 1) {
                return cur.weight;
            }

            for (var dir : DIRS) {
                var nx = cur.x + dir[0];
                var ny = cur.y + dir[1];
                if (isValidMove.apply(nx, ny) && !visited[nx][ny]) {
                    var curDiff = Math.abs(heights[cur.x][cur.y] - heights[nx][ny]);
                    var maxDiff = Math.max(diffMatrix[cur.x][cur.y], curDiff);

                    if (diffMatrix[nx][ny] > maxDiff) {
                        diffMatrix[nx][ny] = maxDiff;
                        pq.add(new Node(nx, ny, maxDiff));
                    }
                }
            }
        }

        return diffMatrix[row - 1][col - 1];
    }

    // https://leetcode.com/problems/path-with-maximum-minimum-value/
    public int maximumMinimumPath(int[][] grid) {
        var row = grid.length;
        var col = grid[0].length;

        class DFS {
            int threshold;
            boolean[][] visited = new boolean[row][col];

            DFS(int threshold) {
                this.threshold = threshold;
            }

            boolean run(int x, int y) {
                if (x == row - 1 && y == col - 1) {
                    return true;
                }
                visited[x][y] = true;

                for (var d : new int[][] {
                        { -1, 0 },
                        { 1, 0 },
                        { 0, 1 },
                        { 0, -1 },
                }) {
                    var nx = x + d[0];
                    var ny = y + d[1];
                    if (nx >= 0 && nx < row && ny >= 0 && ny < col && !visited[nx][ny] && grid[nx][ny] >= threshold) {
                        if (run(nx, ny)) {
                            return true;
                        }
                    }
                }
                return false;
            }
        }

        var left = 0;
        var right = Math.min(grid[0][0], grid[row - 1][col - 1]);

        // while (left <= right) {
        // var mid = (right + left + 1) / 2;
        // var dfs = new DFS(mid);
        // if (dfs.run(0, 0)) {
        // left = mid + 1;
        // } else {
        // right = mid - 1;
        // }
        // }
        // return right;

        while (left < right) {
            var mid = (right + left + 1) / 2;
            var dfs = new DFS(mid);
            if (dfs.run(0, 0)) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        return left;
    }
}
