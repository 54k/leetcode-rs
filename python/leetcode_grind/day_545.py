# https://leetcode.com/problems/find-products-of-elements-of-big-array/description/
class Solution:
    def findProductsOfElements(self, queries: List[List[int]]) -> List[int]:
        def count(x):
            if x == 0:
                return 0
            b = x.bit_length() - 1
            v = 1 << b
            res = b * (v >> 1)
            return res + count(x - v) + x - v
        
        def mul(x):
            if x == 0:
                return 0
            b = x.bit_length() - 1
            v = 1 << b
            res = (b - 1) * b * v >> 2
            return res + mul(x - v) + b * (x - v)
        
        def query(k):
            if k == 0:
                return 0
            k += 1
            x = bisect_left(range(1, 10 ** 15), k, key=count)
            res = mul(x)
            k -= count(x)
            for _ in range(k):
                b = x & -x
                res += b.bit_length() - 1
                x -= b
            return res
            
        return [pow(2, query(j) - query(i - 1), mod)  for i, j, mod in queries]