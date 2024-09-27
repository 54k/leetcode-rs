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

    // https://leetcode.com/problems/my-calendar-iii/submissions/1403756216/
    static class MyCalendarThree1 {

        Map<Integer, Integer> diff;

        public MyCalendarThree1() {
            diff = new TreeMap<>();
        }

        public int book(int start, int end) {
            diff.put(start, diff.getOrDefault(start, 0) + 1);
            diff.put(end, diff.getOrDefault(end, 0) - 1);
            int res = 0, cur = 0;
            for (int delta : diff.values()) {
                cur += delta;
                res = Math.max(res, cur);
            }
            return res;
        }
    }

    static class MyCalendarThree2 {
        Map<Integer, Integer> vals;
        Map<Integer, Integer> lazy;

        public MyCalendarThree2() {
            vals = new HashMap<>();
            lazy = new HashMap<>();
        }

        void update(int start, int end, int left, int right, int idx) {
            if (start > right || end < left) {
                return;
            }
            if (start <= left && right <= end) {
                vals.put(idx, vals.getOrDefault(idx, 0) + 1);
                lazy.put(idx, lazy.getOrDefault(idx, 0) + 1);
            } else {
                int mid = (left + right) / 2;
                update(start, end, left, mid, idx * 2);
                update(start, end, mid + 1, right, idx * 2 + 1);
                vals.put(idx, lazy.getOrDefault(idx, 0)
                        + Math.max(vals.getOrDefault(idx * 2, 0), vals.getOrDefault(idx * 2 + 1, 0)));
            }
        }

        public int book(int start, int end) {
            update(start, end - 1, 0, 1000000000, 1);
            return vals.getOrDefault(1, 0);
        }
    }

    static class MyCalendarThree3 {
        TreeMap<Integer, Integer> starts;
        int res;

        public MyCalendarThree3() {
            starts = new TreeMap<>();
            starts.put(0, 0);
            res = 0;
        }

        void split(int x) {
            Integer prev = starts.floorKey(x);
            Integer next = starts.ceilingKey(x);
            if (next != null && next == x) {
                return;
            }
            starts.put(x, starts.get(prev));
        }

        public int book(int start, int end) {
            split(start);
            split(end);
            for (var interval : starts.subMap(start, true, end, false).entrySet()) {
                res = Math.max(res, interval.setValue(interval.getValue() + 1) + 1);
            }
            return res;
        }
    }

    // https://leetcode.com/problems/count-of-range-sum/description/
    // https://leetcode.com/problems/count-of-range-sum/solutions/1178174/Java-Clean-Merge-Sort-O(N-logN)-Solution-oror-with-detailed-Explanation/
    static class Solution1 {
        int lower;
        int upper;
        int count = 0;
        long[] pfxSum;

        public int countRangeSum(int[] nums, int lower, int upper) {
            int n = nums.length;

            this.lower = lower;
            this.upper = upper;
            this.pfxSum = new long[n + 1];

            for (int i = 0; i < n; i++) {
                pfxSum[i + 1] = pfxSum[i] + nums[i];
            }

            mergeSort(0, n);
            return count;
        }

        void mergeSort(int low, int high) {
            if (low >= high)
                return;
            int mid = low + (high - low) / 2;
            mergeSort(low, mid);
            mergeSort(mid + 1, high);

            int i = mid + 1, j = mid + 1;
            for (int k = low; k <= mid; k++) {
                while (i <= high && pfxSum[i] - pfxSum[k] < lower)
                    i++;
                while (j <= high && pfxSum[j] - pfxSum[k] <= upper)
                    j++;
                count += j - i;
            }

            merge(low, mid, high);
        }

        void merge(int low, int mid, int high) {
            long[] helper = new long[high - low + 1];
            for (int i = low; i <= high; i++) {
                helper[i - low] = pfxSum[i];
            }

            int i = low, j = mid + 1;
            int idx = low;

            while (i <= mid && j <= high) {
                if (helper[i - low] < helper[j - low]) {
                    pfxSum[idx++] = helper[i++ - low];
                } else {
                    pfxSum[idx++] = helper[j++ - low];
                }
            }

            while (i <= mid) {
                pfxSum[idx++] = helper[i++ - low];
            }
        }
    }
}
