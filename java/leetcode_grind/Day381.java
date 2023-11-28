package leetcode_grind;

public class Day381 {
    // https://leetcode.com/problems/plus-one/
    static class Solution1 {
        public int[] plusOne(int[] digits) {
            for (int i = digits.length - 1; i >= 0; i--) {
                if (digits[i] == 9) {
                    digits[i] = 0;
                } else {
                    digits[i] += 1;
                    return digits;
                }
            }

            var ans = new int[digits.length + 1];
            ans[0] = 1;
            return ans;
        }
    }

    // https://leetcode.com/problems/plus-one-linked-list/
    public static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    static class Solution2 {
        public ListNode plusOne1(ListNode head) {
            var reverse = new Object() {
                ListNode apply(ListNode head) {
                    ListNode prev = null;
                    while (head != null) {
                        var next = head.next;
                        head.next = prev;
                        prev = head;
                        head = next;
                    }
                    return prev;
                }
            };

            var rev = reverse.apply(head);
            var carry = 1;
            var ansHead = new ListNode(-1);
            var ansTail = ansHead;

            while (rev != null) {
                var cur = rev.val;
                rev.val = (cur + carry) % 10;
                carry = (cur + carry) / 10;
                ansTail.next = rev;
                ansTail = ansTail.next;
                rev = rev.next;
            }
            if (carry > 0) {
                ansTail.next = new ListNode(carry);
            }
            return reverse.apply(ansHead.next);
        }

        public ListNode plusOne2(ListNode head) {
            ListNode sentinel = new ListNode(0);
            sentinel.next = head;

            ListNode nonNine = sentinel;
            while (head != null) {
                if (head.val != 9) {
                    nonNine = head;
                }
                head = head.next;
            }

            nonNine.val++;
            while (nonNine.next != null) {
                nonNine.next.val = 0;
                nonNine = nonNine.next;
            }

            return sentinel.val != 0 ? sentinel : sentinel.next;
        }
    }

    // https://leetcode.com/problems/knight-dialer/description/
    static class Solution3 {
        public int knightDialer1(int n) {
            if (n == 1) {
                return 10;
            }

            var mod = 1_000_000_007;

            var a = 4;
            var b = 2;
            var c = 2;
            var d = 1;

            while (n-- > 1) {
                var tmpa = a;
                var tmpb = b;
                var tmpc = c;
                var tmpd = d;

                a = ((2 * tmpb) % mod + (2 * tmpc) % mod) % mod;
                b = tmpa;
                c = (tmpa + (2 * tmpd) % mod) % mod;
                d = tmpc;
            }

            int ans = (a + b) % mod;
            ans = (ans + c) % mod;
            return (ans + d) % mod;
        }

        public int knightDialer2(int n) {
            if (n == 1) {
                return 10;
            }

            long mod = 1_000_000_007;

            long[][] A = new long[][] {
                    { 0, 0, 0, 0, 1, 0, 1, 0, 0, 0 },
                    { 0, 0, 0, 0, 0, 0, 1, 0, 1, 0 },
                    { 0, 0, 0, 0, 0, 0, 0, 1, 0, 1 },
                    { 0, 0, 0, 0, 1, 0, 0, 0, 1, 0 },
                    { 1, 0, 0, 1, 0, 0, 0, 0, 0, 1 },
                    { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
                    { 1, 1, 0, 0, 0, 0, 0, 1, 0, 0 },
                    { 0, 0, 1, 0, 0, 0, 1, 0, 0, 0 },
                    { 0, 1, 0, 1, 0, 0, 0, 0, 0, 0 },
                    { 0, 0, 1, 0, 1, 0, 0, 0, 0, 0 }
            };

            long[][] v = new long[][] {
                    { 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 }
            };

            var multiply = new Object() {
                long[][] apply(long[][] a, long[][] b) {
                    long[][] result = new long[a.length][b[0].length];

                    for (int i = 0; i < a.length; i++) {
                        for (int j = 0; j < b[0].length; j++) {
                            for (int k = 0; k < b.length; k++) {
                                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % mod;
                            }
                        }
                    }
                    return result;
                }
            };

            n--;
            while (n > 0) {
                if ((n & 1) != 0) {
                    v = multiply.apply(v, A);
                }

                A = multiply.apply(A, A);
                n >>= 1;
            }

            long ans = 0;
            for (long num : v[0]) {
                ans = (ans + num) % mod;
            }

            return (int) ans;
        }2
    }
}
