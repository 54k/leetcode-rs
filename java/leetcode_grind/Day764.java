package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.PriorityQueue;

public class Day764 {

    static class Pair<F, S> {
        F key;
        S value;

        Pair(F key, S value) {
            this.key = key;
            this.value = value;
        }

        public F getKey() {
            return key;
        }

        public S getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/description/?envType=daily-question&envId=2024-12-22
    static class Solution1 {
        public int[] leftmostBuildingQueries(int[] heights, int[][] queries) {
            List<Pair<Integer, Integer>> monoStack = new ArrayList<>();
            int[] result = new int[queries.length];
            Arrays.fill(result, -1);

            List<List<Pair<Integer, Integer>>> newQueries = new ArrayList<>(heights.length);
            for (int i = 0; i < heights.length; i++) {
                newQueries.add(new ArrayList<>());
            }
            for (int i = 0; i < queries.length; i++) {
                int a = queries[i][0];
                int b = queries[i][1];
                if (a > b) {
                    int temp = a;
                    a = b;
                    b = temp;
                }
                if (heights[b] > heights[a] || a == b) {
                    result[i] = b;
                } else {
                    newQueries.get(b).add(new Pair<>(heights[a], i));
                }
            }

            for (int i = heights.length - 1; i >= 0; i--) {
                int monoStackSize = monoStack.size();
                for (Pair<Integer, Integer> pair : newQueries.get(i)) {
                    int position = search(pair.getKey(), monoStack);
                    if (position < monoStackSize && position >= 0) {
                        result[pair.getValue()] = monoStack.get(position).getValue();
                    }
                }

                while (!monoStack.isEmpty() && monoStack.get(monoStack.size() - 1).getKey() <= heights[i]) {
                    monoStack.remove(monoStack.size() - 1);
                }
                monoStack.add(new Pair<>(heights[i], i));
            }

            return result;
        }

        int search(int height, List<Pair<Integer, Integer>> monoStack) {
            int left = 0;
            int right = monoStack.size() - 1;
            int ans = -1;
            while (left <= right) {
                int mid = (left + right) / 2;
                if (monoStack.get(mid).getKey() > height) {
                    ans = Math.max(ans, mid);
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            return ans;
        }
    }

    static class Solution2 {
        public int[] leftmostBuildingQueries(int[] heights, int[][] queries) {
            List<List<List<Integer>>> storeQueries = new ArrayList<>(heights.length);
            for (int i = 0; i < heights.length; i++)
                storeQueries.add(new ArrayList<>());
            PriorityQueue<List<Integer>> maxIndex = new PriorityQueue<>(
                    (a, b) -> a.get(0) - b.get(0));
            int[] result = new int[queries.length];
            Arrays.fill(result, -1);
            for (int currQuery = 0; currQuery < queries.length; currQuery++) {
                int a = queries[currQuery][0], b = queries[currQuery][1];
                if (a < b && heights[a] < heights[b]) {
                    result[currQuery] = b;
                } else if (a > b && heights[a] > heights[b]) {
                    result[currQuery] = a;
                } else if (a == b) {
                    result[currQuery] = a;
                } else {
                    storeQueries.get(Math.max(a, b)).add(Arrays.asList(Math.max(heights[a], heights[b]), currQuery));
                }
            }

            for (int index = 0; index < heights.length; index++) {
                while (!maxIndex.isEmpty() && maxIndex.peek().get(0) < heights[index]) {
                    result[maxIndex.peek().get(1)] = index;
                    maxIndex.poll();
                }
                for (List<Integer> element : storeQueries.get(index)) {
                    maxIndex.offer(element);
                }
            }
            return result;
        }
    }
}
