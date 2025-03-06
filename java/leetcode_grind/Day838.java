package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Set;

public class Day838 {
    // https://leetcode.com/problems/find-missing-and-repeated-values/description/
    static class Solution1 {
        public int[] findMissingAndRepeatedValues(int[][] grid) {
            long sum = 0, sqrSum = 0;
            long n = grid.length;
            long total = n * n;

            for (int row = 0; row < n; row++) {
                for (int col = 0; col < n; col++) {
                    sum += grid[row][col];
                    sqrSum += (long) grid[row][col] * grid[row][col];
                }
            }

            long sumDiff = sum - (total * (total + 1)) / 2;
            long sqrDiff = sqrSum - (total * (total + 1) * (2 * total + 1)) / 6;

            int repeating = (int) (sqrDiff / sumDiff + sumDiff) / 2;
            int missing = (int) (sqrDiff / sumDiff - sumDiff) / 2;
            return new int[] { repeating, missing };
        }
    }

    // https://leetcode.com/problems/design-tic-tac-toe/description/
    static class TicTacToe {
        int[] rows;
        int[] cols;
        int diagonal;
        int antiDiagonal;

        public TicTacToe(int n) {
            rows = new int[n];
            cols = new int[n];
        }

        public int move(int row, int col, int player) {
            int currentPlayer = (player == 1) ? 1 : -1;
            rows[row] += currentPlayer;
            cols[col] += currentPlayer;
            if (row == col) {
                diagonal += currentPlayer;
            }
            if (col == (cols.length - row - 1)) {
                antiDiagonal += currentPlayer;
            }
            int n = rows.length;
            if (Math.abs(rows[row]) == n || Math.abs(cols[col]) == n || Math.abs(diagonal) == n
                    || Math.abs(antiDiagonal) == n) {
                return player;
            }
            return 0;
        }
    }

    // https://leetcode.com/problems/valid-tic-tac-toe-state/description/
    static class Solution2 {
        public boolean validTicTacToe(String[] board) {
            int xCount = 0, oCount = 0;
            for (String row : board) {
                for (char c : row.toCharArray()) {
                    if (c == 'X')
                        xCount++;
                    if (c == 'O')
                        oCount++;
                }
            }

            if (oCount != xCount && oCount != xCount - 1)
                return false;
            if (win(board, 'X') && oCount != xCount - 1)
                return false;
            if (win(board, 'O') && oCount != xCount)
                return false;
            return true;
        }

        boolean win(String[] B, char P) {
            for (int i = 0; i < 3; i++) {
                if (P == B[0].charAt(i) && P == B[1].charAt(i) && P == B[2].charAt(i)) {
                    return true;
                }
                if (P == B[i].charAt(0) && P == B[i].charAt(1) && P == B[i].charAt(2)) {
                    return true;
                }
            }
            if (P == B[0].charAt(0) && P == B[1].charAt(1) && P == B[2].charAt(2)) {
                return true;
            }
            if (P == B[0].charAt(2) && P == B[1].charAt(1) && P == B[2].charAt(0)) {
                return true;
            }
            return false;
        }
    }

    static class Pair<K, V> {
        K key;
        V value;

        Pair(K k, V v) {
            key = k;
            value = v;
        }

        K getKey() {
            return key;
        }

        V getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/optimize-water-distribution-in-a-village/description/
    static class Solution3 {
        public int minCostToSupplyWater(int n, int[] wells, int[][] pipes) {
            PriorityQueue<Pair<Integer, Integer>> edgesHeap = new PriorityQueue<>(n, (a, b) -> a.getKey() - b.getKey());

            List<List<Pair<Integer, Integer>>> graph = new ArrayList<>(n + 1);
            for (int i = 0; i < n + 1; i++) {
                graph.add(new ArrayList<>());
            }
            for (int i = 0; i < wells.length; i++) {
                Pair<Integer, Integer> virtualEdge = new Pair<>(wells[i], i + 1);
                graph.get(0).add(virtualEdge);
                edgesHeap.add(virtualEdge);
            }

            for (int i = 0; i < pipes.length; i++) {
                int house1 = pipes[i][0];
                int house2 = pipes[i][1];
                int cost = pipes[i][2];
                graph.get(house1).add(new Pair<>(cost, house2));
                graph.get(house2).add(new Pair<>(cost, house1));
            }

            Set<Integer> mstSet = new HashSet<>();
            mstSet.add(0);

            int totalCost = 0;
            while (mstSet.size() < n + 1) {
                Pair<Integer, Integer> edge = edgesHeap.poll();
                int cost = edge.getKey();
                int nextHouse = edge.getValue();
                if (mstSet.contains(nextHouse)) {
                    continue;
                }
                mstSet.add(nextHouse);
                totalCost += cost;
                for (Pair<Integer, Integer> neighborEdge : graph.get(nextHouse)) {
                    if (!mstSet.contains(neighborEdge.getValue())) {
                        edgesHeap.add(neighborEdge);
                    }
                }
            }
            return totalCost;
        }
    }

    static class UnionFind {
        int[] group;
        int[] rank;

        UnionFind(int size) {
            group = new int[size + 1];
            rank = new int[size + 1];
            for (int i = 0; i < size + 1; i++) {
                group[i] = i;
                rank[i] = 0;
            }
        }

        int find(int person) {
            if (group[person] != person) {
                group[person] = find(group[person]);
            }
            return group[person];
        }

        boolean union(int person1, int person2) {
            int group1 = find(person1);
            int group2 = find(person2);

            if (group1 == group2) {
                return false;
            }

            if (rank[group1] > rank[group2]) {
                group[group2] = group1;
            } else if (rank[group1] < rank[group2]) {
                group[group1] = group2;
            } else {
                group[group1] = group2;
                rank[group2] += 1;
            }
            return true;
        }
    }

    static class Solution4 {
        public int minCostToSupplyWater(int n, int[] wells, int[][] pipes) {
            List<int[]> orderedEdges = new ArrayList<>(n + 1 + pipes.length);
            for (int i = 0; i < wells.length; i++) {
                orderedEdges.add(new int[] { 0, i + 1, wells[i] });
            }
            for (int i = 0; i < pipes.length; i++) {
                int[] edge = pipes[i];
                orderedEdges.add(edge);
            }
            Collections.sort(orderedEdges, (a, b) -> a[2] - b[2]);

            UnionFind uf = new UnionFind(n);
            int totalCost = 0;
            for (int[] edge : orderedEdges) {
                int house1 = edge[0];
                int house2 = edge[1];
                int cost = edge[2];
                if (uf.union(house1, house2)) {
                    totalCost += cost;
                }
            }
            return totalCost;
        }
    }
}
