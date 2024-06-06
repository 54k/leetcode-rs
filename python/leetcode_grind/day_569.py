# https://leetcode.com/problems/hand-of-straights/description
class Solution1:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:
        hand_size = len(hand)
        if hand_size % groupSize != 0:
            return False

        card_count = Counter(hand)
        min_heap = list(card_count.keys())
        heapq.heapify(min_heap)

        while min_heap:
            current_card = min_heap[0]
            for i in range(groupSize):
                if card_count[current_card + i] == 0:
                    return False
                card_count[current_card + i] -= 1
                if card_count[current_card + i] == 0:
                    if current_card + i != heapq.heappop(min_heap):
                        return False
        return True

# https://leetcode.com/problems/search-in-rotated-sorted-array/description/
class Solution2:
    def search(self, nums: List[int], target: int) -> int:
        n = len(nums)
        left, right = 0, n-1
        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] >= nums[left]:
                if target >= nums[left] and target < nums[mid]:
                    right = mid - 1
                else:
                    left = mid + 1
            else:
                if target <= nums[right] and target > nums[mid]:
                    left = mid + 1
                else:
                    right = mid - 1
        return -1