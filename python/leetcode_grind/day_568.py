# https://leetcode.com/problems/longest-common-subsequence-between-sorted-arrays/description/
class Solution1:
    def longestCommonSubsequence(self, arrays: List[List[int]]) -> List[int]:
        longest_common_subseq = set(arrays[0])  
        for i in range(1, len(arrays)):
            longest_common_subseq = longest_common_subseq.intersection(
                set(arrays[i])
            ) 
        return sorted(longest_common_subseq)