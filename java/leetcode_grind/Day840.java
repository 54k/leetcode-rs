package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.Random;

public class Day840 {
    // https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/description/
    static class Solution1 {
        public int minimumRecolors(String blocks, int k) {
            var ans = Integer.MAX_VALUE;
            var niggaz = 0;
            for (int i = 0; i < blocks.length(); i++) {
                if (blocks.charAt(i) == 'B') {
                    niggaz++;
                }
                if (i >= k) {
                    if (blocks.charAt(i - k) == 'B') {
                        niggaz--;
                    }
                }
                ans = Math.min(ans, k - niggaz);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/find-k-length-substrings-with-no-repeated-characters/description/
    static class Solution2 {
        public int numKLenSubstrNoRepeats(String s, int k) {
            var ans = 0;
            var hm = new HashMap<Character, Integer>();
            for (int i = 0; i < s.length(); i++) {
                var right = s.charAt(i);
                hm.put(right, hm.getOrDefault(right, 0) + 1);
                if (i >= k) {
                    var left = s.charAt(i - k);
                    hm.put(left, hm.get(left) - 1);
                    if (hm.get(left) == 0) {
                        hm.remove(left);
                    }
                }
                if (hm.size() == k) {
                    ans++;
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/top-k-frequent-elements/description/
    static class Solution3 {
        public int[] topKFrequent(int[] nums, int k) {
            if (k == nums.length) {
                return nums;
            }

            Map<Integer, Integer> count = new HashMap<>();
            for (int n : nums) {
                count.put(n, count.getOrDefault(n, 0) + 1);
            }

            Queue<Integer> heap = new PriorityQueue<>((n1, n2) -> count.get(n1) - count.get(n2));

            for (int n : count.keySet()) {
                heap.add(n);
                if (heap.size() > k)
                    heap.poll();
            }

            int[] top = new int[k];
            for (int i = k - 1; i >= 0; --i) {
                top[i] = heap.poll();
            }
            return top;
        }
    }

    static class Solution4 {
        int[] unique;
        Map<Integer, Integer> count;

        void swap(int a, int b) {
            int tmp = unique[a];
            unique[a] = unique[b];
            unique[b] = tmp;
        }

        int partition(int left, int right, int pivot_index) {
            int pivot_frequency = count.get(unique[pivot_index]);
            swap(pivot_index, right);
            int store_index = left;

            for (int i = left; i <= right; i++) {
                if (count.get(unique[i]) < pivot_frequency) {
                    swap(store_index, i);
                    store_index++;
                }
            }
            swap(store_index, right);
            return store_index;
        }

        void quickselect(int left, int right, int k_smallest) {
            if (left == right)
                return;

            Random random_num = new Random();
            int pivot_index = left + random_num.nextInt(right - left);

            pivot_index = partition(left, right, pivot_index);
            if (k_smallest == pivot_index) {
                return;
            } else if (k_smallest < pivot_index) {
                quickselect(left, pivot_index - 1, k_smallest);
            } else {
                quickselect(pivot_index + 1, right, k_smallest);
            }
        }

        public int[] topKFrequent(int[] nums, int k) {
            count = new HashMap<>();
            for (int num : nums) {
                count.put(num, count.getOrDefault(num, 0) + 1);
            }
            int n = count.size();
            unique = new int[n];
            int i = 0;
            for (int num : count.keySet()) {
                unique[i] = num;
                i++;
            }

            quickselect(0, n - 1, n - k);
            return Arrays.copyOfRange(unique, n - k, n);
        }
    }
}
