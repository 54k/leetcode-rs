package leetcode_grind;

import java.util.*;

public class Day672 {
    // https://leetcode.com/problems/different-ways-to-add-parentheses/description/?envType=daily-question&envId=2024-09-19
    static class Solution1 {
        public List<Integer> diffWaysToCompute(String expression) {
            List<Integer> results = new ArrayList<>();

            if (expression.length() == 0)
                return results;

            if (expression.length() == 1) {
                results.add(Integer.parseInt(expression));
                return results;
            }

            if (expression.length() == 2 && Character.isDigit(expression.charAt(0))) {
                results.add(Integer.parseInt(expression));
                return results;
            }

            for (int i = 0; i < expression.length(); i++) {
                char currentChar = expression.charAt(i);
                if (Character.isDigit(currentChar))
                    continue;

                List<Integer> leftResults = diffWaysToCompute(expression.substring(0, i));
                List<Integer> rightResults = diffWaysToCompute(expression.substring(i + 1));

                for (int leftValue : leftResults) {
                    for (int rightValue : rightResults) {
                        int computedResult = 0;

                        switch (currentChar) {
                            case '+':
                                computedResult = leftValue + rightValue;
                                break;
                            case '-':
                                computedResult = leftValue - rightValue;
                                break;
                            case '*':
                                computedResult = leftValue * rightValue;
                                break;
                        }

                        results.add(computedResult);
                    }
                }
            }

            return results;
        }
    }

    static class Solution2 {
        public List<Integer> diffWaysToCompute(String expression) {
            List<Integer>[][] memo = new ArrayList[expression.length()][expression.length()];
            return computeResults(expression, memo, 0, expression.length() - 1);
        }

        List<Integer> computeResults(String expression, List<Integer>[][] memo, int start, int end) {
            if (memo[start][end] != null) {
                return memo[start][end];
            }

            List<Integer> results = new ArrayList<>();

            if (start == end) {
                results.add(expression.charAt(start) - '0');
                return results;
            }

            if (end - start == 1 && Character.isDigit(expression.charAt(start))) {
                int tens = expression.charAt(start) - '0';
                int ones = expression.charAt(end) - '0';
                results.add(10 * tens + ones);
                return results;
            }

            for (int i = start; i <= end; i++) {
                char currentChar = expression.charAt(i);
                if (Character.isDigit(currentChar))
                    continue;

                List<Integer> leftResults = computeResults(
                        expression,
                        memo,
                        start,
                        i - 1);
                List<Integer> rightResults = computeResults(
                        expression,
                        memo,
                        i + 1,
                        end);
                for (int leftValue : leftResults) {
                    for (int rightValue : rightResults) {
                        switch (currentChar) {
                            case '+':
                                results.add(leftValue + rightValue);
                                break;
                            case '-':
                                results.add(leftValue - rightValue);
                                break;
                            case '*':
                                results.add(leftValue * rightValue);
                                break;
                        }
                    }
                }
            }

            memo[start][end] = results;
            return results;
        }
    }

    static class Solution3 {
        public List<Integer> diffWaysToCompute(String expression) {
            int n = expression.length();
            List<Integer>[][] dp = new ArrayList[n][n];
            initializeBaseCases(expression, dp);
            for (int length = 3; length <= n; length++) {
                for (int start = 0; start + length - 1 < n; start++) {
                    int end = start + length - 1;
                    processSubexpression(expression, dp, start, end);
                }
            }
            return dp[0][n - 1];
        }

        void initializeBaseCases(String expression, List<Integer>[][] dp) {
            int n = expression.length();
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n; j++) {
                    dp[i][j] = new ArrayList<>();
                }
            }

            for (int i = 0; i < n; i++) {
                if (Character.isDigit(expression.charAt(i))) {
                    int dig1 = expression.charAt(i) - '0';
                    if (i + 1 < n && Character.isDigit(expression.charAt(i + 1))) {
                        int dig2 = expression.charAt(i + 1) - '0';
                        int number = dig1 * 10 + dig2;
                        dp[i][i + 1].add(number);
                    }
                    dp[i][i].add(dig1);
                }
            }
        }

        void processSubexpression(
                String expression,
                List<Integer>[][] dp,
                int start,
                int end) {
            for (int split = start; split <= end; split++) {
                if (Character.isDigit(expression.charAt(split)))
                    continue;
                List<Integer> leftResults = dp[start][split - 1];
                List<Integer> rightResults = dp[split + 1][end];
                computeResults(
                        expression.charAt(split),
                        leftResults,
                        rightResults,
                        dp[start][end]);
            }
        }

        void computeResults(
                char op,
                List<Integer> leftResults,
                List<Integer> rightResults,
                List<Integer> results) {
            for (int leftValue : leftResults) {
                for (int rightValue : rightResults) {
                    switch (op) {
                        case '+':
                            results.add(leftValue + rightValue);
                            break;
                        case '-':
                            results.add(leftValue - rightValue);
                            break;
                        case '*':
                            results.add(leftValue * rightValue);
                            break;
                    }
                }
            }
        }
    }
}
