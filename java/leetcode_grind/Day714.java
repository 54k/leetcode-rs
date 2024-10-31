package leetcode_grind;

import java.util.*;

public class Day714 {
    // https://leetcode.com/problems/minimum-total-distance-traveled/description/?envType=daily-question&envId=2024-10-31
    static class Solution1 {
        public long minimumTotalDistance(List<Integer> robot, int[][] factory) {
            Collections.sort(robot);
            Arrays.sort(factory, Comparator.comparingInt(a -> a[0]));

            List<Integer> factoryPoisitions = new ArrayList<>();
            for (int[] f : factory) {
                for (int i = 0; i < f[1]; i++) {
                    factoryPoisitions.add(f[0]);
                }
            }
            int robotCount = robot.size();
            int factoryCount = factoryPoisitions.size();
            long[][] memo = new long[robotCount][factoryCount];
            for (long[] row : memo)
                Arrays.fill(row, -1);
            return calculateMinDistance(0, 0, robot, factoryPoisitions, memo);
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

}
