package leetcode_grind;

import java.util.*;

public class Day1035 {
    // https://leetcode.com/problems/design-spreadsheet/description/?envType=daily-question&envId=2025-09-19
    static class Spreadsheet {
        Map<String, Integer> map;

        public Spreadsheet(int rows) {
            map = new HashMap<>();
        }

        public void setCell(String cell, int value) {
            map.put(cell, value);
        }

        public void resetCell(String cell) {
            map.remove(cell);
        }

        public int getValue(String formula) {
            String[] f = formula.substring(1).split("\\+");
            return toInt(f[0]) + toInt(f[1]);
        }

        int toInt(String s) {
            try {
                return Integer.parseInt(s);
            } catch (Exception e) {
                return map.getOrDefault(s, 0);
            }
        }
    }

    /**
     * Your Spreadsheet object will be instantiated and called as such:
     * Spreadsheet obj = new Spreadsheet(rows);
     * obj.setCell(cell,value);
     * obj.resetCell(cell);
     * int param_3 = obj.getValue(formula);
     */
}