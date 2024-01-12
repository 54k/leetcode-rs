package leetcode_grind;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
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
}
