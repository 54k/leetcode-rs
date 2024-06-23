package leetcode_grind;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.TreeMap;

public class Day586 {
    // https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/description/
    static class Solution1 {
        public int longestSubarray(int[] nums, int limit) {
            var inc = new ArrayDeque<Integer>();
            var dec = new ArrayDeque<Integer>();
            var ans = 0;
            for (int i = 0, j = 0; i < nums.length; i++) {
                while (!inc.isEmpty() && nums[inc.peekLast()] >= nums[i]) {
                    inc.removeLast();
                }
                inc.addLast(i);
                while (!dec.isEmpty() && nums[dec.peekLast()] <= nums[i]) {
                    dec.removeLast();
                }
                dec.addLast(i);
                while (Math.abs(nums[inc.peekFirst()] - nums[dec.peekFirst()]) > limit) {
                    if (inc.peekFirst() <= j)
                        inc.removeFirst();
                    if (dec.peekFirst() <= j)
                        dec.removeFirst();
                    j++;
                }
                ans = Math.max(ans, i - j + 1);
            }
            return ans;
        }
    }

    static class Solution2 {
        public int longestSubarray(int[] nums, int limit) {
            PriorityQueue<int[]> maxHeap = new PriorityQueue<>((a, b) -> b[0] - a[0]);
            PriorityQueue<int[]> minHeap = new PriorityQueue<>(Comparator.comparingInt(a -> a[0]));
            int left = 0, maxLength = 0;

            for (int right = 0; right < nums.length; ++right) {
                maxHeap.offer(new int[] { nums[right], right });
                minHeap.offer(new int[] { nums[right], right });
                while (maxHeap.peek()[0] - minHeap.peek()[0] > limit) {
                    left = Math.min(maxHeap.peek()[1], minHeap.peek()[1]) + 1;
                    while (maxHeap.peek()[1] < left) {
                        maxHeap.poll();
                    }
                    while (minHeap.peek()[1] < left) {
                        minHeap.poll();
                    }
                }
                maxLength = Math.max(maxLength, right - left + 1);
            }
            return maxLength;
        }
    }

    static class Solution3 {
        public int longestSubarray(int[] nums, int limit) {
            TreeMap<Integer, Integer> window = new TreeMap<>();
            int left = 0, maxLength = 0;

            for (int right = 0; right < nums.length; ++right) {
                window.put(nums[right], window.getOrDefault(nums[right], 0) + 1);

                while (window.lastKey() - window.firstKey() > limit) {
                    window.put(nums[left], window.get(nums[left]) - 1);
                    if (window.get(nums[left]) == 0) {
                        window.remove(nums[left]);
                    }
                    ++left;
                }
                maxLength = Math.max(maxLength, right - left + 1);
            }
            return maxLength;
        }
    }

    // https://leetcode.com/problems/repeated-dna-sequences/description/
    static class Solution4 {
        public List<String> findRepeatedDnaSequences(String s) {
            var L = 10;
            HashSet<String> seen = new HashSet<>();
            HashSet<String> output = new HashSet<>();
            for (int i = 0; i < s.length() - L + 1; i++) {
                String tmp = s.substring(i, i + L);
                if (!seen.add(tmp))
                    output.add(tmp);
                seen.add(tmp);
            }
            return new ArrayList<>(output);
        }
    }

    // https://leetcode.com/problems/repeated-dna-sequences/description/
    static class Solution5 {
        public List<String> findRepeatedDnaSequences(String s) {
            int L = 10, n = s.length();
            if (n <= L)
                return new ArrayList<>();
            int a = 4, aL = (int) Math.pow(a, L);
            Map<Character, Integer> toInt = new HashMap() {
                {
                    put('A', 0);
                    put('C', 1);
                    put('G', 2);
                    put('T', 3);
                }
            };

            int[] nums = new int[n];
            for (int i = 0; i < n; i++)
                nums[i] = toInt.get(s.charAt(i));
            int h = 0;
            Set<Integer> seen = new HashSet<>();
            Set<String> output = new HashSet<>();
            for (int start = 0; start < n - L + 1; ++start) {
                if (start != 0)
                    h = h * a - nums[start - 1] * aL + nums[start + L - 1];
                else
                    for (int i = 0; i < L; ++i)
                        h = h * a + nums[i];

                if (seen.contains(h))
                    output.add(s.substring(start, start + L));
                seen.add(h);
            }
            return new ArrayList<String>(output);
        }
    }

    // https://leetcode.com/problems/repeated-dna-sequences/description/
    static class Solution5 {
        public List<String> findRepeatedDnaSequences(String s) {
            int L = 10, n = s.length();
            if (n <= L)
                return new ArrayList<>();
            Map<Character, Integer> toInt = new HashMap<>() {
                {
                    put('A', 0);
                    put('C', 1);
                    put('G', 2);
                    put('T', 3);
                }
            };

            int[] nums = new int[n];
            for (int i = 0; i < n; i++)
                nums[i] = toInt.get(s.charAt(i));

            int bitmask = 0;
            Set<Integer> seen = new HashSet<>();
            Set<String> output = new HashSet<>();
            for (int start = 0; start < n - L + 1; ++start) {
                if (start != 0) {
                    bitmask <<= 2;
                    bitmask |= nums[start + L - 1];
                    bitmask &= ~(3 << (2 * L));
                } else {
                    for (int i = 0; i < L; ++i) {
                        bitmask <<= 2;
                        bitmask |= nums[i];
                    }
                }

                if (seen.contains(bitmask))
                    output.add(s.substring(start, start + L));
                seen.add(bitmask);
            }
            return new ArrayList<String>(output);
        }
    }
}
