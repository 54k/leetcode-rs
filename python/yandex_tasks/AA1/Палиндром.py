class Solution:
    def isPalindrome(self, s: str) -> bool:
        left = 0
        right = len(s) - 1

        while left < right:  # Строгое не равно работает как с четной так и с нечетной строкой
            left_char = s[left]
            right_char = s[right]

            if not left_char.isalnum():  # .isalpha() для только букв
                left += 1  # Увеличиваем индекс пока не наткнемся на валидный символ
                continue

            if not right_char.isalnum():
                right -= 1
                continue
            # Когда оба символа валидные, чекаем на раввенство
            if left_char.lower() != right_char.lower():
                return False

            left += 1
            right -= 1

        return True


# Чуть более пижонски

def isPalindrome(self, s):
    l = 0
    r = len(s) - 1
    while l < r:
        while l < r and not s[l].isalnum():
            l += 1
        while l < r and not s[r].isalnum():
            r -= 1
        if s[l].lower() != s[r].lower():
            return False
        l += 1
        r -= 1
    return True
