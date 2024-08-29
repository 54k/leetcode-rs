package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day651 {
    static class Pair<F, S> {
        F key;
        S value;

        Pair(F f, S s) {
            key = f;
            value = s;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/path-sum-iv/description/?envType=weekly-question&envId=2024-08-29
    static class Solution1 {
        Map<Integer, Integer> map = new HashMap<>();

        public int pathSum(int[] nums) {
            if (nums == null || nums.length == 0)
                return 0;
            for (int num : nums) {
                int key = num / 10;
                int value = num % 10;
                map.put(key, value);
            }
            return dfs(nums[0] / 10, 0);
        }

        int dfs(int root, int preSum) {
            int level = root / 10;
            int pos = root % 10;

            int left = (level + 1) * 10 + pos * 2 - 1;
            int right = (level + 1) * 10 + pos * 2;
            int currSum = preSum + map.get(root);

            if (!map.containsKey(left) && !map.containsKey(right)) {
                return currSum;
            }

            int leftSum = map.containsKey(left) ? dfs(left, currSum) : 0;
            int rightSum = map.containsKey(right) ? dfs(right, currSum) : 0;

            return leftSum + rightSum;
        }
    }

    static class Solution2 {
        public int pathSum(int[] nums) {
            if (nums.length == 0) {
                return 0;
            }

            Map<Integer, Integer> map = new HashMap<>();
            for (int element : nums) {
                int coordinates = element / 10;
                int value = element % 10;
                map.put(coordinates, value);
            }

            Queue<Pair<Integer, Integer>> q = new LinkedList<>();
            int totalSum = 0;

            int rootCoordinates = nums[0] / 10;
            q.add(
                    new Pair<>(rootCoordinates, map.get(rootCoordinates)));
            while (!q.isEmpty()) {
                Pair<Integer, Integer> current = q.poll();
                int coordinates = current.getKey();
                int currentSum = current.getValue();
                int level = coordinates / 10;
                int position = coordinates % 10;

                int left = (level + 1) * 10 + position * 2 - 1;
                int right = (level + 1) * 10 + position * 2;

                if (!map.containsKey(left) && !map.containsKey(right)) {
                    totalSum += currentSum;
                }

                if (map.containsKey(left)) {
                    q.add(
                            new Pair<>(left, currentSum + map.get(left)));
                }
                if (map.containsKey(right)) {
                    q.add(
                            new Pair<>(right, currentSum + map.get(right)));
                }
            }
            return totalSum;
        }
    }

    // https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/?envType=daily-question&envId=2024-08-29
    static class Solution3 {
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

        void depthFirstSearch(
                List<Integer>[] adjacencyList,
                boolean[] visited,
                int stone) {
            visited[stone] = true;

            for (int neighbor : adjacencyList[stone]) {
                if (!visited[neighbor]) {
                    depthFirstSearch(adjacencyList, visited, neighbor);
                }
            }
        }
    }

    static class Solution4 {
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
    }

    static class Solution5 {
        public int removeStones(int[][] stones) {
            int n = stones.length;
            UnionFind uf = new UnionFind(20002);
            for (int i = 0; i < n; i++) {
                uf.union(stones[i][0], stones[i][1] + 10001);
            }
            return n - uf.componentCount;
        }

        static class UnionFind {
            int[] parent;
            int componentCount;
            Set<Integer> uniqueNodes;

            UnionFind(int n) {
                parent = new int[n];
                Arrays.fill(parent, -1);
                componentCount = 0;
                uniqueNodes = new HashSet<>();
            }

            int find(int node) {
                if (!uniqueNodes.contains(node)) {
                    componentCount++;
                    uniqueNodes.add(node);
                }

                if (parent[node] == -1) {
                    return node;
                }
                return parent[node] = find(parent[node]);
            }

            void union(int node1, int node2) {
                int root1 = find(node1);
                int root2 = find(node2);

                if (root1 == root2) {
                    return;
                }

                parent[root1] = root2;
                componentCount--;
            }
        }
    }
}
