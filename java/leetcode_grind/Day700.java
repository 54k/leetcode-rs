package leetcode_grind;

import java.util.*;

public class Day700 {
    // https://leetcode.com/problems/longest-happy-string/description/?envType=daily-question&envId=2024-10-16
    static class Solution1 {
        public String longestDiverseString(int a, int b, int c) {
            var sb = new StringBuilder();
            var pq = new PriorityQueue<int[]>((x, y) -> y[0] - x[0]);

            pq.add(new int[] { a, 'a' });
            pq.add(new int[] { b, 'b' });
            pq.add(new int[] { c, 'c' });
            int[] f;
            int[] s;

            while (true) {
                f = pq.poll();
                s = pq.poll();
                if (sb.length() == 0 || (sb.length() > 0 && sb.charAt(sb.length() - 1) != f[1])) {
                    for (var i = 0; i < f[0] && i < 2; i++) {
                        sb.append((char) f[1]);
                        f[0]--;
                    }
                }

                if (s[0] == 0) {
                    break;
                }

                sb.append((char) s[1]);
                s[0]--;
                pq.add(f);
                pq.add(s);
            }

            return sb.toString();
        }
    }

    static class Solution2 {
        static class Pair {
            int count;
            char character;

            Pair(int count, char character) {
                this.count = count;
                this.character = character;
            }
        }

        public String longestDiverseString(int a, int b, int c) {
            var pq = new PriorityQueue<Pair>((x, y) -> (y.count - x.count));
            if (a > 0) {
                pq.add(new Pair(a, 'a'));
            }
            if (b > 0) {
                pq.add(new Pair(b, 'b'));
            }
            if (c > 0) {
                pq.add(new Pair(c, 'c'));
            }
            StringBuilder ans = new StringBuilder();
            while (!pq.isEmpty()) {
                Pair p = pq.poll();
                int count = p.count;
                char character = p.character;

                if (ans.length() >= 2 && ans.charAt(ans.length() - 1) == p.character
                        && ans.charAt(ans.length() - 2) == p.character) {
                    if (pq.isEmpty())
                        break;
                    Pair temp = pq.poll();
                    ans.append(temp.character);
                    if (temp.count - 1 > 0) {
                        pq.add(new Pair(temp.count - 1, temp.character));
                    }
                } else {
                    count--;
                    ans.append(character);
                }
                if (count > 0) {
                    pq.add(new Pair(count, character));
                }
            }
            return ans.toString();
        }
    }

    static class Solution3 {
        public String longestDiverseString(int a, int b, int c) {
            int curra = 0, currb = 0, currc = 0;
            int totalIterations = a + b + c;
            StringBuilder ans = new StringBuilder();
            for (int i = 0; i < totalIterations; i++) {
                if ((a >= b && a >= c && curra != 2) ||
                        (a > 0 && (currb == 2 || currc == 2))) {
                    ans.append('a');
                    a--;
                    curra++;
                    currb = 0;
                    currc = 0;
                } else if ((b >= a && b >= c && currb != 2) ||
                        (b > 0 && (currc == 2 || curra == 2))) {
                    ans.append('b');
                    b--;
                    currb++;
                    curra = 0;
                    currc = 0;
                } else if ((c >= a && c >= b && currc != 2) ||
                        (c > 0 && (curra == 2 || currb == 2))) {
                    ans.append('c');
                    c--;
                    currc++;
                    curra = 0;
                    currb = 0;
                }
            }
            return ans.toString();
        }
    }

    // https://leetcode.com/problems/the-maze-iii/description/
    static class Solution4 {
        static class State {
            int row;
            int col;
            int dist;
            String path;

            State(int row, int col, int dist, String path) {
                this.row = row;
                this.col = col;
                this.dist = dist;
                this.path = path;
            }
        }

        int[][] directions = new int[][] { { 0, -1 }, { -1, 0 }, { 0, 1 }, { 1, 0 } };
        String[] textDirections = new String[] { "l", "u", "r", "d" };
        int m;
        int n;

