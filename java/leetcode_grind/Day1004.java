package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1004 {
    // https://leetcode.com/problems/number-of-zero-filled-subarrays/description/?envType=daily-question&envId=2025-08-19
    static class Solution1 {
        public long zeroFilledSubarray(int[] nums) {
            long ans = 0, numSubarray = 0;

            for (int num : nums) {
                if (num == 0) {
                    numSubarray++;
                } else {
                    numSubarray = 0;
                }
                ans += numSubarray;
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
    static class Solution2 {
        public int removeStones(int[][] stones) {
            int n = stones.length;
            List<Integer>[] adjacencyList = new List[n];
            for (int i = 0; i < n; i++) {
                adjacencyList[i] = new ArrayList<>();
            }

            for (int i = 0; i < n; i++) {
                for (int j = i + 1; j < n; j++) {
                    if (stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1]) {
                        adjacencyList[i].add(j);
                        adjacencyList[j].add(i);
                    }
                }
            }

            int numOfConnectedComponents = 0;
            boolean[] visited = new boolean[n];

            for (int i = 0; i < n; i++) {
                if (!visited[i]) {
                    depthFirstSearch(adjacencyList, visited, i);
                    numOfConnectedComponents++;
                }
            }

            return n - numOfConnectedComponents;
        }

        void depthFirstSearch(List<Integer>[] adjacenyList, boolean[] visited, int stone) {
            visited[stone] = true;
            for (int neighbor : adjacenyList[stone]) {
                if (!visited[neighbor]) {
                    depthFirstSearch(adjacenyList, visited, neighbor);
                }
            }
        }
    }

    static class Solution3 {
        public int removeStones(int[][] stones) {
            int n = stones.length;
            UnionFind uf = new UnionFind(n);

            for (int i = 0; i < n; i++) {
                for (int j = i + 1; j < n; j++) {
                    if (stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1]) {
                        uf.union(i, j);
                    }
                }
            }

            return n - uf.count;
        }

        static class UnionFind {
            int[] parent;
            int count;

            UnionFind(int n) {
                parent = new int[n];
                Arrays.fill(parent, -1);
                count = n;
            }

            int find(int node) {
                if (parent[node] == -1) {
                    return node;
                }
                return parent[node] = find(parent[node]);
            }

            void union(int n1, int n2) {
                int root1 = find(n1);
                int root2 = find(n2);

                if (root1 == root2) {
                    return;
                }

                count--;
                parent[root1] = root2;
            }
        }
    }

    static class Solution4 {
        public int removeStones(int[][] stones) {
            int n = stones.length;
            UnionFind uf = new UnionFind(20002);

            for (int i = 0; i < n; i++) {
                uf.union(stones[i][0], stones[i][1] + 10001);
            }

            return n - uf.count;
        }

        static class UnionFind {
            int[] parent;
            int count;
            Set<Integer> uniqueNodes;

            UnionFind(int n) {
                parent = new int[n];
                Arrays.fill(parent, -1);
                count = 0;
                uniqueNodes = new HashSet<>();
            }

            int find(int node) {
                if (!uniqueNodes.contains(node)) {
                    count++;
                    uniqueNodes.add(node);
                }

                if (parent[node] == -1) {
                    return node;
                }
                return parent[node] = find(parent[node]);
            }

            void union(int n1, int n2) {
                int root1 = find(n1);
                int root2 = find(n2);

                if (root1 == root2) {
                    return;
                }

                count--;
                parent[root1] = root2;
            }
        }
    }
}
