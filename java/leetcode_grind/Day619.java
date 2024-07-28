package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.Set;

public class Day619 {
    // https://leetcode.com/problems/second-minimum-time-to-reach-destination/description/?envType=daily-question&envId=2024-07-28
    static class Solution1 {
        public int secondMinimum(int n, int[][] edges, int time, int change) {
            Map<Integer, List<Integer>> adj = new HashMap<>();
            for (int[] edge : edges) {
                int a = edge[0], b = edge[1];
                adj.computeIfAbsent(a, value -> new ArrayList<Integer>()).add(b);
                adj.computeIfAbsent(b, value -> new ArrayList<Integer>()).add(a);
            }
            int[] dist1 = new int[n + 1], dist2 = new int[n + 1], freq = new int[n + 1];
            for (int i = 1; i <= n; i++) {
                dist1[i] = dist2[i] = Integer.MAX_VALUE;
                freq[i] = 0;
            }

            PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[1] - b[1]);
            pq.offer(new int[] { 1, 0 });
            dist1[1] = 0;

            while (!pq.isEmpty()) {
                int[] temp = pq.poll();
                int node = temp[0];
                int time_taken = temp[1];

                freq[node]++;

                if (freq[node] == 2 && node == n) {
                    return time_taken;
                }

                if ((time_taken / change) % 2 == 1) {
                    time_taken = change * (time_taken / change + 1) + time;
                } else {
                    time_taken = time_taken + time;
                }

                if (!adj.containsKey(node)) {
                    continue;
                }

                for (int neighbor : adj.get(node)) {
                    if (freq[neighbor] == 2)
                        continue;

                    if (dist1[neighbor] > time_taken) {
                        dist2[neighbor] = dist1[neighbor];
                        dist1[neighbor] = time_taken;
                        pq.offer(new int[] { neighbor, time_taken });
                    } else if (dist2[neighbor] > time_taken && dist1[neighbor] != time_taken) {
                        dist2[neighbor] = time_taken;
                        pq.offer(new int[] { neighbor, time_taken });
                    }
                }
            }
            return 0;
        }
    }

    static class Solution2 {
        public int secondMinimum(int n, int[][] edges, int time, int change) {
            Map<Integer, List<Integer>> adj = new HashMap<>();
            for (int[] edge : edges) {
                int a = edge[0], b = edge[1];
                adj.computeIfAbsent(a, value -> new ArrayList<Integer>()).add(b);
                adj.computeIfAbsent(b, value -> new ArrayList<Integer>()).add(a);
            }
            int[] dist1 = new int[n + 1], dist2 = new int[n + 1];
            for (int i = 1; i <= n; i++) {
                dist1[i] = dist2[i] = -1;
            }
            Queue<int[]> queue = new LinkedList<>();
            queue.offer(new int[] { 1, 1 });
            dist1[1] = 0;

            while (!queue.isEmpty()) {
                int[] temp = queue.poll();
                int node = temp[0];
                int freq = temp[1];

                int timeTaken = freq == 1 ? dist1[node] : dist2[node];

                if ((timeTaken / change) % 2 == 1) {
                    timeTaken = change * (timeTaken / change + 1) + time;
                } else {
                    timeTaken = timeTaken + time;
                }

                if (!adj.containsKey(node)) {
                    continue;
                }

                for (int neighbor : adj.get(node)) {
                    if (dist1[neighbor] == -1) {
                        dist1[neighbor] = timeTaken;
                        queue.offer(new int[] { neighbor, 1 });
                    } else if (dist2[neighbor] == -1 && dist1[neighbor] != timeTaken) {
                        if (neighbor == n)
                            return timeTaken;
                        dist2[neighbor] = timeTaken;
                        queue.offer(new int[] { neighbor, 2 });
                    }
                }
            }
            return 0;
        }
    }

    // https://leetcode.com/problems/multiply-strings/description/
    static class Solution3 {
        StringBuilder sumResults(ArrayList<ArrayList<Integer>> results) {
            ArrayList<Integer> answer = new ArrayList<>(results.get(results.size() - 1));
            ArrayList<Integer> newAnswer = new ArrayList<>();

            for (int j = 0; j < results.size() - 1; j++) {
                ArrayList<Integer> result = new ArrayList<>(results.get(j));
                newAnswer = new ArrayList<>();
                int carry = 0;

                for (int i = 0; i < answer.size() || i < result.size(); ++i) {
                    int digit1 = i < result.size() ? result.get(i) : 0;
                    int digit2 = i < answer.size() ? answer.get(i) : 0;
                    int sum = digit1 + digit2 + carry;
                    carry = sum / 10;
                    newAnswer.add(sum % 10);
                }

                if (carry != 0) {
                    newAnswer.add(carry);
                }
                answer = newAnswer;
            }

            StringBuilder finalAnswer = new StringBuilder();
            for (int digit : answer) {
                finalAnswer.append(digit);
            }
            return finalAnswer;
        }

        ArrayList<Integer> multiplyOneDigit(StringBuilder firstNumber, char secondNumberDigit, int numZeros) {
            ArrayList<Integer> currentResult = new ArrayList<>();
            for (int i = 0; i < numZeros; ++i) {
                currentResult.add(0);
            }
            int carry = 0;
            for (int i = 0; i < firstNumber.length(); i++) {
                char firstNumberDigit = firstNumber.charAt(i);
                int multiplication = (secondNumberDigit - '0') * (firstNumberDigit - '0') + carry;
                carry = multiplication / 10;
                currentResult.add(multiplication % 10);
            }
            if (carry != 0) {
                currentResult.add(carry);
            }
            return currentResult;
        }

        public String multiply(String num1, String num2) {
            if (num1.equals("0") || num2.equals("0")) {
                return "0";
            }
            StringBuilder firstNumber = new StringBuilder(num1);
            StringBuilder secondNumber = new StringBuilder(num2);
            firstNumber.reverse();
            secondNumber.reverse();

            ArrayList<ArrayList<Integer>> results = new ArrayList<>();
            for (int i = 0; i < secondNumber.length(); i++) {
                results.add(multiplyOneDigit(firstNumber, secondNumber.charAt(i), i));
            }

            StringBuilder answer = sumResults(results);
            answer.reverse();
            return answer.toString();
        }
    }

    static class Solution4 {
        ArrayList<Integer> addStrings(ArrayList<Integer> num1, ArrayList<Integer> num2) {
            ArrayList<Integer> ans = new ArrayList<>();
            int carry = 0;

            for (int i = 0; i < num1.size() || i < num2.size(); ++i) {
                int digit1 = i < num1.size() ? num1.get(i) : 0;
                int digit2 = i < num2.size() ? num2.get(i) : 0;

                int sum = digit1 + digit2 + carry;
                carry = sum / 10;
                ans.add(sum % 10);
            }

            if (carry != 0) {
                ans.add(carry);
            }
            return ans;
        }

        ArrayList<Integer> multiplyOneDigit(StringBuilder firstNumber, char secondNumberDigit, int numZeros) {
            ArrayList<Integer> currentResult = new ArrayList<>();
            for (int i = 0; i < numZeros; i++) {
                currentResult.add(0);
            }
            int carry = 0;
            for (int i = 0; i < firstNumber.length(); i++) {
                char firstNumberDigit = firstNumber.charAt(i);
                int multiplication = (secondNumberDigit - '0') * (firstNumberDigit - '0') + carry;
                carry = multiplication / 10;
                currentResult.add(multiplication % 10);
            }
            if (carry != 0) {
                currentResult.add(carry);
            }
            return currentResult;
        }

        public String multiply(String num1, String num2) {
            if (num1.equals("0") || num2.equals("0")) {
                return "0";
            }
            StringBuilder firstNumber = new StringBuilder(num1);
            StringBuilder secondNumber = new StringBuilder(num2);

            firstNumber.reverse();
            secondNumber.reverse();

            int N = firstNumber.length() + secondNumber.length();
            ArrayList<Integer> ans = new ArrayList<>();
            for (int i = 0; i < N; i++) {
                ans.add(0);
            }

            for (int i = 0; i < secondNumber.length(); i++) {
                ans = addStrings(multiplyOneDigit(firstNumber, secondNumber.charAt(i), i), ans);
            }

            if (ans.get(ans.size() - 1) == 0) {
                ans.remove(ans.size() - 1);
            }
            StringBuilder answer = new StringBuilder();
            for (int i = ans.size() - 1; i >= 0; --i) {
                answer.append(ans.get(i));
            }
            return answer.toString();
        }
    }

    static class Solution5 {
        public String multiply(String num1, String num2) {
            if (num1.equals("0") || num2.equals("0")) {
                return "0";
            }

            int len1 = num1.length();
            int len2 = num2.length();
            int len = len1 + len2;
            int[] product = new int[len];

            for (int i = num1.length() - 1; i >= 0; i--) {
                int a = num1.charAt(i) - '0';
                for (int j = num2.length() - 1; j >= 0; j--) {
                    int b = num2.charAt(j) - '0';
                    int idx = i + j + 1;
                    int p = a * b + product[idx];
                    product[idx] = p % 10;
                    product[idx - 1] += p / 10;
                }
            }

            int start = product[0] != 0 ? 0 : 1;
            StringBuilder result = new StringBuilder();
            for (int i = start; i < len; i++) {
                result.append(product[i]);
            }
            return result.toString();
        }
    }

    static class Solution6 {
        public String multiply(String num1, String num2) {
            if (num1.equals("0") || num2.equals("0")) {
                return "0";
            }

            StringBuilder firstNumber = new StringBuilder(num1);
            StringBuilder secondNumber = new StringBuilder(num2);
            firstNumber.reverse();
            secondNumber.reverse();

            int N = firstNumber.length() + secondNumber.length();
            StringBuilder answer = new StringBuilder();
            for (int i = 0; i < N; i++) {
                answer.append(0);
            }

            for (int place2 = 0; place2 < secondNumber.length(); place2++) {
                int digit2 = secondNumber.charAt(place2) - '0';

                for (int place1 = 0; place1 < firstNumber.length(); place1++) {
                    int digit1 = firstNumber.charAt(place1) - '0';
                    int currentPos = place1 + place2;

                    int carry = answer.charAt(currentPos) - '0';
                    int multiplication = digit1 * digit2 + carry;

                    answer.setCharAt(currentPos, (char) ((multiplication % 10) + '0'));
                    int value = (answer.charAt(currentPos + 1) - '0') + multiplication / 10;
                    answer.setCharAt(currentPos + 1, (char) (value + '0'));
                }
            }

            if (answer.charAt(answer.length() - 1) == '0') {
                answer.deleteCharAt(answer.length() - 1);
            }
            answer.reverse();
            return answer.toString();
        }
    }

    // https://leetcode.com/problems/jump-game/description/
    static class Solution7 {
        boolean canJumpFromPosition(int position, int[] nums) {
            if (position == nums.length - 1) {
                return true;
            }

            int furthestJump = Math.min(position + nums[position], nums.length - 1);
            for (int nextPosition = position + 1; nextPosition <= furthestJump; nextPosition++) {
                if (canJumpFromPosition(nextPosition, nums)) {
                    return true;
                }
            }
            return false;
        }

        public boolean canJump(int[] nums) {
            return canJumpFromPosition(0, nums);
        }
    }

    static class Solution8 {
        enum Index {
            GOOD, BAD, UNKNOWN,
        }

        Index[] memo;

        boolean canJumpFromPosition(int position, int[] nums) {
            if (memo[position] != Index.UNKNOWN) {
                return memo[position] == Index.GOOD ? true : false;
            }
            int furthestJump = Math.min(position + nums[position], nums.length - 1);
            for (int nextPosition = position + 1; nextPosition <= furthestJump; nextPosition++) {
                if (canJumpFromPosition(nextPosition, nums)) {
                    memo[position] = Index.GOOD;
                    return true;
                }
            }
            memo[position] = Index.BAD;
            return false;
        }

        public boolean canJump(int[] nums) {
            memo = new Index[nums.length];
            for (int i = 0; i < memo.length; i++) {
                memo[i] = Index.UNKNOWN;
            }
            memo[memo.length - 1] = Index.GOOD;
            return canJumpFromPosition(0, nums);
        }
    }

    static class Solution9 {
        enum Index {
            GOOD, BAD, UNKNOWN,
        }

        public boolean canJump(int[] nums) {
            Index[] memo = new Index[nums.length];
            for (int i = 0; i < nums.length; i++) {
                memo[i] = Index.UNKNOWN;
            }
            memo[memo.length - 1] = Index.GOOD;
            for (int i = nums.length - 2; i >= 0; i--) {
                int furthestJump = Math.min(i + nums[i], nums.length - 1);
                for (int j = i + 1; j <= furthestJump; j++) {
                    if (memo[j] == Index.GOOD) {
                        memo[i] = Index.GOOD;
                        break;
                    }
                }
            }
            return memo[0] == Index.GOOD;
        }
    }

    // https://leetcode.com/problems/minimum-window-substring/description/
    static class Solution10 {
        public String minWindow(String s, String t) {
            if (s.length() == 0 || t.length() == 0) {
                return "";
            }

            Map<Character, Integer> dictT = new HashMap<Character, Integer>();
            for (int i = 0; i < t.length(); i++) {
                int count = dictT.getOrDefault(t.charAt(i), 0);
                dictT.put(t.charAt(i), count + 1);
            }

            int required = dictT.size();
            int l = 0, r = 0;
            int formed = 0;

            Map<Character, Integer> windowCounts = new HashMap<Character, Integer>();
            int[] ans = { -1, 0, 0 };
            while (r < s.length()) {
                char c = s.charAt(r);
                int count = windowCounts.getOrDefault(c, 0);
                windowCounts.put(c, count + 1);

                if (dictT.containsKey(c) && windowCounts.get(c).intValue() == dictT.get(c).intValue()) {
                    formed++;
                }

                while (l <= r && formed == required) {
                    c = s.charAt(l);
                    if (ans[0] == -1 || r - l + 1 < ans[0]) {
                        ans[0] = r - l + 1;
                        ans[1] = l;
                        ans[2] = r;
                    }

                    windowCounts.put(c, windowCounts.get(c) - 1);
                    if (dictT.containsKey(c) && windowCounts.get(c).intValue() < dictT.get(c).intValue()) {
                        formed--;
                    }

                    l++;
                }

                r++;
            }
            return ans[0] == -1 ? "" : s.substring(ans[1], ans[2] + 1);
        }
    }

    // https://leetcode.com/problems/read-n-characters-given-read4/description/
    /**
     * The read4 API is defined in the parent class Reader4.
     * int read4(char[] buf4);
     */

    public class Solution11 {/* extends Reader4 { */
        int bufPtr = 0;
        int bufCnt = 0;
        char[] buf4 = new char[4];

        int read4(char[] buf4) {
            return 0;
        }

        /**
         * @param buf Destination buffer
         * @param n   Number of characters to read
         * @return The number of actual characters read
         */
        public int read(char[] buf, int n) {
            int ptr = 0;
            while (ptr < n) {
                if (bufPtr == 0) {
                    bufCnt = read4(buf4);
                }
                if (bufCnt == 0) {
                    break;
                }
                while (ptr < n && bufPtr < bufCnt) {
                    buf[ptr++] = buf4[bufPtr++];
                }
                if (bufPtr >= bufCnt)
                    bufPtr = 0;
            }
            return ptr;
        }
    }

    // https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/description/
    static class Solution12 {
        public int lengthOfLongestSubstringTwoDistinct(String s) {
            int n = s.length();
            if (n < 3)
                return n;

            int left = 0;
            int right = 0;
            HashMap<Character, Integer> hashmap = new HashMap<Character, Integer>();
            int max_len = 2;
            while (right < n) {
                hashmap.put(s.charAt(right), right++);

                if (hashmap.size() == 3) {
                    int del_idx = Collections.min(hashmap.values());
                    hashmap.remove(s.charAt(del_idx));
                    left = del_idx + 1;
                }

                max_len = Math.max(max_len, right - left);
            }
            return max_len;
        }
    }

    // https://leetcode.com/problems/missing-ranges/description/
    static class Solution13 {
        public List<List<Integer>> findMissingRanges(int[] nums, int lower, int upper) {
            int n = nums.length;
            List<List<Integer>> missingRanges = new ArrayList<>();
            if (n == 0) {
                missingRanges.add(Arrays.asList(lower, upper));
                return missingRanges;
            }

            if (lower < nums[0]) {
                missingRanges.add(Arrays.asList(lower, nums[0] - 1));
            }

            for (int i = 0; i < n - 1; i++) {
                if (nums[i + 1] - nums[i] <= 1) {
                    continue;
                }
                missingRanges.add(Arrays.asList(nums[i] + 1, nums[i + 1] - 1));
            }

            if (upper > nums[n - 1]) {
                missingRanges.add(Arrays.asList(nums[n - 1] + 1, upper));
            }

            return missingRanges;
        }
    }

    // https://leetcode.com/problems/next-closest-time/description/
    static class Solution14 {
        public String nextClosestTime(String time) {
            int cur = 60 * Integer.parseInt(time.substring(0, 2));
            cur += Integer.parseInt(time.substring(3));
            Set<Integer> allowed = new HashSet<>();
            for (char c : time.toCharArray()) {
                if (c != ':') {
                    allowed.add(c - '0');
                }
            }

            while (true) {
                cur = (cur + 1) % (24 * 60);
                int[] digits = new int[] { cur / 60 / 10, cur / 60 % 10, cur % 60 / 10, cur % 60 % 10 };

                search: {
                    for (int d : digits) {
                        if (!allowed.contains(d)) {
                            break search;
                        }
                    }
                    return String.format("%02d:%02d", cur / 60, cur % 60);
                }
            }
        }
    }

    static class Solution15 {
        public String nextClosestTime(String time) {
            int start = 60 * Integer.parseInt(time.substring(0, 2));
            start += Integer.parseInt(time.substring(3));
            int ans = start;
            int elapsed = 24 * 60;
            Set<Integer> allowed = new HashSet<>();
            for (char c : time.toCharArray()) {
                if (c != ':') {
                    allowed.add(c - '0');
                }
            }

            for (int h1 : allowed) {
                for (int h2 : allowed) {
                    if (h1 * 10 + h2 < 24) {
                        for (int m1 : allowed) {
                            for (int m2 : allowed) {
                                if (m1 * 10 + m2 < 60) {
                                    int cur = 60 * (h1 * 10 + h2) + (m1 * 10 + m2);
                                    int candElapsed = Math.floorMod(cur - start, 24 * 60);
                                    if (0 < candElapsed && candElapsed < elapsed) {
                                        ans = cur;
                                        elapsed = candElapsed;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            return String.format("%02d:%02d", ans / 60, ans % 60);
        }
    }
}
