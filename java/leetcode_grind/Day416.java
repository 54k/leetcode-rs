package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.PriorityQueue;
import java.util.Set;

public class Day416 {
    // https://leetcode.com/problems/campus-bikes-ii/description/
    static class Solution1 {
        boolean[] visited = new boolean[10];
        int smallestDistanceSum = Integer.MAX_VALUE;

        private int findDistance(int[] worker, int[] bike) {
            return Math.abs(worker[0] - bike[0]) + Math.abs(worker[1] - bike[1]);
        }

        private void minimumDistanceSum(int[][] workers, int workerIndex, int[][] bikes, int currDistanceSum) {
            if (workerIndex >= workers.length) {
                smallestDistanceSum = Math.min(smallestDistanceSum, currDistanceSum);
                return;
            }

            if (currDistanceSum >= smallestDistanceSum) {
                return;
            }

            for (int bikeIndex = 0; bikeIndex < bikes.length; bikeIndex++) {
                if (!visited[bikeIndex]) {
                    visited[bikeIndex] = true;
                    minimumDistanceSum(workers, workerIndex + 1, bikes,
                            currDistanceSum + findDistance(workers[workerIndex], bikes[bikeIndex]));
                    visited[bikeIndex] = false;
                }
            }
        }

        public int assignBikes(int[][] workers, int[][] bikes) {
            minimumDistanceSum(workers, 0, bikes, 0);
            return smallestDistanceSum;
        }
    }

    static class Solution2 {
        int[] memo = new int[1024];

        int findDistance(int[] worker, int[] bike) {
            return Math.abs(worker[0] - bike[0]) + Math.abs(worker[1] - bike[1]);
        }

        int minimumDistanceSum(int[][] workers, int[][] bikes, int workerIndex, int mask) {
            if (workerIndex >= workers.length) {
                return 0;
            }

            if (memo[mask] != -1) {
                return memo[mask];
            }

            int smallestDistanceSum = Integer.MAX_VALUE;
            for (int bikeIndex = 0; bikeIndex < bikes.length; bikeIndex++) {
                if ((mask & (1 << bikeIndex)) == 0) {
                    smallestDistanceSum = Math.min(
                            smallestDistanceSum,
                            findDistance(workers[workerIndex], bikes[bikeIndex]) +
                                    minimumDistanceSum(workers, bikes, workerIndex + 1, mask | (1 << bikeIndex)));
                }
            }
            return memo[mask] = smallestDistanceSum;
        }

        public int assignBikes(int[][] workers, int[][] bikes) {
            Arrays.fill(memo, -1);
            return minimumDistanceSum(workers, bikes, 0, 0);
        }
    }

    static class Solution3 {
        int[] memo = new int[1024];

        int countNumOfOnes(int mask) {
            int count = 0;
            while (mask != 0) {
                mask &= (mask - 1);
                count++;
            }
            return count;
        }

        int findDistance(int[] worker, int[] bike) {
            return Math.abs(worker[0] - bike[0]) + Math.abs(worker[1] - bike[1]);
        }

        int minimumDistanceSum(int[][] workers, int[][] bikes) {
            int numOfBikes = bikes.length;
            int numOfWorkers = workers.length;
            int smallestDistanceSum = Integer.MAX_VALUE;

            memo[0] = 0;

            for (int mask = 0; mask < (1 << numOfBikes); mask++) {
                int nextWorkderIndex = countNumOfOnes(mask);

                if (nextWorkderIndex >= numOfWorkers) {
                    smallestDistanceSum = Math.min(smallestDistanceSum, memo[mask]);
                    continue;
                }

                for (int bikeIndex = 0; bikeIndex < numOfBikes; bikeIndex++) {
                    if ((mask & (1 << bikeIndex)) == 0) {
                        int newMask = (1 << bikeIndex) | mask;

                        memo[newMask] = Math.min(memo[newMask],
                                memo[mask] + findDistance(workers[nextWorkderIndex], bikes[bikeIndex]));
                    }
                }
            }

            return smallestDistanceSum;
        }

        public int assignBikes(int[][] workers, int[][] bikes) {
            Arrays.fill(memo, Integer.MAX_VALUE);
            return minimumDistanceSum(workers, bikes);
        }
    }

    static class Solution4 {
        private int findDistance(int[] worker, int[] bike) {
            return Math.abs(worker[0] - bike[0]) + Math.abs(worker[1] - bike[1]);
        }

        private int countNumOfOnes(int mask) {
            int count = 0;
            while (mask != 0) {
                mask &= (mask - 1);
                count++;
            }
            return count;
        }

        public int assignBikes(int[][] workers, int[][] bikes) {
            int numOfBikes = bikes.length;
            int numOfWorkers = workers.length;

            PriorityQueue<int[]> priorityQueue = new PriorityQueue<>((a, b) -> a[0] - b[0]);
            Set<Integer> visited = new HashSet<>();

            priorityQueue.add(new int[] { 0, 0 });

            while (!priorityQueue.isEmpty()) {
                int currentDistanceSum = priorityQueue.peek()[0];
                int currentMask = priorityQueue.peek()[1];
                priorityQueue.remove();

                if (visited.contains(currentMask)) {
                    continue;
                }
                visited.add(currentMask);

                int workerIndex = countNumOfOnes(currentMask);
                if (workerIndex == numOfWorkers) {
                    return currentDistanceSum;
                }

                for (int bikeIndex = 0; bikeIndex < numOfBikes; bikeIndex++) {
                    if ((currentMask & (1 << bikeIndex)) == 0) {
                        int nextStateDistanceSum = currentDistanceSum
                                + findDistance(workers[workerIndex], bikes[bikeIndex]);

                        int nextStateMask = currentMask | (1 << bikeIndex);
                        priorityQueue.add(new int[] { nextStateDistanceSum, nextStateMask });
                    }
                }
            }

            return -1;
        }
    }
}
