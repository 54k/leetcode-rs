# https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/description
class Solution1:
    def countTriplets(self, arr: List[int]) -> int:
        count = 0
        for start in range(len(arr) - 1):
            xor_A = 0

            for mid in range(start + 1, len(arr)):
                xor_A ^= arr[mid-1]
                xor_B = 0

                for end in range(mid, len(arr)):
                    xor_B ^= arr[end]
                    if xor_A == xor_B:
                        count += 1
        return count