package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.PriorityQueue;
import java.util.Stack;

public class Day602 {
    // https://leetcode.com/problems/crawler-log-folder/description/?envType=daily-question&envId=2024-07-10
    static class Solution1 {
        public int minOperations(String[] logs) {
            Stack<String> folderStack = new Stack<>();
            for (String currentOperation : logs) {
                if (currentOperation.equals("../")) {
                    if (!folderStack.isEmpty()) {
                        folderStack.pop();
                    }
                } else if (!currentOperation.equals("./")) {
                    folderStack.push(currentOperation);
                }
            }
            return folderStack.size();
        }
    }

    // https://leetcode.com/problems/min-cost-to-connect-all-points/description/
    static class UnionFind {
        public int[] group;
        public int[] rank;

        public UnionFind(int size) {
            group = new int[size];
            rank = new int[size];
            for (int i = 0; i < size; i++) {
                group[i] = i;
            }
        }

        public int find(int node) {
            if (group[node] != node) {
                group[node] = find(group[node]);
            }
            return group[node];
        }

        public boolean union(int node1, int node2) {
            int group1 = find(node1);
            int group2 = find(node2);
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

    static class Solution2 {
        public int minCostConnectPoints(int[][] points) {
            int n = points.length;
            ArrayList<int[]> allEdges = new ArrayList<>();
            for (int currNext = 0; currNext < n; ++currNext) {
                for (int nextNext = currNext + 1; nextNext < n; ++nextNext) {
                    int weight = Math.abs(points[currNext][0] - points[nextNext][0]) +
                            Math.abs(points[currNext][1] - points[nextNext][1]);
                    int[] currEdge = { weight, currNext, nextNext };
                    allEdges.add(currEdge);
                }
            }

            Collections.sort(allEdges, (a, b) -> Integer.compare(a[0], b[0]));

            UnionFind uf = new UnionFind(n);
            int mstCost = 0;
            int edgesUsed = 0;

            for (int i = 0; i < allEdges.size() && edgesUsed < n - 1; i++) {
                int node1 = allEdges.get(i)[1];
                int node2 = allEdges.get(i)[2];
                int weight = allEdges.get(i)[0];

                if (uf.union(node1, node2)) {
                    mstCost += weight;
                    edgesUsed++;
                }
            }

            return mstCost;
        }
    }

    static class Solution3 {
        public int minCostConnectPoints(int[][] points) {
            int n = points.length;
            PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> a[0] - b[0]);
            boolean[] inMST = new boolean[n];
            heap.add(new int[] { 0, 0 });
            int mstCost = 0;
            int edgesUsed = 0;

            while (edgesUsed < n) {
                int[] topElement = heap.poll();
                int weight = topElement[0];
                int currNode = topElement[1];

                if (inMST[currNode]) {
                    continue;
                }

                inMST[currNode] = true;
                mstCost += weight;
                edgesUsed++;

                for (int nextNode = 0; nextNode < n; ++nextNode) {
                    if (!inMST[nextNode]) {
                        int nextWeight = Math.abs(points[currNode][0] - points[nextNode][0]) +
                                Math.abs(points[currNode][1] - points[nextNode][1]);
                        heap.add(new int[] { nextWeight, nextNode });
                    }
                }
            }

            return mstCost;
        }
    }

    static class Solution4 {
        public int minCostConnectPoints(int[][] points) {
            int n = points.length;
            int mstCost = 0;
            int edgesUsed = 0;

            boolean[] inMST = new boolean[n];
            int[] minDist = new int[n];
            minDist[0] = 0;

            for (int i = 1; i < n; i++) {
                minDist[i] = Integer.MAX_VALUE;
            }

            while (edgesUsed < n) {
                int currMinEdge = Integer.MAX_VALUE;
                int currNode = -1;

                for (int node = 0; node < n; ++node) {
                    if (!inMST[node] && currMinEdge > minDist[node]) {
                        currMinEdge = minDist[node];
                        currNode = node;
                    }
                }

                mstCost += currMinEdge;
                edgesUsed++;
                inMST[currNode] = true;

                for (int nextNode = 0; nextNode < n; ++nextNode) {
                    int weight = Math.abs(points[currNode][0] - points[nextNode][0]) +
                            Math.abs(points[currNode][1] - points[nextNode][1]);

                    if (!inMST[nextNode] && minDist[nextNode] > weight) {
                        minDist[nextNode] = weight;
                    }
                }
            }
            return mstCost;
        }
    }
}
