package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

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

    
}
