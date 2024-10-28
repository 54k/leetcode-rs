package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Day711 {
    // https://leetcode.com/problems/longest-square-streak-in-an-array/description/?envType=daily-question&envId=2024-10-28
    static class Solution1 {
        public int longestSquareStreak(int[] nums) {
            Arrays.sort(nums);
            Set<Integer> processedNumbers = new HashSet<>();
            int longestStreak = 0;
            for (int current : nums) {
                if (processedNumbers.contains(current))
                    continue;

                int streak = current;
                int streakLength = 1;

                while (true) {
                    if ((long) streak * (long) streak > 1e5)
                        break;
                    if (binarySearch(nums, streak * streak)) {
                        streak *= streak;
                        processedNumbers.add(streak);
                        streakLength++;
                    } else {
                        break;
                    }
                }
                longestStreak = Math.max(longestStreak, streakLength);
            }
            return longestStreak < 2 ? -1 : longestStreak;
        }

        boolean binarySearch(int[] nums, int target) {
            if (target < 0)
                return false;
            int left = 0;
            int right = nums.length - 1;

            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (nums[mid] == target)
                    return true;
                if (nums[mid] > target)
                    right = mid - 1;
                else
                    left = mid + 1;
            }
            return false;
        }
    }

    static class Solution2 {
        public int longestSquareStreak(int[] nums) {
            Map<Integer, Integer> streakLengths = new HashMap<>();
            Arrays.sort(nums);
            for (int number : nums) {
                int root = (int) Math.sqrt(number);

                if (root * root == number && streakLengths.containsKey(root)) {
                    streakLengths.put(number, streakLengths.get(root) + 1);
                } else {
                    streakLengths.put(number, 1);
                }
            }

            int longestStreak = 0;
            for (int streakLength : streakLengths.values()) {
                longestStreak = Math.max(longestStreak, streakLength);
            }
            return longestStreak == 1 ? -1 : longestStreak;
        }
    }
}
