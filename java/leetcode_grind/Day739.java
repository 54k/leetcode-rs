package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;
import java.util.TreeMap;

public class Day739 {
    // https://leetcode.com/problems/sliding-puzzle/description/?envType=daily-question&envId=2024-11-25
    static class Solution1 {
        final int[][] directions = {
                { 1, 3 },
                { 0, 2, 4 },
                { 1, 5 },
                { 0, 4 },
                { 3, 5, 1 },
                { 4, 2 }
        };

        public int slidingPuzzle(int[][] board) {
            StringBuilder startState = new StringBuilder();
            for (int i = 0; i < 2; i++) {
                for (int j = 0; j < 3; j++) {
                    startState.append(board[i][j]);
                }
            }

            Map<String, Integer> visited = new HashMap<>();
            dfs(startState.toString(), visited, startState.indexOf("0"), 0);
            return visited.getOrDefault("123450", -1);
        }

        void dfs(String state, Map<String, Integer> visited, int zeroPos, int moves) {
            if (visited.containsKey(state) && visited.get(state) <= moves) {
                return;
            }

            visited.put(state, moves);

            for (int nextPos : directions[zeroPos]) {
                String newState = swap(state, zeroPos, nextPos);
                dfs(newState, visited, nextPos, moves + 1);
            }
        }

        String swap(String str, int i, int j) {
            StringBuilder sb = new StringBuilder(str);
            sb.setCharAt(i, str.charAt(j));
            sb.setCharAt(j, str.charAt(i));
            return sb.toString();
        }
    }

    static class Solution2 {
        final int[][] directions = {
                { 1, 3 },
                { 0, 2, 4 },
                { 1, 5 },
                { 0, 4 },
                { 3, 5, 1 },
                { 4, 2 }
        };

        public int slidingPuzzle(int[][] board) {
            String target = "123450";
            StringBuilder startState = new StringBuilder();
            for (int i = 0; i < board.length; i++) {
                for (int j = 0; j < board[0].length; j++) {
                    startState.append(board[i][j]);
                }
            }

            Set<String> visited = new HashSet<>();
            Queue<String> queue = new LinkedList<>();
            queue.add(startState.toString());
            visited.add(startState.toString());

            int moves = 0;

            while (!queue.isEmpty()) {
                int size = queue.size();
                while (size-- > 0) {
                    String currentState = queue.poll();

                    if (currentState.equals(target)) {
                        return moves;
                    }

                    int zeroPos = currentState.indexOf('0');

                    for (int newPos : directions[zeroPos]) {
                        String nextState = swap(currentState, zeroPos, newPos);

                        if (visited.contains(nextState))
                            continue;

                        visited.add(nextState);
                        queue.add(nextState);
                    }
                }
                moves++;
            }

            return -1;
        }

        String swap(String str, int i, int j) {
            StringBuilder sb = new StringBuilder(str);
            sb.setCharAt(i, str.charAt(j));
            sb.setCharAt(j, str.charAt(i));
            return sb.toString();
        }
    }

    // https://leetcode.com/problems/two-city-scheduling/description/
    static class Solution3 {
        public int twoCitySchedCost(int[][] costs) {
            Arrays.sort(costs, new Comparator<int[]>() {
                @Override
                public int compare(int[] o1, int[] o2) {
                    return o1[0] - o1[1] - (o2[0] - o2[1]);
                }
            });

            int total = 0;
            int n = costs.length / 2;

            for (int i = 0; i < n; i++)
                total += costs[i][0] + costs[i + n][1];
            return total;
        }
    }

    // https://leetcode.com/problems/rank-transform-of-a-matrix/description/
    static class Solution4 {
        public int[][] matrixRankTransform(int[][] matrix) {
            int m = matrix.length;
            int n = matrix[0].length;

            Map<Integer, Map<Integer, List<Integer>>> graphs = new HashMap<>();
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    int v = matrix[i][j];

                    if (!graphs.containsKey(v)) {
                        graphs.put(v, new HashMap<Integer, List<Integer>>());
                    }

                    Map<Integer, List<Integer>> graph = graphs.get(v);
                    if (!graph.containsKey(i)) {
                        graph.put(i, new ArrayList<Integer>());
                    }
                    if (!graph.containsKey(~j)) {
                        graph.put(~j, new ArrayList<Integer>());
                    }

                    graph.get(i).add(~j);
                    graph.get(~j).add(i);
                }
            }

            TreeMap<Integer, List<List<int[]>>> value2index = new TreeMap<>();

            int[][] seen = new int[m][n];

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (seen[i][j] == 1) {
                        continue;
                    }
                    seen[i][j] = 1;
                    int v = matrix[i][j];
                    Map<Integer, List<Integer>> graph = graphs.get(v);

                    Set<Integer> rowcols = new HashSet<Integer>();
                    rowcols.add(i);
                    rowcols.add(~j);

                    Queue<Integer> q = new LinkedList<>();
                    q.offer(i);
                    q.offer(~j);

                    while (!q.isEmpty()) {
                        int node = q.poll();
                        for (int rowcol : graph.get(node)) {
                            if (!rowcols.contains(rowcol)) {
                                rowcols.add(rowcol);
                                q.offer(rowcol);
                            }
                        }
                    }

                    List<int[]> points = new ArrayList<>();
                    for (int rowcol : rowcols) {
                        for (int k : graph.get(rowcol)) {
                            if (k >= 0) {
                                points.add(new int[] { k, ~rowcol });
                                seen[k][~rowcol] = 1;
                            } else {
                                points.add(new int[] { rowcol, ~k });
                                seen[rowcol][~k] = 1;
                            }
                        }
                    }
                    if (!value2index.containsKey(v)) {
                        value2index.put(v, new ArrayList<List<int[]>>());
                    }
                    value2index.get(v).add(points);
                }
            }

            int[][] answer = new int[m][n];
            int[] rowMax = new int[m];
            int[] colMax = new int[n];

            for (int v : value2index.keySet()) {
                for (List<int[]> points : value2index.get(v)) {
                    int rank = 1;
                    for (int[] point : points) {
                        rank = Math.max(rank, Math.max(rowMax[point[0]], colMax[point[1]]) + 1);
                    }

                    for (int[] point : points) {
                        answer[point[0]][point[1]] = rank;
                        rowMax[point[0]] = Math.max(rowMax[point[0]], rank);
                        colMax[point[1]] = Math.max(colMax[point[1]], rank);
                    }
                }
            }

            return answer;
        }
    }
}
