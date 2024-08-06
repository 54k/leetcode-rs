package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Random;
import java.util.Set;

public class Day628 {
    // https://leetcode.com/problems/tree-diameter/description/
    static class Solution1 {
        public int treeDiameter(int[][] edges) {
            List<Set<Integer>> graph = new ArrayList<Set<Integer>>();
            for (int i = 0; i < edges.length + 1; ++i) {
                graph.add(new HashSet<Integer>());
            }
            for (int[] edge : edges) {
                Integer u = edge[0], v = edge[1];
                graph.get(u).add(v);
                graph.get(v).add(u);
            }
            int[] nodeDistance = bfs(0, graph);
            nodeDistance = bfs(nodeDistance[0], graph);
            return nodeDistance[1];
        }

        int[] bfs(int start, List<Set<Integer>> graph) {
            boolean[] visited = new boolean[graph.size()];
            visited[start] = true;
            LinkedList<Integer> queue = new LinkedList<Integer>();
            queue.addLast(start);
            Integer lastNode = start, distance = -1;
            while (queue.size() > 0) {
                LinkedList<Integer> nextQueue = new LinkedList<Integer>();
                while (queue.size() > 0) {
                    int nextNode = queue.removeFirst();
                    for (Integer neighbor : graph.get(nextNode)) {
                        if (!visited[neighbor]) {
                            visited[neighbor] = true;
                            nextQueue.addLast(neighbor);
                            lastNode = neighbor;
                        }
                    }
                }
                distance += 1;
                queue = nextQueue;
            }
            return new int[] { lastNode, distance };
        }
    }

    static class Solution2 {
        public int treeDiameter(int[][] edges) {
            List<Set<Integer>> graph = new ArrayList<Set<Integer>>();
            for (int i = 0; i < edges.length + 1; i++) {
                graph.add(new HashSet<Integer>());
            }
            for (int[] edge : edges) {
                Integer u = edge[0], v = edge[1];
                graph.get(u).add(v);
                graph.get(v).add(u);
            }

            LinkedList<Integer> leaves = new LinkedList<Integer>();
            for (int vertex = 0; vertex < graph.size(); ++vertex) {
                if (graph.get(vertex).size() == 1) {
                    leaves.add(vertex);
                }
            }

            int layers = 0;
            int vertexLeft = edges.length + 1;
            while (vertexLeft > 2) {
                vertexLeft -= leaves.size();

                LinkedList<Integer> nextLeaves = new LinkedList<Integer>();

                for (int leaf : leaves) {
                    int neighbor = graph.get(leaf).iterator().next();
                    graph.get(neighbor).remove(leaf);
                    if (graph.get(neighbor).size() == 1) {
                        nextLeaves.add(neighbor);
                    }
                }

                layers += 1;
                leaves = nextLeaves;
            }

            if (vertexLeft == 1) {
                return layers * 2;
            } else {
                return layers * 2 + 1;
            }
        }
    }

    static class Solution3 {
        List<List<Integer>> graph;
        Integer diameter = 0;

        public int treeDiameter(int[][] edges) {
            graph = new ArrayList<List<Integer>>();
            boolean[] visited = new boolean[edges.length + 1];
            for (int i = 0; i < edges.length + 1; i++) {
                graph.add(new ArrayList<Integer>());
                visited[i] = false;
            }
            for (int[] edge : edges) {
                Integer u = edge[0], v = edge[1];
                graph.get(u).add(v);
                graph.get(v).add(u);
            }
            dfs(0, visited);
            return diameter;
        }

        int dfs(int curr, boolean[] visited) {
            Integer topDistance1 = 0, topDistance2 = 0;
            visited[curr] = true;
            for (Integer neighbor : graph.get(curr)) {
                int distance = 0;
                if (!visited[neighbor]) {
                    distance = 1 + dfs(neighbor, visited);
                }

                if (distance > topDistance1) {
                    topDistance2 = topDistance1;
                    topDistance1 = distance;
                } else if (distance > topDistance2) {
                    topDistance2 = distance;
                }
            }
            diameter = Math.max(diameter, topDistance1 + topDistance2);
            return topDistance1;
        }
    }

    // https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/description/
    static class Solution4 {
        public int minimumPushes(String word) {
            Map<Character, Integer> frequencyMap = new HashMap<>();
            for (char c : word.toCharArray()) {
                frequencyMap.put(c, frequencyMap.getOrDefault(c, 0) + 1);
            }

            PriorityQueue<Integer> frequencyQueue = new PriorityQueue<>(
                    (a, b) -> b - a);

            frequencyQueue.addAll(frequencyMap.values());

            int totalPushes = 0;
            int index = 0;

            while (!frequencyQueue.isEmpty()) {
                totalPushes += (index / 8 + 1) * frequencyQueue.poll();
                index++;
            }

            return totalPushes;
        }
    }

    // https://leetcode.com/problems/kth-largest-element-in-an-array/description/
    static class Solution5 {
        public int findKthLargest(int[] nums, int k) {
            PriorityQueue<Integer> heap = new PriorityQueue<>();
            for (int num : nums) {
                heap.add(num);
                if (heap.size() > k) {
                    heap.poll();
                }
            }
            return heap.poll();
        }
    }

    // https://leetcode.com/problems/kth-largest-element-in-an-array/description/
    static class Solution6 {
        public int findKthLargest(int[] nums, int k) {
            var qs = new Object() {
                Random rnd = new Random();

                Integer apply(List<Integer> arr, int k) {
                    var idx = rnd.nextInt(arr.size());

                    var left = new ArrayList<Integer>();
                    var mid = new ArrayList<Integer>();
                    var right = new ArrayList<Integer>();

                    for (var x : arr) {
                        if (x < arr.get(idx)) {
                            right.add(x);
                        } else if (x == arr.get(idx)) {
                            mid.add(x);
                        } else {
                            left.add(x);
                        }
                    }

                    if (k <= left.size()) {
                        return apply(left, k);
                    } else if (k > left.size() + mid.size()) {
                        return apply(right, k - left.size() - mid.size());
                    }
                    return arr.get(idx);
                }
            };
            return qs.apply(Arrays.stream(nums).boxed().toList(), k);
        }
    }
}
