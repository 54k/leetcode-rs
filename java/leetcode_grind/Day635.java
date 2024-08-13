package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Comparator;
import java.util.LinkedList;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Queue;

public class Day635 {
    // https://leetcode.com/problems/combination-sum-ii/description/?envType=daily-question&envId=2024-08-13
    static class Solution1 {
        public List<List<Integer>> combinationSum2(int[] candidates, int target) {
            List<List<Integer>> list = new LinkedList<List<Integer>>();
            Arrays.sort(candidates);
            backtrack(list, new ArrayList<Integer>(), candidates, target, 0);
            return list;
        }

        void backtrack(
                List<List<Integer>> answer,
                List<Integer> tempList,
                int[] candidates,
                int totalLeft,
                int index) {
            if (totalLeft < 0)
                return;
            else if (totalLeft == 0) {
                answer.add(new ArrayList<>(tempList));
            } else {
                for (int i = index; i < candidates.length && totalLeft >= candidates[i]; i++) {
                    if (i > index && candidates[i] == candidates[i - 1])
                        continue;
                    tempList.add(candidates[i]);
                    backtrack(answer, tempList, candidates, totalLeft - candidates[i], i + 1);
                    tempList.remove(tempList.size() - 1);
                }
            }
        }
    }

    static class Pair<F, S> {
        F first;
        S second;

        Pair(F f, S s) {
            first = f;
            second = s;
        }

        F getKey() {
            return first;
        }

        S getValue() {
            return second;
        }
    }

    // https://leetcode.com/problems/minimum-cost-to-hire-k-workers/description/
    static class Solution2 {
        public double mincostToHireWorkers(int[] quality, int[] wage, int k) {
            int n = quality.length;
            double totalCost = Double.MAX_VALUE;
            double currentTotalQuality = 0;
            List<Pair<Double, Integer>> wageToQualityRatio = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                wageToQualityRatio.add(new Pair<>((double) wage[i] / quality[i], quality[i]));
            }
            Collections.sort(wageToQualityRatio, Comparator.comparingDouble(Pair::getKey));

            PriorityQueue<Integer> workers = new PriorityQueue<>(Collections.reverseOrder());
            for (int i = 0; i < n; i++) {
                workers.add(wageToQualityRatio.get(i).getValue());
                currentTotalQuality += wageToQualityRatio.get(i).getValue();
                if (workers.size() > k) {
                    currentTotalQuality -= workers.poll();
                }

                if (workers.size() == k) {
                    totalCost = Math.min(totalCost, currentTotalQuality * wageToQualityRatio.get(i).getKey());
                }
            }
            return totalCost;
        }
    }

    // https://leetcode.com/problems/k-closest-points-to-origin/description/
    static class Solution3 {
        public int[][] kClosest(int[][] points, int k) {
            Arrays.sort(points, (a, b) -> squaredDistance(a) - squaredDistance(b));
            return Arrays.copyOf(points, k);
        }

        int squaredDistance(int[] point) {
            return point[0] * point[0] + point[1] * point[1];
        }
    }

    static class Solution4 {
        public int[][] kClosest(int[][] points, int k) {
            Queue<int[]> maxPQ = new PriorityQueue<>((a, b) -> b[0] - a[0]);
            for (int i = 0; i < points.length; i++) {
                int[] entry = { squaredDistance(points[i]), i };
                if (maxPQ.size() < k) {
                    maxPQ.add(entry);
                } else if (entry[0] < maxPQ.peek()[0]) {
                    maxPQ.poll();
                    maxPQ.add(entry);
                }
            }

            int[][] answer = new int[k][2];
            for (int i = 0; i < k; i++) {
                int entryIndex = maxPQ.poll()[1];
                answer[i] = points[entryIndex];
            }
            return answer;
        }

        int squaredDistance(int[] point) {
            return point[0] * point[0] + point[1] * point[1];
        }
    }

    static class Solution5 {
        public int[][] kClosest(int[][] points, int k) {
            double[] distances = new double[points.length];
            double low = 0, high = 0;
            List<Integer> remaining = new ArrayList<>();
            for (int i = 0; i < points.length; i++) {
                distances[i] = euclideanDistance(points[i]);
                high = Math.max(high, distances[i]);
                remaining.add(i);
            }

            List<Integer> closest = new ArrayList<>();
            while (k > 0) {
                double mid = low + (high - low) / 2;
                List<List<Integer>> result = splitDistances(remaining, distances, mid);
                List<Integer> closer = result.get(0), farther = result.get(1);
                if (closer.size() > k) {
                    remaining = closer;
                    high = mid;
                } else {
                    k -= closer.size();
                    closest.addAll(closer);
                    remaining = farther;
                    low = mid;
                }
            }

            k = closest.size();
            int[][] answer = new int[k][2];
            for (int i = 0; i < k; i++) {
                answer[i] = points[closest.get(i)];
            }
            return answer;
        }

        List<List<Integer>> splitDistances(List<Integer> remaining, double[] distances, double mid) {
            List<List<Integer>> result = new ArrayList<>();
            List<Integer> closer = new ArrayList<>();
            List<Integer> farther = new ArrayList<>();
            result.add(closer);
            result.add(farther);

            for (int point : remaining) {
                if (distances[point] <= mid) {
                    closer.add(point);
                } else {
                    farther.add(point);
                }
            }
            return result;
        }

        double euclideanDistance(int[] point) {
            return point[0] * point[0] + point[1] * point[1];
        }
    }
}
