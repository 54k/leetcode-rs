package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.PriorityQueue;

public class Day544 {
    static class Pair<F, S> {
        F key;
        S value;

        Pair(F f1, S s1) {
            key = f1;
            value = s1;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/minimum-cost-to-hire-k-workers/description
    static class Solution1 {
        public double mincostToHireWorkers(int[] quality, int[] wage, int k) {
            var n = quality.length;
            double totalCost = Double.MAX_VALUE;
            double currentTotalQuality = 0.0;
            var wagetToQualityRatio = new ArrayList<Pair<Double, Integer>>();

            for (int i = 0; i < n; i++) {
                wagetToQualityRatio.add(new Pair<>((double) wage[i] / quality[i], quality[i]));
            }

            Collections.sort(wagetToQualityRatio, Comparator.comparingDouble(Pair::getKey));

            var highestQualityWorkers = new PriorityQueue<Integer>(Collections.reverseOrder());

            for (int i = 0; i < n; i++) {
                highestQualityWorkers.add(wagetToQualityRatio.get(i).getValue());
                currentTotalQuality += wagetToQualityRatio.get(i).getValue();

                if (highestQualityWorkers.size() > k) {
                    currentTotalQuality -= highestQualityWorkers.poll();
                }

                if (highestQualityWorkers.size() == k) {
                    totalCost = Math.min(totalCost, currentTotalQuality * wagetToQualityRatio.get(i).getKey());
                }
            }
            return totalCost;
        }
    }

}
