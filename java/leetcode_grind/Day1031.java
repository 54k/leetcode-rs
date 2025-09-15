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

    // https://leetcode.com/problems/median-of-two-sorted-arrays/description/
    static class Solution3 {
        public double findMedianSortedArrays(int[] nums1, int[] nums2) {
            if (nums1.length > nums2.length) {
                return findMedianSortedArrays(nums2, nums1);
            }

            int n1 = nums1.length;
            int n2 = nums2.length;
            int total = n1 + n2;
            int target = (total + 1) / 2;

            int l = 0;
            int r = n1;

            while (true) {
                int take1 = l + (r - l) / 2;
                int take2 = target - take1;

                int max1 = (take1 == 0) ? Integer.MIN_VALUE : nums1[take1 - 1];
                int min1 = (take1 == n1) ? Integer.MAX_VALUE : nums1[take1];
                int max2 = (take2 == 0) ? Integer.MIN_VALUE : nums2[take2 - 1];
                int min2 = (take2 == n2) ? Integer.MAX_VALUE : nums2[take2];

                if (max1 > min2) {
                    r = take1 - 1;
                    continue;
                }

                if (max2 > min1) {
                    l = take1 + 1;
                    continue;
                }

                int medianValOdd = Math.max(max1, max2);
                if (total % 2 == 1) {
                    return medianValOdd;
                } else {
                    return (medianValOdd + Math.min(min1, min2)) / 2.0;
                }
            }
        }
    }

}
