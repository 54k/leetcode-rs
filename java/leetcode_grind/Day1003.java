package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day1003 {
    // https://leetcode.com/problems/24-game/description/?envType=daily-question&envId=2025-08-18
    static class Solution1 {

        List<Double> generatePossibleResults(double a, double b) {
            List<Double> res = new ArrayList<>();
            res.add(a + b);
            res.add(a - b);
            res.add(b - a);
            res.add(a * b);

            if (b != 0) {
                res.add(a / b);
            }

            if (a != 0) {
                res.add(b / a);
            }

            return res;
        }

        boolean checkIfResultReached(List<Double> list) {
            if (list.size() == 1) {
                return Math.abs(list.get(0) - 24) <= 0.1;
            }

            for (int i = 0; i < list.size(); i++) {
                for (int j = i + 1; j < list.size(); j++) {
                    List<Double> newList = new ArrayList<>();
                    for (int k = 0; k < list.size(); k++) {
                        if (k != j && k != i) {
                            newList.add(list.get(k));
                        }
                    }

                    for (double res : generatePossibleResults(list.get(i), list.get(j))) {
                        newList.add(res);

                        if (checkIfResultReached(newList)) {
                            return true;
                        }

                        newList.remove(newList.size() - 1);
                    }
                }
            }

            return false;
        }

        public boolean judgePoint24(int[] cards) {
            List<Double> list = new ArrayList<>();
            for (int card : cards) {
                list.add((double) card);
            }
            return checkIfResultReached(list);
        }
    }

}
