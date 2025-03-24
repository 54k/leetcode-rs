package leetcode_grind;

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

}
