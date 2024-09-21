package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day674 {
    // https://leetcode.com/problems/lexicographical-numbers/description/?envType=daily-question&envId=2024-09-21
    static class Solution1 {
        public List<Integer> lexicalOrder(int n) {
            var res = new ArrayList<Integer>();
            var dfs = new Object() {
                void apply(int num) {
                    if (num > n)
                        return;
                    res.add(num);
                    for (int i = 0; i < 10; i++) {
                        apply(num * 10 + i);
                    }
                }
            };
            for (int i = 1; i < 10; i++) {
                dfs.apply(i);
            }
            return res;
        }
    }
}
