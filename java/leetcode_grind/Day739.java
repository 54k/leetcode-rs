package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Day739 {
    // https://leetcode.com/problems/sliding-puzzle/description/?envType=daily-question&envId=2024-11-25
    static class Solution1 {
        final int[][] directions = {
                { 1, 3 },
                { 0, 2, 4 },
                { 1, 5 },
                { 0, 4 },
                { 3, 5, 1 },
                { 4, 2 }
        };

        public int slidingPuzzle(int[][] board) {
            StringBuilder startState = new StringBuilder();
            for (int i = 0; i < 2; i++) {
                for (int j = 0; j < 3; j++) {
                    startState.append(board[i][j]);
                }
            }

            Map<String, Integer> visited = new HashMap<>();
            dfs(startState.toString(), visited, startState.indexOf("0"), 0);
            return visited.getOrDefault("123450", -1);
        }

        void dfs(String state, Map<String, Integer> visited, int zeroPos, int moves) {
            if (visited.containsKey(state) && visited.get(state) <= moves) {
                return;
            }

            visited.put(state, moves);

            for (int nextPos : directions[zeroPos]) {
                String newState = swap(state, zeroPos, nextPos);
                dfs(newState, visited, nextPos, moves + 1);
            }
        }

        String swap(String str, int i, int j) {
            StringBuilder sb = new StringBuilder(str);
            sb.setCharAt(i, str.charAt(j));
            sb.setCharAt(j, str.charAt(i));
            return sb.toString();
        }
    }

    static class Solution2 {
        final int[][] directions = {
                { 1, 3 },
                { 0, 2, 4 },
                { 1, 5 },
                { 0, 4 },
                { 3, 5, 1 },
                { 4, 2 }
        };

        public int slidingPuzzle(int[][] board) {
            String target = "123450";
            StringBuilder startState = new StringBuilder();
            for (int i = 0; i < board.length; i++) {
                for (int j = 0; j < board[0].length; j++) {
                    startState.append(board[i][j]);
                }
            }

            Set<String> visited = new HashSet<>();
            Queue<String> queue = new LinkedList<>();
            queue.add(startState.toString());
            visited.add(startState.toString());

            int moves = 0;

            while (!queue.isEmpty()) {
                int size = queue.size();
                while (size-- > 0) {
                    String currentState = queue.poll();

                    if (currentState.equals(target)) {
                        return moves;
                    }

                    int zeroPos = currentState.indexOf('0');

                    for (int newPos : directions[zeroPos]) {
                        String nextState = swap(currentState, zeroPos, newPos);

                        if (visited.contains(nextState))
                            continue;

                        visited.add(nextState);
                        queue.add(nextState);
                    }
                }
                moves++;
            }

            return -1;
        }

        String swap(String str, int i, int j) {
            StringBuilder sb = new StringBuilder(str);
            sb.setCharAt(i, str.charAt(j));
            sb.setCharAt(j, str.charAt(i));
            return sb.toString();
        }
    }
}
