package leetcode_grind;

import java.util.*;

public class Day689 {
    // https://leetcode.com/problems/permutation-in-string/description/?envType=daily-question&envId=2024-10-05
    static class Solution1 {
        boolean flag = false;

        public boolean checkInclusion(String s1, String s2) {
            permute(s1, s2, 0);
            return flag;
        }

        String swap(String s, int i0, int i1) {
            if (i0 == i1) {
                return s;
            }
            String s1 = s.substring(0, i0);
            String s2 = s.substring(i0 + 1, i1);
            String s3 = s.substring(i1 + 1);
            return s1 + s.charAt(i1) + s2 + s.charAt(i0) + s3;
        }

        void permute(String s1, String s2, int l) {
            if (l == s1.length()) {
                if (s2.indexOf(s1) >= 0) {
                    flag = true;
                }
            } else {
                for (int i = l; i < s1.length(); i++) {
                    s1 = swap(s1, l, i);
                    permute(s1, s2, l + 1);
                    s1 = swap(s1, l, i);
                }
            }
        }
    }

    static class Solution2 {
        public boolean checkInclusion(String s1, String s2) {
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
    }

    static class Solution3 {
        public boolean checkInclusion(String s1, String s2) {
            if (s1.length() > s2.length()) {
                return false;
            }
            HashMap<Character, Integer> s1map = new HashMap<>();
            for (int i = 0; i < s1.length(); i++) {
                s1map.put(s1.charAt(i), s1map.getOrDefault(s1.charAt(i), 0) + 1);
            }
            for (int i = 0; i <= s2.length() - s1.length(); i++) {
                HashMap<Character, Integer> s2map = new HashMap<>();
                for (int j = 0; j < s1.length(); j++) {
                    s2map.put(s2.charAt(i + j), s2map.getOrDefault(s2.charAt(i + j), 0) + 1);
                }
                if (matches(s1map, s2map)) {
                    return true;
                }
            }
            return false;
        }

        boolean matches(HashMap<Character, Integer> s1map, HashMap<Character, Integer> s2map) {
            for (char key : s1map.keySet()) {
                if (s1map.get(key) - s2map.getOrDefault(key, -1) != 0) {
                    return false;
                }
            }
            return true;
        }
    }

    static class Solution4 {
        public boolean checkInclusion(String s1, String s2) {
            if (s1.length() > s2.length()) {
                return false;
            }
            int[] s1arr = new int[26];
            for (int i = 0; i < s1.length(); i++) {
                s1arr[s1.charAt(i) - 'a']++;
            }
            for (int i = 0; i <= s2.length() - s1.length(); i++) {
                int[] s2arr = new int[26];
                for (int j = 0; j < s1.length(); j++) {
                    s2arr[s2.charAt(i + j) - 'a']++;
                }
                if (matches(s1arr, s2arr)) {
                    return true;
                }
            }
            return false;
        }

        boolean matches(int[] s1arr, int[] s2arr) {
            for (int i = 0; i < 26; i++) {
                if (s1arr[i] != s2arr[i]) {
                    return false;
                }
            }
            return true;
        }
    }

    static class Solution5 {
        public boolean checkInclusion(String s1, String s2) {
            if (s1.length() > s2.length()) {
                return false;
            }
            int[] s1arr = new int[26];
            int[] s2arr = new int[26];
            for (int i = 0; i < s1.length(); i++) {
                s1arr[s1.charAt(i) - 'a']++;
                s2arr[s2.charAt(i) - 'a']++;
            }
            for (int i = 0; i < s2.length() - s1.length(); i++) {
                if (matches(s1arr, s2arr)) {
                    return true;
                }
                s2arr[s2.charAt(i + s1.length()) - 'a']++;
                s2arr[s2.charAt(i) - 'a']--;
            }
            return matches(s1arr, s2arr);
        }

        boolean matches(int[] s1arr, int[] s2arr) {
            for (int i = 0; i < 26; i++) {
                if (s1arr[i] != s2arr[i]) {
                    return false;
                }
            }
            return true;
        }
    }

    static class Solution6 {
        public boolean checkInclusion(String s1, String s2) {
            if (s1.length() > s2.length()) {
                return false;
            }
            int[] s1arr = new int[26];
            int[] s2arr = new int[26];
            for (int i = 0; i < s1.length(); i++) {
                s1arr[s1.charAt(i) - 'a']++;
                s2arr[s2.charAt(i) - 'a']++;
            }
            int count = 0;
            for (int i = 0; i < 26; i++) {
                if (s1arr[i] == s2arr[i]) {
                    count++;
                }
            }
            for (int i = 0; i < s2.length() - s1.length(); i++) {
                int r = s2.charAt(i + s1.length()) - 'a', l = s2.charAt(i) - 'a';
                if (count == 26) {
                    return true;
                }
                s2arr[r]++;
                if (s2arr[r] == s1arr[r]) {
                    count++;
                } else if (s2arr[r] == s1arr[r] + 1) {
                    count--;
                }
                s2arr[l]--;
                if (s2arr[l] == s1arr[l]) {
                    count++;
                } else if (s2arr[l] == s1arr[l] - 1) {
                    count--;
                }
            }
            return count == 26;
        }
    }

    static class Solution7 {
        Random rnd = new Random();

        public boolean checkInclusion(String s1, String s2) {
            var hashes = new int[26];
            for (int i = 0; i < hashes.length; i++) {
                hashes[i] = rnd.nextInt();
            }

            var h1 = 0l;
            for (int i = 0; i < s1.length(); i++) {
                h1 += hashes[s1.charAt(i) - 'a'];
            }

            var h2 = 0l;
            for (int i = 0; i < s2.length(); i++) {
                h2 += hashes[s2.charAt(i) - 'a'];
                if (i >= s1.length()) {
                    h2 -= hashes[s2.charAt(i - s1.length()) - 'a'];
                }
                if (h1 == h2) {
                    return true;
                }
            }
            return false;
        }
    }
}
