# https://leetcode.com/problems/sort-the-people/description/?envType=daily-question&envId=2024-07-22
class Solution:
    def sortPeople(self, names: List[str], heights: List[int]) -> List[str]:
        self._merge_sort(names, heights, 0, len(heights) - 1)
        return names
    
    def _merge_sort(self, names: List[str], heights: List[int], start: int, end: int):
        if start < end:
            mid = start + (end - start) // 2
            self._merge_sort(names, heights, start, mid)
            self._merge_sort(names, heights, mid + 1, end)
            self._merge(names, heights, start, mid, end)
    
    def _merge(self, names: List[str], heights: List[int], start: int, mid: int, end: int):
        left_size = mid - start + 1
        right_size = end - mid

        left_heights = heights[start : start + left_size]
        right_heights = heights[mid + 1 : mid + 1 + right_size]

        left_names = names[start : start + left_size]
        right_names = names[mid + 1 : mid + 1 + right_size]

        left_index, right_index = 0, 0
        merge_index = start

        while left_index < left_size and right_index < right_size:
            if left_heights[left_index] >= right_heights[right_index]:
                heights[merge_index] = left_heights[left_index]
                names[merge_index] = left_names[left_index]
                left_index += 1
            else:
                heights[merge_index] = right_heights[right_index]
                names[merge_index] = right_names[right_index]
                right_index += 1
            merge_index += 1

        while left_index < left_size:
            heights[merge_index] = left_heights[left_index]
            names[merge_index] = left_names[left_index]
            left_index += 1
            merge_index += 1
        
        while right_index < right_size:
            heights[merge_index] = right_heights[right_index]
            names[merge_index] = right_names[right_index]
            right_index += 1
            merge_index += 1