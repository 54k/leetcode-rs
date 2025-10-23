package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1069 {
    // https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i/description/?envType=daily-question&envId=2025-10-23
    static class Solution1 {
        public boolean hasSameDigits(String s) {
            int n = s.length();
            char[] sArray = s.toCharArray();
            for (int i = 1; i <= n - 2; i++) {
                for (int j = 0; j <= n - 1 - i; j++) {
                    int digit1 = sArray[j] - '0';
                    int digit2 = sArray[j + 1] - '0';
                    sArray[j] = (char) (((digit1 + digit2) % 10) + '0');
                }
            }
            return sArray[0] == sArray[1];
        }
    }

    // https://leetcode.com/problems/my-calendar-ii/description/
    static class MyCalendarTwo1 {
        List<int[]> bookings;
        List<int[]> overlapBookings;

        boolean doesOverlap(int start1, int end1, int start2, int end2) {
            return Math.max(start1, start2) < Math.min(end1, end2);
        }

        int[] getOverlapped(int start1, int end1, int start2, int end2) {
            return new int[] { Math.max(start1, start2), Math.min(end1, end2) };
        }

        public MyCalendarTwo1() {
            bookings = new ArrayList<>();
            overlapBookings = new ArrayList<>();
        }

        public boolean book(int start, int end) {
            for (int[] booking : overlapBookings) {
                if (doesOverlap(booking[0], booking[1], start, end)) {
                    return false;
                }
            }

            for (int[] booking : bookings) {
                if (doesOverlap(booking[0], booking[1], start, end)) {
                    overlapBookings.add(getOverlapped(booking[0], booking[1], start, end));
                }
            }

            bookings.add(new int[] { start, end });
            return true;
        }
    }

}
