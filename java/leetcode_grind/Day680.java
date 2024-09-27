package leetcode_grind;

import java.util.*;

public class Day680 {
    // https://leetcode.com/problems/my-calendar-ii/description/?envType=daily-question&envId=2024-09-27
    static class MyCalendarTwo1 {

        List<int[]> bookings;
        List<int[]> overlapBookings;

        public MyCalendarTwo1() {
            bookings = new ArrayList<>();
            overlapBookings = new ArrayList<>();
        }

        boolean doesOverlap(int start1, int end1, int start2, int end2) {
            return Math.max(start1, start2) < Math.min(end1, end2);
        }

        int[] getOverlapped(int start1, int end1, int start2, int end2) {
            return new int[] { Math.max(start1, start2), Math.min(end1, end2) };
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

    /**
     * Your MyCalendarTwo object will be instantiated and called as such:
     * MyCalendarTwo obj = new MyCalendarTwo();
     * boolean param_1 = obj.book(start,end);
     */

}
