package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day426 {
    // https://leetcode.com/problems/determine-if-string-halves-are-alike/description
    static class Solution1 {
        public boolean halvesAreAlike(String s) {
            var n = s.length();
            var vows = Set.of('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U');
            int i = 0, j = n - 1;
            int l = 0, r = 0;
            while (i < j) {
                if (vows.contains(s.charAt(i++))) {
                    l++;
                }
                if (vows.contains(s.charAt(j--))) {
                    r++;
                }
            }
            return l == r;
        }
    }

    // https://leetcode.com/problems/find-k-closest-elements/description/
    static class Solution2 {
        public List<Integer> findClosestElements1(int[] arr, int k, int x) {
            List<Integer> sortedArray = new ArrayList<>();
            for (int num : arr) {
                sortedArray.add(num);
            }
            Collections.sort(sortedArray, (num1, num2) -> Math.abs(num1 - x) - Math.abs(num2 - x));
            sortedArray = sortedArray.subList(0, k);
            Collections.sort(sortedArray);
            return sortedArray;
        }

        public List<Integer> findClosestElements2(int[] arr, int k, int x) {
            int left = 0, right = arr.length - k;
            while (left < right) {
                int mid = (left + right) / 2;
                if (x - arr[mid] > arr[mid + k] - x) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            var res = new ArrayList<Integer>();
            for (int i = left; i < left + k; i++) {
                res.add(arr[i]);
            }

            return res;
        }
    }

    // https://leetcode.com/problems/permutation-in-string/description/
    static class Solution3 {
        public boolean checkInclusion1(String s1, String s2) {
            s1 = sort(s1);
            for (int i = 0; i <= s2.length() - s1.length(); i++) {
                if (s1.equals(sort(s2.substring(i, i + s1.length())))) {
                    return true;
                }
            }
            return false;
        }

        String sort(String s) {
            char[] t = s.toCharArray();
            Arrays.sort(t);
            return new String(t);
        }

        public boolean checkInclusion2(String s1, String s2) {
            if (s1.length() > s2.length()) {
                return false;
            }

            Map<Character, Integer> s1map = new HashMap<>();
            for (int i = 0; i < s1.length(); i++) {
                s1map.put(s1.charAt(i), s1map.getOrDefault(s1.charAt(i), 0) + 1);
            }

            for (int i = 0; i <= s2.length() - s1.length(); i++) {
                Map<Character, Integer> s2map = new HashMap<>();
                for (int j = 0; j < s1.length(); j++) {
                    s2map.put(s2.charAt(i + j), s2map.getOrDefault(s2.charAt(i + j), 0) + 1);
                }
                if (matches(s1map, s2map)) {
                    return true;
                }
            }
            return false;
        }

        boolean matches(Map<Character, Integer> s1map, Map<Character, Integer> s2map) {
            for (char key : s1map.keySet()) {
                if (s1map.get(key) - s2map.getOrDefault(key, -1) != 0) {
                    return false;
                }
            }
            return true;
        }

        public boolean checkInclusion3(String s1, String s2) {
            if (s1.length() > s2.length()) {
                return false;
            }

            int[] s1map = new int[26];
            for (int i = 0; i < s1.length(); i++) {
                s1map[s1.charAt(i) - 'a']++;
            }

            for (int i = 0; i <= s2.length() - s1.length(); i++) {
                int[] s2map = new int[26];

                for (int j = 0; j < s1.length(); j++) {
                    s2map[s2.charAt(i + j) - 'a']++;
                }

                if (matches(s1map, s2map)) {
                    return true;
                }
            }

            return false;
        }

        boolean matches(int[] s1map, int[] s2map) {
            for (int i = 0; i < 26; i++) {
                if (s1map[i] != s2map[i]) {
                    return false;
                }
            }
            return true;
        }
    }
}
