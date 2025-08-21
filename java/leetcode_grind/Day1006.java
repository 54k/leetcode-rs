package leetcode_grind;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Stack;

public class Day1006 {

    // https://leetcode.com/problems/count-submatrices-with-all-ones/description/?envType=daily-question&envId=2025-08-21
    static class Solution1 {
        public int numSubmat(int[][] mat) {
            int m = mat.length;
            int n = mat[0].length;
            int res = 0;
            int[][] row = new int[m][n];

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (j == 0) {
                        row[i][j] = mat[i][j];
                    } else {
                        row[i][j] = mat[i][j] == 0 ? 0 : row[i][j - 1] + 1;
                    }
                    int cur = row[i][j];
                    for (int k = i; k >= 0; k--) {
                        cur = Math.min(cur, row[k][j]);
                        if (cur == 0) {
                            break;
                        }
                        res += cur;
                    }
                }
            }
            return res;
        }
    }

    static class Solution2 {
        public int numSubmat(int[][] mat) {
            int n = mat[0].length;
            int[] heights = new int[n];
            int res = 0;
            for (int[] row : mat) {
                for (int i = 0; i < n; i++) {
                    heights[i] = row[i] == 0 ? 0 : heights[i] + 1;
                }

                Stack<int[]> stack = new Stack<>();
                stack.push(new int[] { -1, 0, -1 });

                for (int i = 0; i < n; i++) {
                    int h = heights[i];
                    while (stack.peek()[2] >= h) {
                        stack.pop();
                    }
                    int[] top = stack.peek();
                    int j = top[0];
                    int prev = top[1];
                    int cur = prev + (i - j) * h;
                    stack.push(new int[] { i, cur, h });
                    res += cur;
                }
            }
            return res;
        }
    }

    // https://leetcode.com/problems/word-squares/description/
    static class Solution3 {
        int N = 0;
        String[] words = null;
        Map<String, List<String>> prefixHashTable = null;

        public List<List<String>> wordSquares(String[] words) {
            this.words = words;
            this.N = words[0].length();

            List<List<String>> results = new ArrayList<>();
            buildPrefixHashTable(words);

            for (String word : words) {
                LinkedList<String> wordSquares = new LinkedList<>();
                wordSquares.addLast(word);
                backtracking(1, wordSquares, results);
            }
            return results;
        }

        void backtracking(int step, LinkedList<String> wordSquares, List<List<String>> results) {
            if (step == N) {
                results.add((List<String>) wordSquares.clone());
                return;
            }

            StringBuilder prefix = new StringBuilder();
            for (String word : wordSquares) {
                prefix.append(word.charAt(step));
            }

            for (String candidate : getWordWithPrefix(prefix.toString())) {
                wordSquares.addLast(candidate);
                backtracking(step + 1, wordSquares, results);
                wordSquares.removeLast();
            }
        }

        void buildPrefixHashTable(String[] words) {
            prefixHashTable = new HashMap<>();
            for (String word : words) {
                for (int i = 1; i <= N; i++) {
                    String prefix = word.substring(0, i);
                    prefixHashTable.computeIfAbsent(prefix, $ -> new ArrayList<>()).add(word);
                }
            }
        }

        List<String> getWordWithPrefix(String prefix) {
            List<String> wordList = this.prefixHashTable.get(prefix);
            return (wordList != null ? wordList : new ArrayList<>());
        }
    }
}
