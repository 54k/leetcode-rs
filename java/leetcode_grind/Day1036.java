package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class Day1036 {
    // https://leetcode.com/problems/implement-router/description/?envType=daily-question&envId=2025-09-20
    static class Router {
        int size;

        Map<Integer, List<Integer>> counts;
        Map<Long, int[]> packets;
        Queue<Long> queue;

        public Router(int memoryLimit) {
            size = memoryLimit;
            packets = new HashMap<>();
            counts = new HashMap<>();
            queue = new LinkedList<>();
        }

        public boolean addPacket(int source, int destination, int timestamp) {
            long key = encode(source, destination, timestamp);
            if (packets.containsKey(key)) {
                return false;
            }
            if (packets.size() >= size) {
                forwardPacket();
            }

            packets.put(key, new int[] { source, destination, timestamp });
            queue.offer(key);

            counts.putIfAbsent(destination, new ArrayList<>());
            counts.get(destination).add(timestamp);
            return true;
        }

        public int[] forwardPacket() {
            if (packets.isEmpty()) {
                return new int[] {};
            }

            final long key = queue.poll();
            final int[] packet = packets.remove(key);

            if (packet == null) {
                return new int[] {};
            }

            final int destination = packet[1];
            List<Integer> list = counts.get(destination);
            list.remove(0);
            return packet;
        }

        public int getCount(int destination, int startTime, int endTime) {
            List<Integer> list = counts.get(destination);
            if (list == null || list.isEmpty()) {
                return 0;
            }
            int left = lowerBound(list, startTime);
            int right = upperBound(list, endTime);
            return right - left;
        }

        long encode(int source, int destination, int timestamp) {
            return ((long) source << 40) | ((long) destination << 20) | timestamp;
        }

        int lowerBound(List<Integer> list, int target) {
            int low = 0, high = list.size();
            while (low < high) {
                int mid = (low + high) >>> 1;
                if (list.get(mid) < target) {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            return low;
        }

        int upperBound(List<Integer> list, int target) {
            int low = 0, high = list.size();
            while (low < high) {
                int mid = (low + high) >>> 1;
                if (list.get(mid) <= target) {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            return low;
        }
    }

    /**
     * Your Router object will be instantiated and called as such:
     * Router obj = new Router(memoryLimit);
     * boolean param_1 = obj.addPacket(source,destination,timestamp);
     * int[] param_2 = obj.forwardPacket();
     * int param_3 = obj.getCount(destination,startTime,endTime);
     */

}
