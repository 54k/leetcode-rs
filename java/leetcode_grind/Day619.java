package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;

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
}
