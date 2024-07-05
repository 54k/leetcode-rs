package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;

public class Day597 {
    public class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    // https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/description/?envType=daily-question&envId=2024-07-05
    static class Solution1 {
        public int[] nodesBetweenCriticalPoints(ListNode head) {
            int[] result = { -1, -1 };
            int minDistance = Integer.MAX_VALUE;

            ListNode previousNode = head;
            ListNode currentNode = head.next;
            int currentIndex = 1;
            int previousCriticalIndex = 0;
            int firstCriticalIndex = 0;

            while (currentNode.next != null) {
                if ((currentNode.val < previousNode.val && currentNode.val < currentNode.next.val)
                        || (currentNode.val > previousNode.val && currentNode.val > currentNode.next.val)) {
                    if (previousCriticalIndex == 0) {
                        previousCriticalIndex = currentIndex;
                        firstCriticalIndex = currentIndex;
                    } else {
                        minDistance = Math.min(minDistance, currentIndex - previousCriticalIndex);
                        previousCriticalIndex = currentIndex;
                    }
                }
                currentIndex++;
                previousNode = currentNode;
                currentNode = currentNode.next;
            }

            if (minDistance != Integer.MAX_VALUE) {
                int maxDistance = previousCriticalIndex - firstCriticalIndex;
                result = new int[] { minDistance, maxDistance };
            }
            return result;
        }
    }

    // https://leetcode.com/problems/cheapest-flights-within-k-stops/description/
    static class Solution2 {
        public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
            Map<Integer, List<int[]>> adj = new HashMap<>();
            for (int[] i : flights) {
                adj.computeIfAbsent(i[0], value -> new ArrayList<>()).add(new int[] { i[1], i[2] });
            }
            int[] dist = new int[n];
            Arrays.fill(dist, Integer.MAX_VALUE);
            Queue<int[]> q = new LinkedList<>();
            q.offer(new int[] { src, 0 });
            int stops = 0;

            while (stops <= k && !q.isEmpty()) {
                int sz = q.size();
                while (sz-- > 0) {
                    int[] temp = q.poll();
                    int node = temp[0];
                    int distance = temp[1];

                    if (!adj.containsKey(node)) {
                        continue;
                    }

                    for (int[] e : adj.get(node)) {
                        int neighbor = e[0];
                        int price = e[1];
                        if (price + distance >= dist[neighbor]) {
                            continue;
                        }
                        dist[neighbor] = price + distance;
                        q.offer(new int[] { neighbor, dist[neighbor] });
                    }
                }
                stops++;
            }
            return dist[dst] == Integer.MAX_VALUE ? -1 : dist[dst];
        }
    }

    static class Solution3 {
        public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
            int[] dist = new int[n];
            Arrays.fill(dist, Integer.MAX_VALUE);
            dist[src] = 0;

            for (int i = 0; i <= k; i++) {
                int[] temp = Arrays.copyOf(dist, n);
                for (int[] flight : flights) {
                    if (dist[flight[0]] != Integer.MAX_VALUE) {
                        temp[flight[1]] = Math.min(temp[flight[1]], dist[flight[0]] + flight[2]);
                    }
                }
                dist = temp;
            }
            return dist[dst] == Integer.MAX_VALUE ? -1 : dist[dst];
        }
    }

    static class Solution4 {
        public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
            Map<Integer, List<int[]>> adj = new HashMap<>();
            for (int[] i : flights) {
                adj.computeIfAbsent(i[0], value -> new ArrayList<>()).add(new int[] { i[1], i[2] });
            }

            int[] stops = new int[n];
            Arrays.fill(stops, Integer.MAX_VALUE);
            PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
            pq.offer(new int[] { 0, src, 0 });

            while (!pq.isEmpty()) {
                int[] temp = pq.poll();
                int dist = temp[0];
                int node = temp[1];
                int steps = temp[2];

                if (steps > stops[node] || steps > k + 1) {
                    continue;
                }

                stops[node] = steps;
                if (node == dst) {
                    return dist;
                }

                if (!adj.containsKey(node)) {
                    continue;
                }

                for (int[] a : adj.get(node)) {
                    pq.offer(new int[] { dist + a[1], a[0], steps + 1 });
                }
            }
            return -1;
        }
    }
}
