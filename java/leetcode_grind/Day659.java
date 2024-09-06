package leetcode_grind;

import java.util.*;

public class Day659 {
    public static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    // https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/description/?envType=daily-question&envId=2024-09-06
    static class Solution1 {
        public ListNode modifiedList(int[] nums, ListNode head) {
            var hs = new HashSet<Integer>();
            for (var n : nums) {
                hs.add(n);
            }
            var dummy = new ListNode(0, head);
            var dummyPtr = dummy;
            var ptr = head;
            while (ptr != null) {
                if (!hs.contains(ptr.val)) {
                    dummyPtr.next = ptr;
                    dummyPtr = dummyPtr.next;
                }
                var t = ptr;
                ptr = ptr.next;
                t.next = null;
            }
            return dummy.next;
        }
    }

    // https://leetcode.com/problems/subarrays-with-k-different-integers/description/
    static class Solution2 {
        public int subarraysWithKDistinct(int[] nums, int k) {
            return slidingWindowAtMost(nums, k) - slidingWindowAtMost(nums, k - 1);
        }

        int slidingWindowAtMost(int[] nums, int distinctK) {
            Map<Integer, Integer> freqMap = new HashMap<>();
            int left = 0, totalCount = 0;

            for (int right = 0; right < nums.length; right++) {
                freqMap.put(nums[right], freqMap.getOrDefault(nums[right], 0) + 1);

                while (freqMap.size() > distinctK) {
                    freqMap.put(nums[left], freqMap.get(nums[left]) - 1);
                    if (freqMap.get(nums[left]) == 0) {
                        freqMap.remove(nums[left]);
                    }
                    left++;
                }

                totalCount += (right - left + 1);
            }
            return totalCount;
        }
    }

    static class Solution3 {
        public int subarraysWithKDistinct(int[] nums, int k) {
            int[] distinctCount = new int[nums.length + 1];

            int totalCount = 0;
            int left = 0;
            int right = 0;
            int currCount = 0;

            while (right < nums.length) {
                if (distinctCount[nums[right++]]++ == 0) {
                    k--;
                }

                if (k < 0) {
                    --distinctCount[nums[left++]];
                    k++;
                    currCount = 0;
                }

                if (k == 0) {
                    while (distinctCount[nums[left]] > 1) {
                        --distinctCount[nums[left++]];
                        currCount++;
                    }
                    totalCount += (currCount + 1);
                }
            }
            return totalCount;
        }
    }

    // https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters/description/
    static class Solution4 {
        public int lengthOfLongestSubstringKDistinct(String s, int k) {
            Map<Character, Integer> count = new HashMap<>();

            int left = 0, right = 0, ans = 0;

            while (right < s.length()) {
                count.put(s.charAt(right), count.getOrDefault(s.charAt(right), 0) + 1);

                while (count.size() > k) {
                    count.put(s.charAt(left), count.get(s.charAt(left)) - 1);
                    if (count.get(s.charAt(left)) == 0) {
                        count.remove(s.charAt(left));
                    }
                    left++;
                }

                ans = Math.max(ans, right - left + 1);
                right++;
            }

            return ans;
        }
    }

    static class Solution5 {
        public int lengthOfLongestSubstringKDistinct(String s, int k) {
            int n = s.length();
            int maxSize = 0;
            Map<Character, Integer> counter = new HashMap<>();

            for (int right = 0; right < n; right++) {
                counter.put(s.charAt(right), counter.getOrDefault(s.charAt(right), 0) + 1);
                if (counter.size() <= k) {
                    maxSize++;
                } else {
                    counter.put(s.charAt(right - maxSize), counter.get(s.charAt(right - maxSize)) - 1);
                    if (counter.get(s.charAt(right - maxSize)) == 0) {
                        counter.remove(s.charAt(right - maxSize));
                    }
                }
            }

            return maxSize;
        }
    }

    static class Solution6 {
        public int lengthOfLongestSubstringKDistinct(String s, int k) {
            int n = s.length();
            if (k >= n) {
                return n;
            }

            int left = k, right = n;
            while (left < right) {
                int mid = (left + right + 1) / 2;

                if (isValid(s, mid, k)) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            return left;
        }

        boolean isValid(String s, int size, int k) {
            int n = s.length();

            Map<Character, Integer> counter = new HashMap<>();

            for (int i = 0; i < size; i++) {
                char c = s.charAt(i);
                counter.put(c, counter.getOrDefault(c, 0) + 1);
            }

            if (counter.size() <= k) {
                return true;
            }

            for (int i = size; i < n; i++) {
                char c1 = s.charAt(i);
                counter.put(c1, counter.getOrDefault(c1, 0) + 1);
                char c2 = s.charAt(i - size);
                counter.put(c2, counter.getOrDefault(c2, 0) - 1);
                if (counter.get(c2) == 0) {
                    counter.remove(c2);
                }
                if (counter.size() <= k) {
                    return true;
                }
            }
            return false;
        }
    }

    // https://leetcode.com/problems/dice-roll-simulation/description/
    static class Solution7 {
        int[][][] dp = new int[5001][7][16];
        int mod = (int) 1e9 + 7;

        public int dieSimulator(int n, int[] rollMax) {
            return helper(n, rollMax, 0, 0);
        }

        int helper(int n, int[] a, int prev, int cons) {
            if (n == 0) {
                return 1;
            }

            int answer = 0;
            if (dp[n][prev][cons] != 0) {
                return dp[n][prev][cons];
            }

            for (int i = 0; i < 6; i++) {
                if ((i + 1) == prev) {
                    if (a[i] > cons) {
                        answer = (answer + helper(n - 1, a, prev, cons + 1)) % mod;
                    }
                } else {
                    answer = (answer + helper(n - 1, a, i + 1, 1)) % mod;
                }
            }
            return dp[n][prev][cons] = answer;
        }
    }
}
