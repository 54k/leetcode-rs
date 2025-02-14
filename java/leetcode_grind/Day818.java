package leetcode_grind;

import java.util.*;

public class Day818 {
    // https://leetcode.com/problems/product-of-the-last-k-numbers/description/?envType=daily-question&envId=2025-02-14
    static class ProductOfNumbers {
        List<Integer> prefixProduct = new ArrayList<>();
        int sz = 0;

        public ProductOfNumbers() {
            prefixProduct.add(1);
        }

        public void add(int num) {
            if (num == 0) {
                prefixProduct.clear();
                prefixProduct.add(1);
                sz = 0;
            } else {
                prefixProduct.add(prefixProduct.get(sz) * num);
                sz++;
            }
        }

        public int getProduct(int k) {
            if (k > sz) {
                return 0;
            }
            return prefixProduct.get(sz) / prefixProduct.get(sz - k);
        }
    }

    /**
     * Your ProductOfNumbers object will be instantiated and called as such:
     * ProductOfNumbers obj = new ProductOfNumbers();
     * obj.add(num);
     * int param_2 = obj.getProduct(k);
     */

    // https://leetcode.com/problems/shopping-offers/description/
    static class Solution2 {
        public int shoppingOffers(List<Integer> price, List<List<Integer>> special, List<Integer> needs) {
            Map<List<Integer>, Integer> map = new HashMap<>();
            return shopping(price, special, needs, map);
        }

        int shopping(List<Integer> price, List<List<Integer>> special, List<Integer> needs,
                Map<List<Integer>, Integer> map) {
            if (map.containsKey(needs)) {
                return map.get(needs);
            }

            int j = 0, res = dot(needs, price);
            for (List<Integer> s : special) {
                List<Integer> clone = new ArrayList<>(needs);
                for (j = 0; j < needs.size(); j++) {
                    int diff = clone.get(j) - s.get(j);
                    if (diff < 0) {
                        break;
                    }
                    clone.set(j, diff);
                }
                if (j == needs.size()) {
                    res = Math.min(res, s.get(j) + shopping(price, special, clone, map));
                }
            }
            map.put(needs, res);
            return res;
        }

        int dot(List<Integer> a, List<Integer> b) {
            int sum = 0;
            for (int i = 0; i < a.size(); i++) {
                sum += a.get(i) * b.get(i);
            }
            return sum;
        }
    }
}