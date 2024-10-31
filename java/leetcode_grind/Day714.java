package leetcode_grind;

import java.util.*;

public class Day714 {
    // https://leetcode.com/problems/minimum-total-distance-traveled/description/?envType=daily-question&envId=2024-10-31
    static class Solution1 {
        public long minimumTotalDistance(List<Integer> robot, int[][] factory) {
            Collections.sort(robot);
            Arrays.sort(factory, Comparator.comparingInt(a -> a[0]));

            List<Integer> factoryPositions = new ArrayList<>();
            for (int[] f : factory) {
                for (int i = 0; i < f[1]; i++) {
                    factoryPositions.add(f[0]);
                }
            }
            int robotCount = robot.size();
            int factoryCount = factoryPositions.size();
            long[][] memo = new long[robotCount][factoryCount];
            for (long[] row : memo)
                Arrays.fill(row, -1);
            return calculateMinDistance(0, 0, robot, factoryPositions, memo);
        }

        long calculateMinDistance(
                int robotIdx,
                int factoryIdx,
                List<Integer> robot,
                List<Integer> factoryPositions,
                long[][] memo) {
            if (robotIdx == robot.size())
                return 0;
            if (factoryIdx == factoryPositions.size())
                return (long) 1e12;
            if (memo[robotIdx][factoryIdx] != -1)
                return memo[robotIdx][factoryIdx];
            long assign = Math.abs(robot.get(robotIdx) - factoryPositions.get(factoryIdx)) +
                    calculateMinDistance(robotIdx + 1, factoryIdx + 1, robot, factoryPositions, memo);

            long skip = calculateMinDistance(robotIdx, factoryIdx + 1, robot, factoryPositions, memo);

            return memo[robotIdx][factoryIdx] = Math.min(assign, skip);
        }
    }

    static class Solution2 {
        public long minimumTotalDistance(List<Integer> robots, int[][] factories) {
            Collections.sort(robots);
            Arrays.sort(factories, Comparator.comparingInt(a -> a[0]));

            List<Integer> factoryPositions = new ArrayList<>();
            for (int[] factory : factories) {
                for (int i = 0; i < factory[1]; i++) {
                    factoryPositions.add(factory[0]);
                }
            }

            int robotCount = robots.size(), factoryCount = factoryPositions.size();
            long[] next = new long[factoryCount + 1];
            long[] current = new long[factoryCount + 1];

            for (int i = robotCount - 1; i >= 0; i--) {
                if (i != robotCount - 1)
                    next[factoryCount] = (long) 1e12;
                current[factoryCount] = (long) 1e12;

                for (int j = factoryCount - 1; j >= 0; j--) {
                    long assign = Math.abs((long) robots.get(i) - factoryPositions.get(j)) + next[j + 1];
                    long skip = current[j + 1];
                    current[j] = Math.min(assign, skip);
                }
                System.arraycopy(current, 0, next, 0, factoryCount + 1);
            }

            return current[0];
        }
    }
}
