# https://www.geeksforgeeks.org/problems/insertion-sort-for-singly-linked-list/1
# Node Class
class Node:
    def __init__(self, data):   # data -> value stored in node
        self.data = data
        self.next = None

class Solution:
    def insertionSort(self, head):
        #code here
        #code here
        curr = head
        dummy = Node(-1000000007)
        prev = dummy

        while curr is not None:
            next = curr.next
            curr.next = None

            while prev is not None:
                if prev.next is None or prev.next.data >= curr.data:
                    curr.next = prev.next
                    prev.next = curr
                    break
                prev = prev.next

            curr = next
            prev = dummy

        return dummy.next