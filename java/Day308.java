import java.util.Comparator;
import java.util.PriorityQueue;

public class Day308 {
    static class Solution {
        public int minimumEffortPath(int[][] heights) {
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
    }
}
