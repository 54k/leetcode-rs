package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day1014 {
    // https://leetcode.com/problems/shortest-distance-to-target-color/description/?envType=weekly-question&envId=2025-08-29
    static class Solution1 {
        public List<Integer> shortestDistanceColor(int[] colors, int[][] queries) {
            List<Integer> queryResults = new ArrayList<>();
            Map<Integer, List<Integer>> hashmap = new HashMap<>();

            for (int i = 0; i < colors.length; i++) {
                hashmap.putIfAbsent(colors[i], new ArrayList<>());
                hashmap.get(colors[i]).add(i);
            }

            for (int i = 0; i < queries.length; i++) {
                int target = queries[i][0], color = queries[i][1];
                if (!hashmap.containsKey(color)) {
                    queryResults.add(-1);
                    continue;
                }

                List<Integer> indexList = hashmap.get(color);
                int insert = Collections.binarySearch(indexList, target);

                if (insert < 0) {
                    insert = (insert + 1) * -1;
                }

                if (insert == 0) {
                    queryResults.add(indexList.get(insert) - target);
                } else if (insert == indexList.size()) {
                    queryResults.add(target - indexList.get(insert - 1));
                } else {
                    int leftNearest = target - indexList.get(insert - 1);
                    int rightNearest = indexList.get(insert) - target;
                    queryResults.add(Math.min(leftNearest, rightNearest));
                }
            }
            return queryResults;
        }
    }

    static class Solution2 {
        public List<Integer> shortestDistanceColor(int[] colors, int[][] queries) {
            int n = colors.length;
            int[] rightmost = { 0, 0, 0 };
            int[] leftmost = { n - 1, n - 1, n - 1 };

            int[][] distance = new int[3][n];

            for (int i = 0; i < 3; i++) {
                for (int j = 0; j < n; j++) {
                    distance[i][j] = -1;
                }
            }

            for (int i = 0; i < n; i++) {
                int color = colors[i] - 1;
                for (int j = rightmost[color]; j < i + 1; j++) {
                    distance[color][j] = i - j;
                }
                rightmost[color] = i + 1;
            }

            for (int i = n - 1; i > -1; i--) {
                int color = colors[i] - 1;
                for (int j = leftmost[color]; j > i - 1; j--) {
                    if (distance[color][j] == -1 || distance[color][j] > j - i) {
                        distance[color][j] = j - i;
                    }
                }
                leftmost[color] = i - 1;
            }

            List<Integer> queryResults = new ArrayList<>();
            for (int i = 0; i < queries.length; i++) {
                queryResults.add(distance[queries[i][1] - 1][queries[i][0]]);
            }
            return queryResults;
        }
    }

    // https://leetcode.com/problems/alice-and-bob-playing-flower-game/description/?envType=daily-question&envId=2025-08-29
    static class Solution3 {
        public long flowerGame(int n, int m) {
            return ((long) m * n) / 2;
        }
    }

}
