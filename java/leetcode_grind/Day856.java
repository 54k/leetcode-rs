package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class Day856 {
    // https://leetcode.com/problems/count-days-without-meetings/description/?envType=daily-question&envId=2025-03-24
    static class Solution1 {
        public int countDays(int days, int[][] meetings) {
            TreeMap<Integer, Integer> dayMap = new TreeMap<>();
            int prefixSum = 0, freeDays = 0, previousDay = days;

            for (int[] meeting : meetings) {
                previousDay = Math.min(previousDay, meeting[0]);
                dayMap.put(meeting[0], dayMap.getOrDefault(meeting[0], 0) + 1);
                dayMap.put(meeting[1] + 1, dayMap.getOrDefault(meeting[1] + 1, 0) - 1);
            }

            freeDays += (previousDay - 1);
            for (Map.Entry<Integer, Integer> day : dayMap.entrySet()) {
                int currentDay = day.getKey();
                if (prefixSum == 0) {
                    freeDays += (currentDay - previousDay);
                }
                prefixSum += day.getValue();
                previousDay = currentDay;
            }

            freeDays += days - previousDay + 1;
            return freeDays;
        }
    }

    static class Solution2 {
        public int[] rearrangeBarcodes(int[] barcodes) {
            Map<Integer, Integer> cnt = new HashMap<>();
            for (int i : barcodes)
                cnt.put(i, cnt.getOrDefault(i, 0) + 1);

            List<Map.Entry<Integer, Integer>> list = new ArrayList<>(cnt.entrySet());
            Collections.sort(list, Map.Entry.<Integer, Integer>comparingByValue().reversed());

            int l = barcodes.length, i = 0;
            int[] res = new int[l];
            for (Map.Entry<Integer, Integer> e : list) {
                int time = e.getValue();
                while (time-- > 0) {
                    res[i] = e.getKey();
                    i += 2;
                    if (i >= barcodes.length)
                        i = 1;
                }
            }

            return res;
        }
    }
}
