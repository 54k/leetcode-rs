package leetcode_grind;

import java.util.HashMap;
import java.util.LinkedList;
import java.util.Map;
import java.util.Queue;

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

}
