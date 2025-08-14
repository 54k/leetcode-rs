package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.TreeMap;

public class Day999 {
    // https://leetcode.com/problems/largest-3-same-digit-number-in-string/description/?envType=daily-question&envId=2025-08-14
    static class Solution1 {
        public String largestGoodInteger(String num) {
            // Assign 'maxDigit' to the NUL character (smallest ASCII value character)
            char maxDigit = '\0';

            // Iterate on characters of the num string.
            for (int index = 0; index <= num.length() - 3; ++index) {
                // If 3 consecutive characters are the same,
                // store the character in 'maxDigit' if it's bigger than what it already stores.
                if (num.charAt(index) == num.charAt(index + 1) && num.charAt(index) == num.charAt(index + 2)) {
                    maxDigit = (char) Math.max(maxDigit, num.charAt(index));
                }
            }

            // If 'maxDigit' is NUL, return an empty string; otherwise, return a string of
            // size 3 with 'maxDigit' characters.
            return maxDigit == '\0' ? "" : new String(new char[] { maxDigit, maxDigit, maxDigit });
        }
    }

    static class TimeMap1 {
        HashMap<String, TreeMap<Integer, String>> keyTimeMap;

        public TimeMap1() {
            keyTimeMap = new HashMap<>();
        }

        public void set(String key, String value, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                keyTimeMap.put(key, new TreeMap<>());
            }
            keyTimeMap.get(key).put(timestamp, value);
        }

        public String get(String key, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                return "";
            }

            Integer floorKey = keyTimeMap.get(key).floorKey(timestamp);
            if (floorKey != null) {
                return keyTimeMap.get(key).get(floorKey);
            }
            return "";
        }
    }

    static class TimeMap2 {
        HashMap<String, ArrayList<Pair<Integer, String>>> keyTimeMap;

        public TimeMap2() {
            keyTimeMap = new HashMap<>();
        }

        public void set(String key, String value, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                keyTimeMap.put(key, new ArrayList<>());
            }
            keyTimeMap.get(key).add(new Pair<>(timestamp, value));
        }

        public String get(String key, int timestamp) {
            if (!keyTimeMap.containsKey(key)) {
                return "";
            }
            if (timestamp < keyTimeMap.get(key).get(0).getKey()) {
                return "";
            }

            int left = 0;
            int right = keyTimeMap.get(key).size();
            while (left < right) {
                int mid = (left + right) / 2;
                if (keyTimeMap.get(key).get(mid).getKey() <= timestamp) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            if (right == 0) {
                return "";
            }

            return keyTimeMap.get(key).get(right - 1).getValue();
        }
    }

    private static class Pair<T0, T1> {
        T0 key;
        T1 value;

        public Pair(T0 key, T1 value) {
            this.key = key;
            this.value = value;
        }

        T0 getKey() {
            return key;
        }

        T1 getValue() {
            return value;
        }
    }
}
