package leetcode_grind;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Day451 {
    // https://leetcode.com/problems/group-anagrams/
    static class Solution {
        public List<List<String>> groupAnagrams(String[] strs) {
            var groups = Arrays.stream(strs).collect(Collectors.groupingBy(s -> {
                var a = s.toCharArray();
                Arrays.sort(a);
                return String.valueOf(a);
            }));
            return groups.values().stream().toList();
        }
    }
}
