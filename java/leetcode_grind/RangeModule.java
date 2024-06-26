package leetcode_grind;
import java.util.TreeMap;

//https://leetcode.com/problems/range-module/solutions/108910/java-treemap/?orderBy=most_relevant
class RangeModule {
    TreeMap<Integer, Integer> intervals = new TreeMap<>();

    public void addRange(int left, int right) {
        Integer start = intervals.floorKey(left);
        Integer end = intervals.floorKey(right);
        if (start != null && intervals.get(start) >= left) {
            left = start;
        }
        if (end != null && intervals.get(end) > right) {
            right = intervals.get(end);
        }
        intervals.put(left, right);

        intervals.subMap(left, false, right, true).clear();
    }

    public boolean queryRange(int left, int right) {
        Integer start = intervals.floorKey(left);
        if (start == null) {
            return false;
        }
        return intervals.get(start) >= right;
    }

    public void removeRange(int left, int right) {
        Integer start = intervals.floorKey(left);
        Integer end = intervals.floorKey(right);

        if (end != null && intervals.get(end) > right) {
            intervals.put(right, intervals.get(end));
        }
        if (start != null && intervals.get(start) > left) {
            intervals.put(start, left);
        }
        intervals.subMap(left, true, right, false).clear();
    }
}

class Scratch {
    public static void main(String[] args) {
        var rangeModule = new RangeModule();
        rangeModule.addRange(1, 3);
        rangeModule.addRange(2, 5);
        System.out.println(rangeModule.queryRange(1, 5));
        rangeModule.removeRange(1, 5);
        System.out.println(rangeModule.queryRange(1, 5));
    }
}