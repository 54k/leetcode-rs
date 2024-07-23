package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

public class Day614 {
    // https://leetcode.com/problems/sort-array-by-increasing-frequency/description/?envType=daily-question&envId=2024-07-23
    static class Solution1 {
        public int[] frequencySort(int[] nums) {
            Map<Integer, Integer> freq = new HashMap<>();
            for (int num : nums) {
                freq.put(num, freq.getOrDefault(num, 0) + 1);
            }

            Integer[] numsObj = new Integer[nums.length];
            for (int i = 0; i < nums.length; i++) {
                numsObj[i] = nums[i];
            }

            Arrays.sort(numsObj, (a, b) -> {
                if (freq.get(a).equals(freq.get(b))) {
                    return Integer.compare(b, a);
                }
                return Integer.compare(freq.get(a), freq.get(b));
            });

            for (int i = 0; i < nums.length; i++) {
                nums[i] = numsObj[i];
            }
            return nums;
        }
    }

    // https://leetcode.com/problems/minimum-cost-to-reach-city-with-discounts/description/?envType=weekly-question&envId=2024-07-22
    static class Solution2 {
        public int minimumCost(int n, int[][] highways, int discounts) {
            List<List<int[]>> graph = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                graph.add(new ArrayList<>());
            }
            for (int[] highway : highways) {
                int u = highway[0], v = highway[1], toll = highway[2];
                graph.get(u).add(new int[] { v, toll });
                graph.get(v).add(new int[] { u, toll });
            }

            PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[0]));
            pq.offer(new int[] { 0, 0, 0 });

            int[][] dist = new int[n][discounts + 1];
            for (int[] row : dist) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            dist[0][0] = 0;
            boolean[][] visited = new boolean[n][discounts + 1];

            while (!pq.isEmpty()) {
                int[] current = pq.poll();
                int currentCost = current[0], city = current[1], discountsUsed = current[2];

                if (visited[city][discountsUsed]) {
                    continue;
                }
                visited[city][discountsUsed] = true;

                for (int[] neighbor : graph.get(city)) {
                    int nextCity = neighbor[0], toll = neighbor[1];

                    if (currentCost + toll < dist[nextCity][discountsUsed]) {
                        dist[nextCity][discountsUsed] = currentCost + toll;
                        pq.offer(new int[] {
                                dist[nextCity][discountsUsed],
                                nextCity,
                                discountsUsed,
                        });
                    }

                    if (discountsUsed < discounts) {
                        int newCostWithDiscount = currentCost + toll / 2;
                        if (newCostWithDiscount < dist[nextCity][discountsUsed + 1]) {
                            dist[nextCity][discountsUsed + 1] = newCostWithDiscount;
                            pq.offer(new int[] {
                                    newCostWithDiscount,
                                    nextCity,
                                    discountsUsed + 1,
                            });
                        }
                    }
                }
            }

            int minCost = Integer.MAX_VALUE;
            for (int d = 0; d <= discounts; ++d) {
                minCost = Math.min(minCost, dist[n - 1][d]);
            }
            return minCost == Integer.MAX_VALUE ? -1 : minCost;
        }
    }

    static class Solution3 {
        public int minimumCost(int n, int[][] highways, int discounts) {
            List<List<int[]>> graph = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                graph.add(new ArrayList<>());
            }

            for (int[] highway : highways) {
                int u = highway[0], v = highway[1], toll = highway[2];
                graph.get(u).add(new int[] { v, toll });
                graph.get(v).add(new int[] { u, toll });
            }

            PriorityQueue<int[]> pq = new PriorityQueue<>(
                    Comparator.comparingInt(a -> a[0]));
            pq.offer(new int[] { 0, 0, 0 });

            int[][] dist = new int[n][discounts + 1];
            for (int[] row : dist) {
                Arrays.fill(row, Integer.MAX_VALUE);
            }
            dist[0][0] = 0;

            while (!pq.isEmpty()) {
                int[] current = pq.poll();
                int currentCost = current[0], city = current[1], discountUsed = current[2];

                if (currentCost > dist[city][discountUsed])
                    continue;

                for (int[] neighbor : graph.get(city)) {
                    int nextCity = neighbor[0], toll = neighbor[1];

                    if (currentCost + toll < dist[nextCity][discountUsed]) {
                        dist[nextCity][discountUsed] = currentCost + toll;
                        pq.offer(new int[] { dist[nextCity][discountUsed], nextCity, discountUsed });
                    }

                    if (discountUsed < discounts) {
                        int newCostWithDiscount = currentCost + toll / 2;
                        if (newCostWithDiscount < dist[nextCity][discountUsed + 1]) {
                            dist[nextCity][discountUsed + 1] = newCostWithDiscount;
                            pq.offer(new int[] { newCostWithDiscount, nextCity, discountUsed + 1 });
                        }
                    }
                }
            }

            int minCost = Integer.MAX_VALUE;
            for (int d = 0; d <= discounts; ++d) {
                minCost = Math.min(minCost, dist[n - 1][d]);
            }
            return minCost == Integer.MAX_VALUE ? -1 : minCost;
        }
    }
}
