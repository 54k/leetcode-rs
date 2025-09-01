package leetcode_grind;

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

}
