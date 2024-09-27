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

    static class MyCalendarTwo2 {
        TreeMap<Integer, Integer> bookingCount;
        int maxOverlappedBooking;

        public MyCalendarTwo2() {
            bookingCount = new TreeMap<>();
            maxOverlappedBooking = 2;
        }

        public boolean book(int start, int end) {
            bookingCount.put(start, bookingCount.getOrDefault(start, 0) + 1);
            bookingCount.put(end, bookingCount.getOrDefault(end, 0) - 1);

            int overlappedBooking = 0;

            for (Map.Entry<Integer, Integer> entry : bookingCount.entrySet()) {
                overlappedBooking += entry.getValue();

                if (overlappedBooking > maxOverlappedBooking) {
                    bookingCount.put(start, bookingCount.get(start) - 1);
                    bookingCount.put(end, bookingCount.get(end) + 1);
                    if (bookingCount.get(start) == 0) {
                        bookingCount.remove(start);
                    }
                    return false;
                }
            }
            return true;
        }
    }
}
