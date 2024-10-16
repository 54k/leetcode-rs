package leetcode_grind;

import java.util.*;

public class Day699 {
    // https://leetcode.com/problems/separate-black-and-white-balls/description/?envType=daily-question&envId=2024-10-15
    static class Solution1 {
        public long minimumSteps(String s) {
            var arr = s.toCharArray();
            var w = 0;
            var b = 0;
            var ans = 0l;
            while (b < arr.length) {
                if (arr[b] == '0') {
                    var t = arr[w];
                    arr[w] = arr[b];
                    arr[b] = t;
                    ans += b - w;
                    w++;
                }
                b++;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/first-unique-number/description/?envType=weekly-question&envId=2024-10-15
    static class FirstUnique1 {
        Queue<Integer> queue = new ArrayDeque<>();

        public FirstUnique1(int[] nums) {
            for (int num : nums) {
                queue.add(num);
            }
        }

        public int showFirstUnique() {
            for (int num : queue) {
                int count = Collections.frequency(queue, num);
                if (count == 1) {
                    return num;
                }
            }
            return -1;
        }

        public void add(int value) {
            queue.add(value);
        }
    }

    static class FirstUnique2 {
        Map<Integer, Set<Integer>> countToNums = new HashMap<>();
        Map<Integer, Integer> numToCount = new HashMap<>();

        public FirstUnique2(int[] nums) {
            for (var num : nums) {
                add(num);
            }
        }

        public int showFirstUnique() {
            return countToNums.getOrDefault(1, new LinkedHashSet<>()).stream().findFirst().orElse(-1);
        }

        public void add(int value) {
            int num = value;
            var k = numToCount.getOrDefault(num, 0);
            countToNums.computeIfAbsent(k, ($) -> new LinkedHashSet<>()).remove(num);
            numToCount.put(num, numToCount.getOrDefault(num, 0) + 1);
            k = numToCount.getOrDefault(num, 0);
            countToNums.computeIfAbsent(k, ($) -> new LinkedHashSet<>()).add(num);
        }
    }

    static class FirstUnique3 {
        Queue<Integer> queue = new ArrayDeque<>();
        Map<Integer, Boolean> isUnique = new HashMap<>();

        public FirstUnique3(int[] nums) {
            for (int num : nums) {
                this.add(num);
            }
        }

        public int showFirstUnique() {
            while (!queue.isEmpty() && !isUnique.get(queue.peek())) {
                queue.remove();
            }
            if (!queue.isEmpty()) {
                return queue.peek();
            }
            return -1;
        }

        public void add(int value) {
            if (!isUnique.containsKey(value)) {
                isUnique.put(value, true);
                queue.add(value);
            } else {
                isUnique.put(value, false);
            }
        }
    }

    static class FirstUnique4 {
        Set<Integer> setQueue = new LinkedHashSet<>();
        Map<Integer, Boolean> isUnique = new HashMap<>();

        public FirstUnique4(int[] nums) {
            for (int num : nums) {
                this.add(num);
            }
        }

        public int showFirstUnique() {
            if (!setQueue.isEmpty()) {
                return setQueue.iterator().next();
            }
            return -1;
        }

        public void add(int value) {
            if (!isUnique.containsKey(value)) {
                isUnique.put(value, true);
                setQueue.add(value);
            } else if (isUnique.get(value)) {
                isUnique.put(value, false);
                setQueue.remove(value);
            }
        }
    }
}
