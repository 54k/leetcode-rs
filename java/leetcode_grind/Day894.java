package leetcode_grind;

import java.util.Arrays;
import java.util.TreeMap;

public class Day894 {
    // https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/description/?envType=daily-question&envId=2025-05-01
    static class Solution1 {
        public int maxTaskAssign(int[] tasks, int[] workers, int pills, int strength) {
            Arrays.sort(tasks);
            Arrays.sort(workers);
            int n = tasks.length, m = workers.length;
            int left = 1, right = Math.min(m, n), ans = 0;
            while (left <= right) {
                int mid = (left + right) / 2;
                if (check(tasks, workers, pills, strength, mid)) {
                    ans = mid;
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            return ans;
        }

        boolean check(int[] tasks, int[] workers, int pills, int strength, int mid) {
            int p = pills;
            TreeMap<Integer, Integer> ws = new TreeMap<>();
            for (int i = workers.length - mid; i < workers.length; i++) {
                ws.put(workers[i], ws.getOrDefault(workers[i], 0) + 1);
            }

            for (int i = mid - 1; i >= 0; --i) {
                Integer key = ws.lastKey();
                if (key >= tasks[i]) {
                    ws.put(key, ws.get(key) - 1);
                    if (ws.get(key) == 0) {
                        ws.remove(key);
                    }
                } else {
                    if (p == 0) {
                        return false;
                    }
                    key = ws.ceilingKey(tasks[i] - strength);
                    if (key == null) {
                        return false;
                    }
                    ws.put(key, ws.get(key) - 1);
                    if (ws.get(key) == 0) {
                        ws.remove(key);
                    }
                    --p;
                }
            }
            return true;
        }
    }

    // https://leetcode.com/problems/design-snake-game/description/?envType=weekly-question&envId=2025-05-01
    // static class SnakeGame {
    //     Map<Pair<Integer, Integer>, Boolean> snakeMap;
    //     Deque<Pair<Integer, Integer>> snake;
    //     int[][] food;
    //     int foodIndex;
    //     int width;
    //     int height;

    //     public SnakeGame(int width, int height, int[][] food) {
    //         this.width = width;
    //         this.height = height;
    //         this.food = food;
    //         this.snakeMap = new HashMap<Pair<Integer, Integer>, Boolean>();
    //         this.snakeMap.put(new Pair<Integer, Integer>(0, 0), true);
    //         this.snake = new LinkedList<Pair<Integer, Integer>>();
    //         this.snake.offerLast(new Pair<Integer, Integer>(0, 0));
    //     }

    //     public int move(String direction) {
    //         Pair<Integer, Integer> snakeCell = this.snake.peekFirst();
    //         int newHeadRow = snakeCell.getKey();
    //         int newHeadColumn = snakeCell.getValue();

    //         switch (direction) {
    //             case "U":
    //                 newHeadRow--;
    //                 break;
    //             case "D":
    //                 newHeadRow++;
    //                 break;
    //             case "L":
    //                 newHeadColumn--;
    //                 break;
    //             case "R":
    //                 newHeadColumn++;
    //                 break;
    //         }

    //         Pair<Integer, Integer> newHead = new Pair<Integer, Integer>(newHeadRow, newHeadColumn);
    //         Pair<Integer, Integer> currentTail = this.snake.peekLast();

    //         boolean crossesBoundary1 = newHeadRow < 0 || newHeadRow >= this.height;
    //         boolean crossesBoundary2 = newHeadColumn < 0 || newHeadColumn >= this.width;

    //         boolean bitesItself = this.snakeMap.containsKey(newHead)
    //                 && !(newHead.getKey() == currentTail.getKey() && newHead.getValue() == currentTail.getValue());

    //         if (crossesBoundary1 || crossesBoundary2 || bitesItself) {
    //             return -1;
    //         }

    //         if ((this.foodIndex < this.food.length) && (this.food[this.foodIndex][0] == newHeadRow) &&
    //                 (this.food[this.foodIndex][1] == newHeadColumn)) {
    //             this.foodIndex++;
    //         } else {
    //             this.snake.pollLast();
    //             this.snakeMap.remove(currentTail);
    //         }

    //         this.snake.addFirst(newHead);
    //         this.snakeMap.put(newHead, true);
    //         return this.snake.size() - 1;
    //     }
    // }

}