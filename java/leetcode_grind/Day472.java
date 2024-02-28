package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.SortedSet;
import java.util.TreeSet;

public class Day472 {
    // https://leetcode.com/problems/the-skyline-problem/
    static class Solution1 {
        public List<List<Integer>> getSkyline(int[][] buildings) {
            SortedSet<Integer> edgeSet = new TreeSet<>();
            for (int[] building : buildings) {
                int left = building[0], right = building[1];
                edgeSet.add(left);
                edgeSet.add(right);
            }
            List<Integer> edges = new ArrayList<>(edgeSet);
            Map<Integer, Integer> edgeIndexMap = new HashMap<>();
            for (int i = 0; i < edges.size(); i++) {
                edgeIndexMap.put(edges.get(i), i);
            }

            int[] heights = new int[edges.size()];

            for (int[] building : buildings) {
                int left = building[0], right = building[1], height = building[2];
                int leftIndex = edgeIndexMap.get(left), rightIndex = edgeIndexMap.get(right);

                for (int j = leftIndex; j < rightIndex; j++) {
                    heights[j] = Math.max(heights[j], height);
                }
            }

            List<List<Integer>> answer = new ArrayList<>();
            for (int i = 0; i < heights.length; i++) {
                int currHeight = heights[i], currPos = edges.get(i);

                if (answer.isEmpty() || answer.get(answer.size() - 1).get(1) != currHeight) {
                    answer.add(Arrays.asList(currPos, currHeight));
                }
            }
            return answer;
        }
    }

    static class Solution2 {
        public List<List<Integer>> getSkyline(int[][] buildings) {
            SortedSet<Integer> edgeSet = new TreeSet<>();
            for (var b : buildings) {
                edgeSet.add(b[0]);
                edgeSet.add(b[1]);
            }

            List<Integer> edges = new ArrayList<>(edgeSet);
            List<List<Integer>> result = new ArrayList<>();

            for (var edge : edges) {
                int maxHeight = 0;

                for (var b : buildings) {
                    if (b[0] <= edge && b[1] > edge) {
                        maxHeight = Math.max(maxHeight, b[2]);
                    }
                }

                if (result.isEmpty() || result.get(result.size() - 1).get(1) != maxHeight) {
                    result.add(Arrays.asList(edge, maxHeight));
                }
            }

            return result;
        }
    }
}
