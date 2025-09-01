package leetcode_grind;

import java.util.Arrays;
import java.util.PriorityQueue;

public class Day1017 {
    // https://leetcode.com/problems/maximum-average-pass-ratio/description/?envType=daily-question&envId=2025-09-01
    static class Solution1 {
        public double maxAverageRatio(int[][] classes, int extraStudents) {
            PriorityQueue<double[]> maxHeap = new PriorityQueue<>((a, b) -> Double.compare(b[0], a[0]));

            for (int[] singleClass : classes) {
                int passes = singleClass[0];
                int totalStudents = singleClass[1];
                double gain = calculateGain(passes, totalStudents);
                maxHeap.offer(new double[] { gain, passes, totalStudents });
            }

            while (extraStudents-- > 0) {
                double[] current = maxHeap.poll();
                int passes = (int) current[1];
                int totalStudents = (int) current[2];
                maxHeap.offer(new double[] {
                        calculateGain(passes + 1, totalStudents + 1),
                        passes + 1,
                        totalStudents + 1
                });
            }

            double totalPassRatio = 0;
            while (!maxHeap.isEmpty()) {
                double[] current = maxHeap.poll();
                int passes = (int) current[1];
                int totalStudents = (int) current[2];
                totalPassRatio += (double) passes / totalStudents;
            }
            return totalPassRatio / classes.length;
        }

        double calculateGain(int passes, int totalStudents) {
            return (double) (passes + 1) / (totalStudents + 1) -
                    (double) passes / totalStudents;
        }
    }

    // https://leetcode.com/problems/campus-bikes-ii/description/?envType=weekly-question&envId=2025-09-01
    static class Solution2 {
        boolean visited[] = new boolean[10];
        int smallestDistanceSum = Integer.MAX_VALUE;

        int findDistance(int[] worker, int[] bike) {
            return Math.abs(worker[0] - bike[0]) + Math.abs(worker[1] - bike[1]);
        }

        void minimumDistanceSum(int[][] workers, int workerIndex, int[][] bikes, int currDistanceSum) {
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

    static class Solution3 {
        int memo[] = new int[1024];

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
                    smallestDistanceSum = Math.min(smallestDistanceSum,
                            findDistance(workers[workerIndex], bikes[bikeIndex])
                                    + minimumDistanceSum(workers, bikes, workerIndex + 1, mask | (1 << bikeIndex)));
                }
            }
            return memo[mask] = smallestDistanceSum;
        }

        public int assignBikes(int[][] workers, int[][] bikes) {
            Arrays.fill(memo, -1);
            return minimumDistanceSum(workers, bikes, 0, 0);
        }
    }
}
