package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.TreeMap;

public class Day395 {
    // https://leetcode.com/problems/number-of-flowers-in-full-bloom/description/
    static class Solution {
        public int[] fullBloomFlowersHeap(int[][] flowers, int[] people) {
            int[] sortedPeople = Arrays.copyOf(people, people.length);
            Arrays.sort(sortedPeople);
            Arrays.sort(flowers, (a, b) -> Arrays.compare(a, b));

            Map<Integer, Integer> dic = new HashMap<>();
            PriorityQueue<Integer> heap = new PriorityQueue<>();

            int i = 0;
            for (var person : sortedPeople) {
                while (i < flowers.length && flowers[i][0] <= person) {
                    heap.add(flowers[i][1]);
                    i++;
                }

                while (!heap.isEmpty() && heap.peek() < person) {
                    heap.remove();
                }

                dic.put(person, heap.size());
            }

            int[] ans = new int[people.length];
            for (int j = 0; j < people.length; j++) {
                ans[j] = dic.get(people[j]);
            }

            return ans;
        }

        public int[] fullBloomFlowersDiffArray(int[][] flowers, int[] people) {
            TreeMap<Integer, Integer> diffArr = new TreeMap<>();
            diffArr.put(0, 0);

            for (int[] flower : flowers) {
                int start = flower[0];
                int end = flower[1] + 1;
                diffArr.put(start, diffArr.getOrDefault(start, 0) + 1);
                diffArr.put(end, diffArr.getOrDefault(end, 0) - 1);
            }

            List<Integer> positions = new ArrayList<>();
            List<Integer> prefix = new ArrayList<>();
            int curr = 0;

            for (int key : diffArr.keySet()) {
                positions.add(key);
                curr += diffArr.get(key);
                prefix.add(curr);
            }

            var binarySearch = new Object() {
                int apply(int target) {
                    int lo = 0;
                    int hi = positions.size();

                    while (lo < hi) {
                        int mid = (lo + hi) / 2;
                        if (positions.get(mid) <= target) {
                            lo = mid + 1;
                        } else {
                            hi = mid;
                        }
                    }

                    return lo;
                }
            };

            int[] ans = new int[people.length];
            for (int j = 0; j < ans.length; j++) {
                int i = binarySearch.apply(people[j]) - 1;
                ans[j] = prefix.get(i);
            }

            return ans;
        }

        public int[] fullBloomFlowersBinSearch(int[][] flowers, int[] people) {
            int n = flowers.length;
            int[] starts = new int[n];
            int[] ends = new int[n];
            for (int i = 0; i < flowers.length; i++) {
                starts[i] = flowers[i][0];
                ends[i] = flowers[i][1] + 1;
            }

            Arrays.sort(starts);
            Arrays.sort(ends);

            var upperBound = new Object() {
                int apply(int[] arr, int target) {
                    int lo = -1;
                    int hi = arr.length;

                    while (lo + 1 < hi) {
                        int mid = (lo + hi) / 2;
                        if (arr[mid] <= target) {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }

                    return hi;
                }
            };

            int[] ans = new int[people.length];
            for (int i = 0; i < people.length; i++) {
                int si = upperBound.apply(starts, people[i]);
                int ei = upperBound.apply(ends, people[i]);

                ans[i] = si - ei;
            }

            return ans;
        }
    }

}
