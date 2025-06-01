package leetcode_grind;

import java.util.Arrays;
import java.util.Collections;
import java.util.LinkedList;
import java.util.Queue;

public class Day924 {
    // https://leetcode.com/problems/snakes-and-ladders/description/?envType=daily-question&envId=2025-05-31
    static class Solution1 {
        public int snakesAndLadders(int[][] board) {
            int n = board.length;
            Pair<Integer, Integer>[] cells = new Pair[n * n + 1];
            int label = 1;
            Integer[] columns = new Integer[n];
            for (int i = 0; i < n; i++) {
                columns[i] = i;
            }
            for (int row = n - 1; row >= 0; row--) {
                for (int column : columns) {
                    cells[label++] = new Pair<>(row, column);
                }
                Collections.reverse(Arrays.asList(columns));
            }
            int[] dist = new int[n * n + 1];
            Arrays.fill(dist, -1);
            Queue<Integer> q = new LinkedList<Integer>();
            dist[1] = 0;
            q.add(1);
            while (!q.isEmpty()) {
                int curr = q.remove();
                for (int next = curr + 1; next <= Math.min(curr + 6, n * n); next++) {
                    int row = cells[next].getKey(), column = cells[next].getValue();
                    int destination = board[row][column] != -1 ? board[row][column] : next;
                    if (dist[destination] == -1) {
                        dist[destination] = dist[curr] + 1;
                        q.add(destination);
                    }
                }
            }
            return dist[n * n];
        }
    }

    static class Solution2 {
        public int snakesAndLadders(int[][] board) {
            int n = board.length;
            Pair<Integer, Integer>[] cells = new Pair[n * n + 1];
            int label = 1;
            Integer[] columns = new Integer[n];
            for (int i = 0; i < n; i++) {
                columns[i] = i;
            }
            for (int row = n - 1; row >= 0; row--) {
                for (int column : columns) {
                    cells[label++] = new Pair<>(row, column);
                }
                Collections.reverse(Arrays.asList(columns));
            }
            int[] dist = new int[n * n + 1];
            Arrays.fill(dist, -1);

            class Vertex implements Comparable<Vertex> {
                int distance, label;

                public Vertex(int distance, int label) {
                    this.distance = distance;
                    this.label = label;
                }

                public int compareTo(Vertex v) {
                    return this.distance - v.distance;
                }
            }

            PriorityQueue<Vertex> q = new PriorityQueue<>();
            dist[1] = 0;
            q.add(new Vertex(0, 1));
            while (!q.isEmpty()) {
                Vertex vertex = q.remove();
                int d = vertex.distance, curr = vertex.label;
                if (d != dist[curr]) {
                    continue;
                }

                for (int next = curr + 1; next <= Math.min(curr + 6, n * n); next++) {
                    int row = cells[next].getKey(), column = cells[next].getValue();
                    int destination = board[row][column] != -1 ? board[row][column] : next;
                    if (dist[destination] == -1 || dist[curr] + 1 < dist[destination]) {
                        dist[destination] = dist[curr] + 1;
                        q.add(new Vertex(dist[destination], destination));
                    }
                }
            }
            return dist[n * n];
        }
    }

}
