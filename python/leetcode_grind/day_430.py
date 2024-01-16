# https://leetcode.com/problems/insert-delete-getrandom-o1/
from random import choice

class RandomizedSet:

    def __init__(self):
        self.idx = {}
        self.val = []
        

    def insert(self, val: int) -> bool:
        if val in self.idx:
            return False
        self.val.append(val)
        self.idx[val] = len(self.val)-1
        return True

    def remove(self, val: int) -> bool:
        if not (val in self.idx):
            return False
        idx = self.idx[val] 
        new = self.val[len(self.val)-1]
        self.val[idx] = new
        self.idx[new] = idx
        self.val.pop(len(self.val) - 1)
        del self.idx[val]
        return True

    def getRandom(self) -> int:
        return choice(self.val)


# Your RandomizedSet object will be instantiated and called as such:
# obj = RandomizedSet()
# param_1 = obj.insert(val)
# param_2 = obj.remove(val)
# param_3 = obj.getRandom()