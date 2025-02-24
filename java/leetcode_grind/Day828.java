package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class Day828 {
    // https://leetcode.com/problems/most-profitable-path-in-a-tree/description/?envType=daily-question&envId=2025-02-24
    static class Solution1 {
        Map<Integer, Integer> bobPath;
        boolean[] visited;
        List<List<Integer>> tree;

        public int mostProfitablePath(int[][] edges, int bob, int[] amount) {
            int n = amount.length, maxIncome = Integer.MIN_VALUE;
            tree = new ArrayList<>();
            bobPath = new HashMap<>();
            visited = new boolean[n];

            Queue<int[]> nodeQueue = new LinkedList<>();
            nodeQueue.add(new int[] { 0, 0, 0 });

            for (int i = 0; i < n; i++) {
                tree.add(new ArrayList<>());
            }

            for (int[] edge : edges) {
                tree.get(edge[0]).add(edge[1]);
                tree.get(edge[1]).add(edge[0]);
            }

            findBobPath(bob, 0);

            Arrays.fill(visited, false);
            while (!nodeQueue.isEmpty()) {
                int[] node = nodeQueue.poll();
                int sourceNode = node[0], time = node[1], income = node[2];

                if (!bobPath.containsKey(sourceNode) || time < bobPath.get(sourceNode)) {
                    income += amount[sourceNode];
                } else if (time == bobPath.get(sourceNode)) {
                    income += amount[sourceNode] / 2;
                }

                if (tree.get(sourceNode).size() == 1 && sourceNode != 0) {
                    maxIncome = Math.max(maxIncome, income);
                }

                for (int adjacentNode : tree.get(sourceNode)) {
                    if (!visited[adjacentNode]) {
                        nodeQueue.add(new int[] { adjacentNode, time + 1, income });
                    }
                }

                visited[sourceNode] = true;
            }
            return maxIncome;
        }

        boolean findBobPath(int sourceNode, int time) {
            bobPath.put(sourceNode, time);
            visited[sourceNode] = true;

            if (sourceNode == 0) {
                return true;
            }

            for (int adjacentNode : tree.get(sourceNode)) {
                if (!visited[adjacentNode]) {
                    if (findBobPath(adjacentNode, time + 1)) {
                        return true;
                    }
                }
            }

            bobPath.remove(sourceNode);
            return false;
        }
    }

    static class Solution2 {
        Map<Integer, Integer> bobPath;
        boolean[] visited;
        List<List<Integer>> tree;
        int maxIncome = Integer.MIN_VALUE;

        public int mostProfitablePath(int[][] edges, int bob, int[] amount) {
            int n = amount.length;
            tree = new ArrayList<>();
            bobPath = new HashMap<>();
            visited = new boolean[n];

            for (int i = 0; i < n; i++) {
                tree.add(new ArrayList<>());
            }

            for (int[] edge : edges) {
                tree.get(edge[0]).add(edge[1]);
                tree.get(edge[1]).add(edge[0]);
            }

            findBobPath(bob, 0);

            Arrays.fill(visited, false);
            findAlicePath(0, 0, 0, amount);
            return maxIncome;
        }

        boolean findBobPath(int sourceNode, int time) {
            bobPath.put(sourceNode, time);
            visited[sourceNode] = true;

            if (sourceNode == 0) {
                return true;
            }

            for (int adjacentNode : tree.get(sourceNode)) {
                if (!visited[adjacentNode]) {
                    if (findBobPath(adjacentNode, time + 1)) {
                        return true;
                    }
                }
            }

            bobPath.remove(sourceNode);
            return false;
        }

        void findAlicePath(int sourceNode, int time, int income, int[] amount) {
            visited[sourceNode] = true;
            if (!bobPath.containsKey(sourceNode) || time < bobPath.get(sourceNode)) {
                income += amount[sourceNode];
            } else if (time == bobPath.get(sourceNode)) {
                income += amount[sourceNode] / 2;
            }

            if (tree.get(sourceNode).size() == 1 && sourceNode != 0) {
                maxIncome = Math.max(maxIncome, income);
            }

            for (int adjacentNode : tree.get(sourceNode)) {
                if (!visited[adjacentNode]) {
                    findAlicePath(adjacentNode, time + 1, income, amount);
                }
            }
        }
    }

    static class Solution3 {
        List<List<Integer>> tree;
        int[] distanceFromBob;
        int n;

        public int mostProfitablePath(int[][] edges, int bob, int[] amount) {
            n = amount.length;
            tree = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                tree.add(new ArrayList<>());
            }
            distanceFromBob = new int[n];
            for (int[] edge : edges) {
                tree.get(edge[0]).add(edge[1]);
                tree.get(edge[1]).add(edge[0]);
            }
            return findPaths(0, 0, 0, bob, amount);
        }

        int findPaths(
                int sourceNode,
                int parentNode,
                int time,
                int bob,
                int[] amount) {
            int maxIncome = 0, maxChild = Integer.MIN_VALUE;

            if (sourceNode == bob) {
                distanceFromBob[sourceNode] = 0;
            } else {
                distanceFromBob[sourceNode] = n;
            }

            for (int adjacentNode : tree.get(sourceNode)) {
                if (adjacentNode != parentNode) {
                    maxChild = Math.max(maxChild, findPaths(adjacentNode, sourceNode, time + 1, bob, amount));
                    distanceFromBob[sourceNode] = Math.min(distanceFromBob[sourceNode],
                            distanceFromBob[adjacentNode] + 1);
                }
            }

            if (distanceFromBob[sourceNode] > time) {
                maxIncome += amount[sourceNode];
            } else if (distanceFromBob[sourceNode] == time) {
                maxIncome += amount[sourceNode] / 2;
            }

            return (maxChild == Integer.MIN_VALUE) ? maxIncome : maxIncome + maxChild;
        }
    }

    // https://leetcode.com/problems/max-consecutive-ones-ii/description/
    static class Solution4 {
        public int findMaxConsecutiveOnes(int[] nums) {
            var z = 0;
            var ans = 0;
            for (int i = 0, j = 0; i < nums.length; i++) {
                z += (nums[i] == 0 ? 1 : 0);
                while (z > 1) {
                    z -= (nums[j] == 0 ? 1 : 0);
                    j++;
                }
                ans = Math.max(i - j + 1, ans);
            }
            return ans;
        }
    }
}
