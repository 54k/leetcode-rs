package leetcode_grind;

public class Day319 {
    // https://leetcode.com/problems/decoded-string-at-index/description/
    static class Solution1 {
        public String decodeAtIndex(String s, int k) {
            long size = 0;
            var n = s.length();

            for (var i = 0; i < n; i++) {
                var c = s.charAt(i);
                if (Character.isDigit(c)) {
                    size *= c - '0';
                } else {
                    size++;
                }
            }

            for (var i = n - 1; i >= 0; i--) {
                var c = s.charAt(i);
                k %= size;
                if (k == 0 && Character.isLetter(c)) {
                    return Character.toString(c);
                }

                if (Character.isDigit(c)) {
                    size /= c - '0';
                } else {
                    size--;
                }
            }

            return "";
        }
    }

    // https://leetcode.com/problems/search-insert-position/description/
    static class Solution2 {
        public int searchInsert1(int[] nums, int target) {
            var lo = 0;
            var hi = nums.length;
            while (lo <= hi) {
                var mid = lo + (hi - lo) / 2;
                if (nums[mid] == target) {
                    return mid;
                }
                if (nums[mid] < target) {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            return lo;
        }

        public int searchInsert2(int[] nums, int target) {
            var lo = 0;
            var hi = nums.length;
            while (lo < hi) {
                var mid = (lo + hi) / 2;
                if (nums[mid] == target) {
                    return mid;
                }
                if (nums[mid] < target) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            return lo;
        }
    }

    // https://leetcode.com/problems/first-bad-version/description/
    /*
     * The isBadVersion API is defined in the parent class VersionControl.
     * boolean isBadVersion(int version);
     */
    interface VersionControl {
        boolean isBadVersion(int version);
    }

    public class Solution3 implements VersionControl {
        public boolean isBadVersion(int version) {
            return true;
        }

        public int firstBadVersion(int n) {
            var lo = 0;
            var hi = n;
            while (lo < hi) {
                var mid = lo + (hi - lo) / 2;
                if (isBadVersion(mid)) {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            return lo;
        }
    }
}
