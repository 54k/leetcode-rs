package leetcode_grind;

public class Day679 {
    // https://leetcode.com/problems/my-calendar-i/description/?envType=daily-question&envId=2024-09-26
    // class MyCalendar {

    // List<int[]> calendar;

    // public MyCalendar() {
    // calendar = new ArrayList<>();
    // }

    // public boolean book(int start, int end) {
    // for (int[] iv : calendar) {
    // if (iv[0] < end && start < iv[1]) {
    // return false;
    // }
    // }
    // calendar.add(new int[] { start, end });
    // return true;
    // }
    // }

    // /**
    // * Your MyCalendar object will be instantiated and called as such:
    // * MyCalendar obj = new MyCalendar();
    // * boolean param_1 = obj.book(start,end);
    // */

    // class MyCalendar {
    //     TreeMap<Integer, Integer> calendar;

    //     public MyCalendar() {
    //         calendar = new TreeMap<>();
    //     }

    //     public boolean book(int start, int end) {
    //         Integer prev = calendar.floorKey(start),
    //                 next = calendar.ceilingKey(start);
    //         if ((prev == null || calendar.get(prev) <= start) &&
    //                 (next == null || end <= next)) {
    //             calendar.put(start, end);
    //             return true;
    //         }
    //         return false;
    //     }
    // }
}
