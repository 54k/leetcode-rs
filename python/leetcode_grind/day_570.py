# https://leetcode.com/problems/replace-words/description
class Solution1:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        res = [] 
        dic = set(dictionary)
        for word in sentence.split():
            for i in range(1, len(word) + 1):
                if word[:i] in dic or i == len(word):
                    res.append(word[:i])
                    break
        return " ".join(res)