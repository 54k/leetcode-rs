package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

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

    // https://leetcode.com/problems/analyze-user-website-visit-pattern/description/?envType=weekly-question&envId=2025-09-22
    static class Solution2 {

        static class Pair {
            int time;
            String web;

            Pair(int time, String web) {
                this.time = time;
                this.web = web;
            }
        }

        public List<String> mostVisitedPattern(String[] username, int[] timestamp, String[] website) {
            Map<String, List<Pair>> map = new HashMap<>();
            int n = username.length;
            for (int i = 0; i < n; i++) {
                map.putIfAbsent(username[i], new ArrayList<>());
                map.get(username[i]).add(new Pair(timestamp[i], website[i]));
            }
            Map<String, Integer> count = new HashMap<>();
            String res = "";
            for (String key : map.keySet()) {
                Set<String> set = new HashSet<>();
                List<Pair> list = map.get(key);
                Collections.sort(list, (a, b) -> (a.time - b.time));

                for (int i = 0; i < list.size(); i++) {
                    for (int j = i + 1; j < list.size(); j++) {
                        for (int k = j + 1; k < list.size(); k++) {
                            String str = list.get(i).web + " " + list.get(j).web + " " + list.get(k).web;
                            if (!set.contains(str)) {
                                count.put(str, count.getOrDefault(str, 0) + 1);
                                set.add(str);
                            }
                            if (res.equals("") || count.get(res) < count.get(str)
                                    || (count.get(res) == count.get(str) && res.compareTo(str) > 0)) {
                                res = str;
                            }
                        }
                    }
                }
            }

            String[] r = res.split(" ");
            List<String> result = new ArrayList<>();
            for (String str : r) {
                result.add(str);
            }
            return result;
        }
    }

}
