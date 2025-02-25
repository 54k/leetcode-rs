package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.PriorityQueue;
import java.util.TreeSet;
import java.util.stream.Collectors;

public class Day829 {
    // https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/description/
    static class Solution1 {
        public int numOfSubarrays(int[] arr) {
            int MOD = (int) Math.pow(10, 9) + 7;
            int n = arr.length;
            for (int num = 0; num < n; num++) {
                arr[num] %= 2;
            }

            int[] dpEven = new int[n], dpOdd = new int[n];

            if (arr[n - 1] == 0)
                dpEven[n - 1] = 1;
            else
                dpOdd[n - 1] = 1;

            for (int num = n - 2; num >= 0; num--) {
                if (arr[num] == 1) {
                    dpOdd[num] = (1 + dpEven[num + 1]) % MOD;
                    dpEven[num] = dpOdd[num + 1];
                } else {
                    dpEven[num] = (1 + dpEven[num + 1]) % MOD;
                    dpOdd[num] = dpOdd[num + 1];
                }
            }

            int count = 0;
            for (int oddCount : dpOdd) {
                count += oddCount;
                count %= MOD;
            }
            return count;
        }
    }

    static class Solution2 {
        public int numOfSubarrays(int[] arr) {
            final int MOD = 1_000_000_007;
            int count = 0, prefixSum = 0;
            int oddCount = 0, evenCount = 1;
            for (int num : arr) {
                prefixSum += num;
                if (prefixSum % 2 == 0) {
                    count += oddCount;
                    evenCount++;
                } else {
                    count += evenCount;
                    oddCount++;
                }
                count %= MOD;
            }
            return count;
        }
    }

    static class Pair<F, S> {
        F key;
        S value;

        public Pair(F k, S v) {
            key = k;
            value = v;
        }

        F getKey() {
            return key;
        }

        S getValue() {
            return value;
        }
    }

    // https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/description/
    static class Solution3 {

        public List<Integer> busiestServers(int k, int[] arrival, int[] load) {
            int[] count = new int[k];

            TreeSet<Integer> free = new TreeSet<Integer>();
            PriorityQueue<Pair<Integer, Integer>> busy = new PriorityQueue<>((a, b) -> a.getKey() - b.getKey());

            for (int i = 0; i < k; i++) {
                free.add(i);
            }

            for (int i = 0; i < arrival.length; i++) {
                int start = arrival[i];
                while (!busy.isEmpty() && busy.peek().getKey() <= start) {
                    Pair<Integer, Integer> curr = busy.remove();
                    int serverId = curr.getValue();
                    free.add(serverId);
                }

                if (!free.isEmpty()) {
                    Integer busyId = free.ceiling(i % k);
                    if (busyId == null) {
                        busyId = free.first();
                    }
                    free.remove(busyId);
                    busy.add(new Pair<>(start + load[i], busyId));
                    count[busyId]++;
                }
            }

            int maxJob = Collections.max(Arrays.stream(count).boxed().toList());
            List<Integer> answer = new ArrayList<>();
            for (int i = 0; i < k; ++i) {
                if (count[i] == maxJob) {
                    answer.add(i);
                }
            }
            return answer;
        }
    }

    static class Solution4 {
        public List<Integer> busiestServers(int k, int[] arrival, int[] load) {
            int[] count = new int[k];
            PriorityQueue<Integer> free = new PriorityQueue<>((a, b) -> a - b);
            PriorityQueue<Pair<Integer, Integer>> busy = new PriorityQueue<>((a, b) -> a.getKey() - b.getKey());

            for (int i = 0; i < k; i++) {
                free.add(i);
            }

            for (int i = 0; i < arrival.length; i++) {
                int start = arrival[i];

                while (!busy.isEmpty() && busy.peek().getKey() <= start) {
                    Pair<Integer, Integer> curr = busy.remove();
                    int serverId = curr.getValue();
                    int modifiedId = ((serverId - i) % k + k) % k + i;
                    free.add(modifiedId);
                }

                if (!free.isEmpty()) {
                    int busyId = free.peek() % k;
                    free.remove();
                    busy.add(new Pair<>(start + load[i], busyId));
                    count[busyId]++;
                }
            }

            int maxJob = Collections.max(Arrays.stream(count).boxed().collect(Collectors.toList()));
            List<Integer> answer = new ArrayList<>();
            for (int i = 0; i < k; i++) {
                if (count[i] == maxJob) {
                    answer.add(i);
                }
            }
            return answer;
        }
    }
}
