package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Day932 {
    // https://leetcode.com/problems/lexicographical-numbers/description/?envType=daily-question&envId=2025-06-08
    static class Solution1 {
        public List<Integer> lexicalOrder(int n) {
            List<Integer> lexicographicalNumbers = new ArrayList<>();
            for (int start = 1; start <= 9; start++) {
                generateLexicalNumbers(start, n, lexicographicalNumbers);
            }
            return lexicographicalNumbers;
        }

        void generateLexicalNumbers(int currentNumber, int limit, List<Integer> result) {
            if (currentNumber > limit)
                return;
            result.add(currentNumber);

            for (int nextDigit = 0; nextDigit <= 9; nextDigit++) {
                int nextNumber = currentNumber * 10 + nextDigit;
                generateLexicalNumbers(nextNumber, limit, result);
            }
        }
    }

    static class Solution2 {
        public List<Integer> lexicalOrder(int n) {
            List<Integer> lexicographicalNumbers = new ArrayList<>();
            int currentNumber = 1;

            for (int i = 0; i < n; i++) {
                lexicographicalNumbers.add(currentNumber);

                if (currentNumber * 10 <= n) {
                    currentNumber *= 10;
                } else {
                    while (currentNumber % 10 == 9 || currentNumber >= n) {
                        currentNumber /= 10;
                    }
                    currentNumber += 1;
                }
            }

            return lexicographicalNumbers;
        }
    }

    // https://leetcode.com/problems/verify-preorder-sequence-in-binary-search-tree/description/?envType=weekly-question&envId=2025-06-08
    static class Solution3 {
        public boolean verifyPreorder(int[] preorder) {
            int minLimit = Integer.MIN_VALUE;
            Stack<Integer> stack = new Stack<>();
            for (int num : preorder) {
                while (!stack.isEmpty() && stack.peek() < num) {
                    minLimit = stack.pop();
                }
                if (num <= minLimit) {
                    return false;
                }
                stack.push(num);
            }
            return true;
        }
    }

    static class Solution4 {
        public boolean verifyPreorder(int[] preorder) {
            int minLimit = Integer.MIN_VALUE;
            int i = 0;
            for (int num : preorder) {
                while (i > 0 && preorder[i - 1] < num) {
                    minLimit = preorder[i - 1];
                    i--;
                }
                if (num <= minLimit) {
                    return false;
                }
                preorder[i] = num;
                i++;
            }
            return true;
        }
    }

    static class Solution5 {
        public boolean verifyPreorder(int[] preorder) {
            int[] i = { 0 };
            return helper(preorder, i, Integer.MIN_VALUE, Integer.MAX_VALUE);
        }

        boolean helper(int[] preorder, int[] i, int minLimit, int maxLimit) {
            if (i[0] == preorder.length) {
                return true;
            }
            int root = preorder[i[0]];
            if (root <= minLimit || root >= maxLimit) {
                return false;
            }

            i[0]++;
            boolean left = helper(preorder, i, minLimit, root);
            boolean right = helper(preorder, i, root, maxLimit);
            return left || right;
        }
    }
}
