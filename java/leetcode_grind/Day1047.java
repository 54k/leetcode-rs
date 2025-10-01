package leetcode_grind;

import java.util.Arrays;

public class Day1047 {
    // https://leetcode.com/problems/water-bottles/description/?envType=daily-question&envId=2025-10-01
    static class Solution1 {
        public int numWaterBottles(int numBottles, int numExchange) {
            int consumedBottles = 0;
            while (numBottles >= numExchange) {
                int K = numBottles / numExchange;
                consumedBottles += numExchange * K;
                numBottles -= numExchange * K;
                numBottles += K;
            }
            return consumedBottles + numBottles;
        }
    }

    static class Solution2 {
        static class DisjoinSet {
            int[] weights;
            int[] parents;

            DisjoinSet(int N) {
                this.parents = new int[N + 1];
                this.weights = new int[N + 1];

                for (int i = 1; i <= N; i++) {
                    this.parents[i] = i;
                    this.weights[i] = 1;
                }
            }

            void Union(int a, int b) {
                int rootA = Find(a);
                int rootB = Find(b);

                if (rootA == rootB) {
                    return;
                }

                if (this.weights[rootA] > this.weights[rootB]) {
                    this.parents[rootB] = rootA;
                    this.weights[rootA] += this.weights[rootB];
                } else {
                    this.parents[rootA] = rootB;
                    this.weights[rootB] += this.weights[rootA];
                }
            }

            int Find(int a) {
                while (a != this.parents[a]) {
                    this.parents[a] = this.parents[parents[a]];
                    a = this.parents[a];
                }
                return a;
            }

            boolean isInSameGroup(int a, int b) {
                return Find(a) == Find(b);
            }
        }

        public int minimumCost(int n, int[][] connections) {
            DisjoinSet disjoinSet = new DisjoinSet(n);
            Arrays.sort(connections, (a, b) -> a[2] - b[2]);
            int total = 0;
            int cost = 0;

            for (int i = 0; i < connections.length; i++) {
                int a = connections[i][0];
                int b = connections[i][1];

                if (disjoinSet.isInSameGroup(a, b))
                    continue;

                disjoinSet.Union(a, b);
                cost += connections[i][2];
                total++;
            }

            if (total == n - 1) {
                return cost;
            }
            return -1;
        }
    }

}
