package leetcode_grind;

import java.util.Arrays;
import java.util.LinkedList;
import java.util.PriorityQueue;
import java.util.Queue;

public class Day1153 {
    // https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/description/?envType=daily-question&envId=2026-01-15
    static class Solution1 {
        public int maximizeSquareHoleArea(int n, int m, int[] hBars, int[] vBars) {
            Arrays.sort(hBars);
            Arrays.sort(vBars);
            int hmax = 1;
            int vmax = 1;
            int hcur = 1;
            int vcur = 1;

            for (int i = 1; i < hBars.length; i++) {
                if (hBars[i] == hBars[i - 1] + 1) {
                    hcur++;
                } else {
                    hcur = 1;
                }
                hmax = Math.max(hmax, hcur);
            }

            for (int i = 1; i < vBars.length; i++) {
                if (vBars[i] == vBars[i - 1] + 1) {
                    vcur++;
                } else {
                    vcur = 1;
                }
                vmax = Math.max(vmax, vcur);
            }

            int side = Math.min(hmax, vmax) + 1;
            return side * side;
        }
    }

    // https://leetcode.com/problems/the-maze-ii/description/?source=submission-ac
    static class Solution2 {
        public int shortestDistance(int[][] maze, int[] start, int[] dest) {
            int[][] distance = new int[maze.length][maze[0].length];
            for (int[] row : distance) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            distance[start[0]][start[1]] = 0;
            int[][] dirs = { { 0, 1 }, { 0, -1 }, { -1, 0 }, { 1, 0 } };
            Queue<int[]> queue = new LinkedList<>();
            queue.add(start);
            while (!queue.isEmpty()) {
                int[] s = queue.remove();
                for (int[] dir : dirs) {
                    int x = s[0] + dir[0];
                    int y = s[1] + dir[1];
                    int count = 0;
                    while (x >= 0 && y >= 0 && x < maze.length && y < maze[0].length && maze[x][y] == 0) {
                        x += dir[0];
                        y += dir[1];
                        count++;
                    }
                    if (distance[s[0]][s[1]] + count < distance[x - dir[0]][y - dir[1]]) {
                        distance[x - dir[0]][y - dir[1]] = distance[s[0]][s[1]] + count;
                        queue.add(new int[] { x - dir[0], y - dir[1] });
                    }
                }
            }
            // dfs(maze, start, distance);
            return distance[dest[0]][dest[1]] == Integer.MAX_VALUE ? -1 : distance[dest[0]][dest[1]];
        }

        void dfs(int[][] maze, int[] start, int[][] distance) {
            int[][] dirs = { { 0, 1 }, { 0, -1 }, { -1, 0 }, { 1, 0 } };
            for (int[] dir : dirs) {
                int x = start[0] + dir[0];
                int y = start[1] + dir[1];
                int count = 0;

                while (x >= 0 && y >= 0 && x < maze.length && y < maze[0].length && maze[x][y] == 0) {
                    x += dir[0];
                    y += dir[1];
                    count++;
                }

                if (distance[start[0]][start[1]] + count < distance[x - dir[0]][y - dir[1]]) {
                    distance[x - dir[0]][y - dir[1]] = distance[start[0]][start[1]] + count;
                    dfs(maze, new int[] { x - dir[0], y - dir[1] }, distance);
                }
            }
        }
    }

    static class Solution3 {
        public int shortestDistance(int[][] maze, int[] start, int[] dest) {
            int[][] distance = new int[maze.length][maze[0].length];
            boolean[][] visited = new boolean[maze.length][maze[0].length];
            for (int[] row : distance) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            distance[start[0]][start[1]] = 0;
            dijkstra(maze, distance, visited);
            return distance[dest[0]][dest[1]] == Integer.MAX_VALUE ? -1 : distance[dest[0]][dest[1]];
        }

        int[] minDistance(int[][] distance, boolean[][] visited) {
            int[] min = { -1, -1 };
            int min_val = Integer.MAX_VALUE;
            for (int i = 0; i < distance.length; i++) {
                for (int j = 0; j < distance[0].length; j++) {
                    if (!visited[i][j] && distance[i][j] < min_val) {
                        min = new int[] { i, j };
                        min_val = distance[i][j];
                    }
                }
            }
            return min;
        }

        void dijkstra(int[][] maze, int[][] distance, boolean[][] visited) {
            int[][] dirs = { { 0, 1 }, { 0, -1 }, { -1, 0 }, { 1, 0 } };
            while (true) {
                int[] s = minDistance(distance, visited);
                if (s[0] < 0) {
                    break;
                }
                visited[s[0]][s[1]] = true;
                for (int[] dir : dirs) {
                    int x = s[0] + dir[0];
                    int y = s[1] + dir[1];
                    int count = 0;
                    while (x >= 0 && y >= 0 && x < maze.length && y < maze[0].length && maze[x][y] == 0) {
                        x += dir[0];
                        y += dir[1];
                        count++;
                    }
                    if (distance[s[0]][s[1]] + count < distance[x - dir[0]][y - dir[1]]) {
                        distance[x - dir[0]][y - dir[1]] = distance[s[0]][s[1]] + count;
                    }
                }
            }
        }
    }

    static class Solution4 {
        public int shortestDistance(int[][] maze, int[] start, int[] dest) {
            int[][] distance = new int[maze.length][maze[0].length];
            for (int[] row : distance) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            distance[start[0]][start[1]] = 0;
            dijkstra(maze, start, distance);
            return distance[dest[0]][dest[1]] == Integer.MAX_VALUE ? -1 : distance[dest[0]][dest[1]];
        }

        void dijkstra(int[][] maze, int[] start, int[][] distance) {
            int[][] dirs = { { 0, 1 }, { 0, -1 }, { -1, 0 }, { 1, 0 } };
            PriorityQueue<int[]> queue = new PriorityQueue<>((a, b) -> a[2] - b[2]);
            queue.offer(new int[] { start[0], start[1], 0 });
            while (!queue.isEmpty()) {
                int[] s = queue.poll();
                if (distance[s[0]][s[1]] < s[2]) {
                    continue;
                }

                for (int[] dir : dirs) {
                    int x = s[0] + dir[0];
                    int y = s[1] + dir[1];
                    int count = 0;
                    while (x >= 0 && y >= 0 && x < maze.length && y < maze[0].length && maze[x][y] == 0) {
                        x += dir[0];
                        y += dir[1];
                        count++;
                    }
                    if (distance[s[0]][s[1]] + count < distance[x - dir[0]][y - dir[1]]) {
                        distance[x - dir[0]][y - dir[1]] = distance[s[0]][s[1]] + count;
                        queue.offer(new int[] { x - dir[0], y - dir[1], distance[x - dir[0]][y - dir[1]] });
                    }
                }
            }
        }
    }
}