        public String findShortestWay(int[][] maze, int[] ball, int[] hole) {
            m = maze.length;
            n = maze[0].length;

            var heap = new PriorityQueue<State>((a, b) -> {
                int distA = a.dist;
                int distB = b.dist;
                if (distA == distB) {
                    return a.path.compareTo(b.path);
                }
                return distA - distB;
            });

            boolean[][] seen = new boolean[m][n];
            heap.add(new State(ball[0], ball[1], 0, ""));

            while (!heap.isEmpty()) {
                State curr = heap.remove();
                int row = curr.row;
                int col = curr.col;
                if (seen[row][col]) {
                    continue;
                }
                if (row == hole[0] && col == hole[1]) {
                    return curr.path;
                }
                seen[row][col] = true;

                for (State nextState : getNeighbors(row, col, maze, hole)) {
                    int nextRow = nextState.row;
                    int nextCol = nextState.col;
                    int nextDist = nextState.dist;
                    String nextChar = nextState.path;
                    heap.add(new State(nextRow, nextCol, curr.dist + nextDist, curr.path + nextChar));
                }
            }

            return "impossible";
        }

        boolean valid(int row, int col, int[][] maze) {
            if (row < 0 || row >= m || col < 0 || col >= n) {
                return false;
            }
            return maze[row][col] == 0;
        }

        List<State> getNeighbors(int row, int col, int[][] maze, int[] hole) {
            List<State> neighbors = new ArrayList<>();
            for (int i = 0; i < 4; i++) {
                int dy = directions[i][0];
                int dx = directions[i][1];
                String direction = textDirections[i];

                int currRow = row;
                int currCol = col;
                int dist = 0;

                while (valid(currRow + dy, currCol + dx, maze)) {
                    currRow += dy;
                    currCol += dx;
                    dist++;
                    if (currRow == hole[0] && currCol == hole[1]) {
                        break;
                    }
                }

                neighbors.add(new State(currRow, currCol, dist, direction));
            }
            return neighbors;
        }
    }

    // https://leetcode.com/problems/the-maze/description/
    static class Solution5 {
        boolean dfs(int m, int n, int[][] maze, int[] curr, int[] destination, boolean[][] visit) {
            if (visit[curr[0]][curr[1]]) {
                return false;
            }
            if (curr[0] == destination[0] && curr[1] == destination[1]) {
                return true;
            }

            visit[curr[0]][curr[1]] = true;
            int[] dirX = { 0, 1, 0, -1 };
            int[] dirY = { -1, 0, 1, 0 };

            for (int i = 0; i < 4; i++) {
                int r = curr[0], c = curr[1];
                while (r >= 0 && r < m && c >= 0 && c < n && maze[r][c] == 0) {
                    r += dirX[i];
                    c += dirY[i];
                }
                if (dfs(m, n, maze, new int[] { r - dirX[i], c - dirY[i] }, destination, visit)) {
                    return true;
                }
            }
            return false;
        }

        public boolean hasPath(int[][] maze, int[] start, int[] destination) {
            int m = maze.length;
            int n = maze[0].length;
            boolean[][] visit = new boolean[m][n];
            return dfs(m, n, maze, start, destination, visit);
        }
    }

    static class Solution6 {
        public boolean hasPath(int[][] maze, int[] start, int[] destination) {
            int m = maze.length;
            int n = maze[0].length;

            boolean[][] visit = new boolean[m][n];
            int[] dirX = { 0, 1, 0, -1 };
            int[] dirY = { -1, 0, 1, 0 };

            Queue<int[]> queue = new LinkedList<>();
            queue.offer(start);
            visit[start[0]][start[1]] = true;

            while (!queue.isEmpty()) {
                int[] curr = queue.poll();
                if (curr[0] == destination[0] && curr[1] == destination[1]) {
                    return true;
                }
                for (int i = 0; i < 4; i++) {
                    int r = curr[0];
                    int c = curr[1];
                    while (r >= 0 && r < m && c >= 0 && c < n && maze[r][c] == 0) {
                        r += dirX[i];
                        c += dirY[i];
                    }
                    r -= dirX[i];
                    c -= dirY[i];
                    if (!visit[r][c]) {
                        queue.offer(new int[] { r, c });
                        visit[r][c] = true;
                    }
                }
            }
            return false;
        }
    }

