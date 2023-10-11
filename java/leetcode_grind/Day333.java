package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.PriorityQueue;
import java.util.TreeMap;
import java.util.function.Function;

public class Day333 {
    // https://leetcode.com/problems/number-of-flowers-in-full-bloom/description
    static class Solution {
        public int[] fullBloomFlowersPriorityQueue(int[][] flowers, int[] people) {
            var sortedPeople = people.clone();
            Arrays.sort(sortedPeople);
            Arrays.sort(flowers, (f1, f2) -> f1[0] - f2[0]);

            var personToBloomedFlowers = new HashMap<Integer, Integer>();
            var pq = new PriorityQueue<Integer>();
            var i = 0;
            for (var person : sortedPeople) {
                while (i < flowers.length && flowers[i][0] <= person) {
                    pq.add(flowers[i][1]);
                    i++;
                }
                while (!pq.isEmpty() && pq.peek() < person) {
                    pq.poll();
                }
                personToBloomedFlowers.put(person, pq.size());
            }

            i = 0;
            var ans = new int[people.length];
            for (var person : people) {
                ans[i++] = personToBloomedFlowers.get(person);
            }
            return ans;
        }

        public int[] fullBloomFlowersLineSweep(int[][] flowers, int[] people) {
            var sortedMap = new TreeMap<Integer, Integer>();
            sortedMap.put(0, 0);
            for (var flower : flowers) {
                var start = flower[0];
                var end = flower[1] + 1;
                sortedMap.put(start, sortedMap.getOrDefault(start, 0) + 1);
                sortedMap.put(end, sortedMap.getOrDefault(end, 0) - 1);
            }

            var i = 0;
            var curr = 0;
            var positions = new int[sortedMap.size()];
            var prefix = new int[sortedMap.size()];
            for (var entry : sortedMap.entrySet()) {
                curr += entry.getValue();
                positions[i] = entry.getKey();
                prefix[i] = curr;
                i++;
            }

            Function<Integer, Integer> upperBound = (target) -> {
                var lo = 0;
                var hi = positions.length;
                while (lo < hi) {
                    var mid = (lo + hi) / 2;
                    if (positions[mid] <= target) {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                return lo;
            };

            var ans = new int[people.length];
            for (int j = 0; j < people.length; j++) {
                var idx = upperBound.apply(people[j]) - 1;
                ans[j] = prefix[idx];
            }

            return ans;
        }

        public int[] fullBloomFlowersBinarySearch(int[][] flowers, int[] people) {
            var startBloom = new int[flowers.length];
            var endBloom = new int[flowers.length];
            var i = 0;
            for (var flower : flowers) {
                startBloom[i] = flower[0];
                endBloom[i] = flower[1] + 1;
                i++;
            }
            Arrays.sort(startBloom);
            Arrays.sort(endBloom);

            var binSearch = new Object() {
                int apply(int[] arr, int target) {
                    var lo = 0;
                    var hi = arr.length;
                    while (lo < hi) {
                        var mid = (lo + hi) / 2;
                        if (arr[mid] <= target) {
                            lo = mid + 1;
                        } else {
                            hi = mid;
                        }
                    }
                    return lo;
                }
            };

            var ans = new int[people.length];
            for (int j = 0; j < people.length; j++) {
                var person = people[j];
                var start = binSearch.apply(startBloom, person);
                var end = binSearch.apply(endBloom, person);
                ans[j] = start - end;
            }
            return ans;
        }
    }
}
