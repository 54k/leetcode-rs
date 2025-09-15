package leetcode_grind;

public class Day1031 {
    // https://leetcode.com/problems/maximum-number-of-words-you-can-type/description/?envType=daily-question&envId=2025-09-15
    static class Solution1 {
        public int canBeTypedWords(String text, String brokenLetters) {
            int ans = 0;
            for (String word : text.split("\\s")) {
                int a = 1;
                for (char bl : brokenLetters.toCharArray()) {
                    if (word.indexOf(bl) > -1) {
                        a--;
                        break;
                    }
                }
                ans += a;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/check-if-a-number-is-majority-element-in-a-sorted-array/description/?envType=weekly-question&envId=2025-09-15
    static class Solution2 {
        int lower_bound(int[] nums, int target) {
            int start = 0;
            int end = nums.length - 1;
            int index = nums.length;

            while (start <= end) {
                int mid = (start + end) / 2;
                if (nums[mid] >= target) {
                    end = mid - 1;
                    index = mid;
                } else {
                    start = mid + 1;
                }
            }
            return index;
        }

        int upper_bound(int[] nums, int target) {
            int start = 0;
            int end = nums.length - 1;
            int index = nums.length;

            while (start <= end) {
                int mid = (start + end) / 2;
                if (nums[mid] > target) {
                    end = mid - 1;
                    index = mid;
                } else {
                    start = mid + 1;
                }
            }
            return index;
        }

        public boolean isMajorityElement(int[] nums, int target) {
            int firstIndex = lower_bound(nums, target);
            int nextToLastIndex = upper_bound(nums, target);
            return nextToLastIndex - firstIndex > nums.length / 2;
        }
    }

}
