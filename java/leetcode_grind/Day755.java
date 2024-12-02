package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day755 {
    // https://leetcode.com/problems/check-if-n-and-its-double-exist/description/
    static class Solution1 {
        public boolean checkIfExist(int[] arr) {
            Arrays.sort(arr);
            for (int i = 0; i < arr.length; i++) {
                int target = 2 * arr[i];
                int index = customBinarySearch(arr, target);
                if (index >= 0 && index != i) {
                    return true;
                }
            }
            return false;
        }

        int customBinarySearch(int[] arr, int target) {
            int left = 0;
            int right = arr.length - 1;
            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (arr[mid] == target) {
                    return mid;
                } else if (arr[mid] < target) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            return -1;
        }
    }

    static class Solution2 {
        public boolean checkIfExist(int[] arr) {
            Map<Integer, Integer> map = new HashMap<>();
            for (int num : arr) {
                map.put(num, map.getOrDefault(num, 0) + 1);
            }
            for (int num : arr) {
                if (num != 0 && map.containsKey(2 * num)) {
                    return true;
                }
                if (num == 0 && map.get(num) > 1) {
                    return true;
                }
            }
            return false;
        }
    }

    // https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/description/
    static class Solution3 {
        public int isPrefixOfWord(String sentence, String searchWord) {
            var ans = 0;
            outer: for (var w : sentence.split("\\s+")) {
                ans++;
                for (int i = 0; i < searchWord.length(); i++) {
                    if (w.length() < searchWord.length() || searchWord.charAt(i) != w.charAt(i)) {
                        continue outer;
                    }
                }
                return ans;
            }
            return -1;
        }
    }
}
