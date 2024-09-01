package leetcode_grind;

import java.util.*;

public class Day654 {
    // https://leetcode.com/problems/combinations/description/
    static class Solution1 {
        public List<List<Integer>> combine(int n, int k) {
            var res = new ArrayList<List<Integer>>();
            var rec = new Object() {
                void apply(List<Integer> curr, int start) {
                    if (curr.size() == k) {
                        res.add(new ArrayList<>(curr));
                        return;
                    }
                    for (int i = start; i <= n; i++) {
                        curr.add(i);
                        apply(curr, i + 1);
                        curr.remove(curr.size() - 1);
                    }
                }
            };
            rec.apply(new ArrayList<>(), 1);
            return res;
        }
    }

    static class Solution2 {
        public List<List<Integer>> combine(int n, int k) {
            var res = new ArrayList<List<Integer>>();
            var rec = new Object() {
                void apply(List<Integer> curr, int start) {
                    if (curr.size() == k) {
                        res.add(new ArrayList<>(curr));
                        return;
                    }

                    int need = k - curr.size();
                    int remain = n - start + 1;
                    int available = remain - need;

                    for (int i = start; i <= start + available; i++) {
                        curr.add(i);
                        apply(curr, i + 1);
                        curr.remove(curr.size() - 1);
                    }
                }
            };
            rec.apply(new ArrayList<>(), 1);
            return res;
        }
    }

    // https://leetcode.com/problems/permutations/description/
    static class Solution3 {
        public List<List<Integer>> permute(int[] nums) {
            var cur = new ArrayList<Integer>();
            var res = new ArrayList<List<Integer>>();
            var rec = new Object() {
                void apply(int seen) {
                    if (cur.size() == nums.length) {
                        res.add(new ArrayList<>(cur));
                        return;
                    }
                    for (int i = 0; i < nums.length; i++) {
                        if (((seen >> i) & 1) == 0) {
                            cur.add(nums[i]);
                            apply(seen | (1 << i));
                            cur.remove(cur.size() - 1);
                        }
                    }
                }
            };
            rec.apply(0);
            return res;
        }
    }

    // https://leetcode.com/problems/add-two-polynomials-represented-as-linked-lists/description/?envType=weekly-question&envId=2024-09-01
    static class PolyNode {
        int coefficient, power;
        PolyNode next = null;

        PolyNode() {
        }

        PolyNode(int x, int y) {
            this.coefficient = x;
            this.power = y;
        }

        PolyNode(int x, int y, PolyNode next) {
            this.coefficient = x;
            this.power = y;
            this.next = next;
        }
    }

    static class Solution4 {
        public PolyNode addPoly(PolyNode poly1, PolyNode poly2) {
            if (poly1 == null)
                return poly2;
            if (poly2 == null)
                return poly1;

            if (poly1.power == poly2.power) {
                if (poly1.coefficient + poly2.coefficient == 0) {
                    return addPoly(poly1.next, poly2.next);
                }
                return new PolyNode(poly1.coefficient + poly2.coefficient, poly1.power,
                        addPoly(poly1.next, poly2.next));
            } else if (poly1.power > poly2.power) {
                poly1.next = addPoly(poly1.next, poly2);
                return poly1;
            } else {
                poly2.next = addPoly(poly1, poly2.next);
                return poly2;
            }
        }
    }

    static class Solution5 {
        public PolyNode addPoly(PolyNode poly1, PolyNode poly2) {
            PolyNode sum = new PolyNode();
            PolyNode current = sum;
            Map<Integer, Integer> map = new TreeMap<>(Collections.reverseOrder());
            processNode(map, poly1);
            processNode(map, poly2);
            for (int key : map.keySet()) {
                current.next = new PolyNode(map.get(key), key);
                current = current.next;
            }
            return sum.next;
        }

        void processNode(Map<Integer, Integer> map, PolyNode node) {
            while (node != null) {
                if (map.containsKey(node.power)) {
                    int newCoefficient = node.coefficient + map.get(node.power);
                    if (newCoefficient == 0) {
                        map.remove(node.power);
                    } else {
                        map.put(node.power, newCoefficient);
                    }
                } else {
                    map.put(node.power, node.coefficient);
                }
                node = node.next;
            }
        }
    }

    static class Solution6 {
        public PolyNode addPoly(PolyNode poly1, PolyNode poly2) {
            var p1 = poly1;
            var p2 = poly2;
            var sum = new PolyNode();
            var current = sum;

            while (p1 != null && p2 != null) {
                if (p1.power == p2.power) {
                    if (p1.coefficient + p2.coefficient != 0) {
                        current.next = new PolyNode(
                                p1.coefficient + p2.coefficient,
                                p1.power);
                        current = current.next;
                    }
                    p1 = p1.next;
                    p2 = p2.next;
                } else if (p1.power > p2.power) {
                    current.next = p1;
                    p1 = p1.next;
                    current = current.next;
                } else {
                    current.next = p2;
                    p2 = p2.next;
                    current = current.next;
                }
            }

            if (p1 == null) {
                current.next = p2;
            } else {
                current.next = p1;
            }
            return sum.next;
        }
    }
}
