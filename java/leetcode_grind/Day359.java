package leetcode_grind;

import java.util.PriorityQueue;
import java.util.TreeSet;

public class Day359 {
    // https://leetcode.com/problems/seat-reservation-manager/description
    static class SeatManager1 {
        PriorityQueue<Integer> queue = new PriorityQueue<>();
        int marker = 1;

        public SeatManager1(int n) {
        }

        public int reserve() {
            if (!queue.isEmpty()) {
                return queue.poll();
            }
            return marker++;
        }

        public void unreserve(int seatNumber) {
            queue.add(seatNumber);
        }
    }

    static class SeatManager2 {
        int marker = 1;
        TreeSet<Integer> availableSeats = new TreeSet<>();

        public SeatManager2(int n) {

        }

        public int reserve() {
            if (!availableSeats.isEmpty()) {
                int seatNumber = availableSeats.first();
                availableSeats.remove(seatNumber);
                return seatNumber;
            }
            int seatNumber = marker;
            marker++;
            return seatNumber;
        }

        public void unreserve(int seatNumber) {
            availableSeats.add(seatNumber);
        }
    }
}
