package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1056 {
    // https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/description/?envType=daily-question&envId=2025-10-10
    static class Solution1 {
        public int maximumEnergy(int[] energy, int k) {
            int n = energy.length;
            int ans = Integer.MIN_VALUE;

            for (int i = n - k; i < n; i++) {
                int sum = 0;
                for (int j = i; j >= 0; j -= k) {
                    sum += energy[j];
                    ans = Math.max(ans, sum);
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/find-k-closest-elements/description/?envType=company&envId=yandex&favoriteSlug=yandex-all
    static class Solution2 {
        public List<Integer> findClosestElements(int[] arr, int k, int x) {
            List<Integer> result = new ArrayList<Integer>();

            if (arr.length == k) {
                for (int i = 0; i < k; i++) {
                    result.add(arr[i]);
                }
                return result;
            }

            int left = 0;
            int right = arr.length;
            int mid = 0;

            while (left < right) {
                mid = (left + right) / 2;
                if (arr[mid] >= x) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            left -= 1;
            right = left + 1;

            while (right - left - 1 < k) {
                if (left == -1) {
                    right += 1;
                    continue;
                }

                if (right == arr.length || Math.abs(arr[left] - x) <= Math.abs(arr[right] - x)) {
                    left -= 1;
                } else {
                    right += 1;
                }
            }

            for (int i = left + 1; i < right; i++) {
                result.add(arr[i]);
            }

            return result;
        }
    }
}