package leetcode_grind;
// todo https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/editorial/

import java.util.PriorityQueue;

public class Day543 {
    // https://leetcode.com/problems/k-th-smallest-prime-fraction/description
    static class Solution1 {
        public int[] kthSmallestPrimeFraction(int[] arr, int k) {
            int n = arr.length;
            double left = 0, right = 1.0;
            while (left < right) {
                double mid = (left + right) / 2;
                double maxFraction = 0.0;
                int totalSmallerFractions = 0, numeratorIdx = 0, denominatorIdx = 0;
                int j = 1;

                for (int i = 0; i < n - 1; i++) {
                    while (j < n && arr[i] >= mid * arr[j]) {
                        j++;
                    }
                    totalSmallerFractions += (n - j);
                    if (j == n)
                        break;
                    double fraction = (double) arr[i] / arr[j];
                    if (fraction > maxFraction) {
                        numeratorIdx = i;
                        denominatorIdx = j;
                        maxFraction = fraction;
                    }
                }

                if (totalSmallerFractions == k) {
                    return new int[] { arr[numeratorIdx], arr[denominatorIdx] };
                } else if (totalSmallerFractions > k) {
                    right = mid;
                } else {
                    left = mid;
                }
            }

            return new int[] {};
        }
    }

    // https://leetcode.com/problems/k-th-smallest-prime-fraction/description/
    static class Solution2 {
        public int[] kthSmallestPrimeFraction(int[] arr, int k) {
            PriorityQueue<double[]> pq = new PriorityQueue<>((a, b) -> Double.compare(b[0], a[0]));

            for (int i = 0; i < arr.length; i++) {
                pq.offer(
                        new double[] {
                                -1.0 * arr[i] / arr[arr.length - 1],
                                i,
                                arr.length - 1
                        });
            }

            while (--k > 0) {
                double[] cur = pq.poll();
                int numeratorIndex = (int) cur[1];
                int denominatorIndex = (int) cur[2] - 1;
                if (denominatorIndex > numeratorIndex) {
                    pq.offer(
                            new double[] {
                                    -1.0 * arr[numeratorIndex] / arr[denominatorIndex],
                                    numeratorIndex,
                                    denominatorIndex
                            });
                }
            }

            double[] result = pq.poll();
            return new int[] { arr[(int) result[1]], arr[(int) result[2]] };
        }
    }
}
