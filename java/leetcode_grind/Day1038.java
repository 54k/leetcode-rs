package leetcode_grind;

import java.util.HashMap;
import java.util.Map;

public class Day1038 {
    // https://leetcode.com/problems/count-elements-with-maximum-frequency/description/?envType=daily-question&envId=2025-09-22
    static public class Solution1 {
        public int maxFrequencyElements(int[] nums) {
            Map<Integer, Integer> frequencies = new HashMap<>();
            int maxFrequency = 0;
            int totalFrequencies = 0;

            // Find the frequency of each element
            // Determine the maximum frequency
            // Calculate the total frequencies of elements with the maximum frequency
            for (int num : nums) {
                frequencies.put(num, frequencies.getOrDefault(num, 0) + 1);
                int frequency = frequencies.get(num);

                // If we discover a higher frequency element
                // Update maxFrequency
                // Re-set totalFrequencies to the new max frequency
                if (frequency > maxFrequency) {
                    maxFrequency = frequency;
                    totalFrequencies = frequency;
                    // If we find an element with the max frequency, add it to the total
                } else if (frequency == maxFrequency) {
                    totalFrequencies += frequency;
                }
            }
            return totalFrequencies;
        }
    }

}