    // https://leetcode.com/problems/the-maze-ii/description/
    static class Solution7 {
        public int shortestDistance(int[][] maze, int[] start, int[] destination) {
            int[][] distance = new int[maze.length][maze[0].length];
            for (int[] row : distance) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            distance[start[0]][start[1]] = 0;
            dfs(maze, start, distance);
            return distance[destination[0]][destination[1]] == Integer.MAX_VALUE ? -1
                    : distance[destination[0]][destination[1]];
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

    static class Solution8 {
        public int shortestDistance(int[][] maze, int[] start, int[] destination) {
            int[][] distance = new int[maze.length][maze[0].length];
            boolean[][] visited = new boolean[maze.length][maze[0].length];
            for (int[] row : distance) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            distance[start[0]][start[1]] = 0;
            dijkstra(maze, distance, visited);
            return distance[destination[0]][destination[1]] == Integer.MAX_VALUE ? -1
                    : distance[destination[0]][destination[1]];
        }

        int[] minDistance(int[][] distance, boolean[][] visited) {
            int[] min = { -1, 1 };
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

    static class Solution9 {
        public int shortestDistance(int[][] maze, int[] start, int[] destination) {
            int[][] distance = new int[maze.length][maze[0].length];
            for (int[] row : distance) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            distance[start[0]][start[1]] = 0;
            dijkstra(maze, start, distance);
            return distance[destination[0]][destination[1]] == Integer.MAX_VALUE ? -1
                    : distance[destination[0]][destination[1]];
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

    // https://leetcode.com/problems/redundant-connection/description/
    static class Solution10 {
        Set<Integer> seen = new HashSet<>();
        int MAX_EDGE_VAL = 1000;

        public int[] findRedundantConnection(int[][] edges) {
            ArrayList<Integer>[] graph = new ArrayList[MAX_EDGE_VAL + 1];
            for (int i = 0; i <= MAX_EDGE_VAL; i++) {
                graph[i] = new ArrayList<>();
            }
            for (int[] edge : edges) {
                seen.clear();
                if (!graph[edge[0]].isEmpty() && !graph[edge[1]].isEmpty() && dfs(graph, edge[0], edge[1])) {
                    return edge;
                }
                graph[edge[0]].add(edge[1]);
                graph[edge[1]].add(edge[0]);
            }
            throw new Error("should never happen");
        }

        boolean dfs(ArrayList<Integer>[] graph, int source, int target) {
            if (!seen.contains(source)) {
                seen.add(source);
                if (source == target)
                    return true;
                for (int nei : graph[source]) {
                    if (dfs(graph, nei, target))
                        return true;
                }
            }
            return false;
        }
    }

    static class Solution11 {
        int MAX_EDGE_VAL = 1000;

        public int[] findRedundantConnection(int[][] edges) {
            DSU dsu = new DSU(MAX_EDGE_VAL + 1);
            for (int[] edge : edges) {
                if (!dsu.union(edge[0], edge[1]))
                    return edge;
            }
            throw new AssertionError();
        }

        static class DSU {
            int[] parent;
            int[] rank;

            DSU(int size) {
                parent = new int[size];
                for (int i = 0; i < size; i++)
                    parent[i] = i;
                rank = new int[size];
            }

            int find(int x) {
                if (parent[x] != x)
                    parent[x] = find(parent[x]);
                return parent[x];
            }

            boolean union(int x, int y) {
                int xr = find(x), yr = find(y);
                if (xr == yr) {
                    return false;
                } else if (rank[xr] < rank[yr]) {
                    parent[xr] = yr;
                } else if (rank[xr] > rank[yr]) {
                    parent[yr] = xr;
                } else {
                    parent[yr] = xr;
                    rank[xr]++;
                }
                return true;
            }
        }
    }
}