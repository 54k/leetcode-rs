# Author: Boris Korotaev
import math

# task 1
a, b = 4, 5
hypotenuse = math.sqrt(a * a + b * b)

# task 2
def check_conditions(x: int, y: int, z: int) -> bool:
    return x == y or x > z or y <= z


# task 3
def sum_even_numbers(arr: list[int]):
    return sum([x for x in arr if not x % 2])


# task 4
def fibonacci(n: int) -> list[int]:
    assert n > 0
    ret = [0] * n
    ret[1] = 1
    if n <= 2:
        return ret[:i]
    i = 2
    while i < n:
        ret[i] = ret[i - 1] + ret[i - 2]
        i += 1
    return ret


# tast 5
def double_values(m: dict[int, int]) -> dict[int, int]:
    ret = {}
    for k, v in m.items():
        if type(k) is int:
            ret[k * 2] = v
        else:
            ret[k] = v
    return ret


# task 6
def count_strings(strs: list[str]) -> dict[str, int]:
    ret = {}
    for s in [x.lower() for x in strs]:
        ret[s] = ret.get(s, 0) + 1
    return ret


# task 7
class UserDatabase:
    def __init__(self) -> None:
        self.__users = {}

    def add_user(self, username: str, password: str) -> None:
        if username in self.__users:
            print("Пользователь уже существует")
            return
        self.__users[username] = password

    def authenticate(self, username: str, password: str):
        if self.__validate_user(username, password):
            print("Аутентификация успешна")
            return
        print("Ошибка аутентификации")

    def __validate_user(self, username: str, password: str) -> bool:
        return self.__users[username] == password if username in self.__users else False


# task 8
class Person:
    def __init__(self, id: int, name: str) -> None:
        self._id = id
        self._name = name

    def get_details(self) -> None:
        print(f"Имя: {self._name}, ID: {self._id}")


class Student(Person):
    def __init__(self, id: int, name: str, course: str) -> None:
        super().__init__(id, name)
        self._course = course

    def get_details(self) -> None:
        print(f"Имя: {self._name}, ID: {self._id}, course: {self._course}")


class Instructor(Person):
    def __init__(self, id: int, name: str, department: str) -> None:
        super().__init__(id, name)
        self._department = department

    def get_details(self) -> None:
        print(f"Имя: {self._name}, ID: {self._id}, departnment: {self._department}")


# task 9
class Shape:
    def get_area(self) -> float:
        pass


class Circle(Shape):
    PI = 3.14159

    def __init__(self, radius: float) -> None:
        assert radius > 0
        self.radius = radius

    def get_area(self) -> float:
        return Circle.PI * self.radius * self.radius


class Rectangle(Shape):
    def __init__(self, width: float, height: float) -> None:
        assert width > 0 and height > 0
        self.width = width
        self.height = height

    def get_area(self) -> float:
        return self.width * self.height


if __name__ == "__main__":
    # 1
    print(hypotenuse)
    # 2
    print(check_conditions(1, 2, 3))
    # 3
    print(sum_even_numbers([1, 2, 3, 4]))
    # 4
    print(fibonacci(10))
    # 5
    print(double_values({1: "1", 2: "2", 3: "3", "4": "4"}))
    # 6
    print(count_strings(["one", "TWO", "two"]))
    # 7
    udb = UserDatabase()
    udb.add_user("Alice", "123")
    udb.add_user("Alice", "123")

    udb.authenticate("Alice", "123")
    udb.authenticate("Bob", "123")
    udb.authenticate("Eva", "123")
    # 8
    student = Student(1, "Alice", "Cybersecurity")
    instructor = Instructor(1, "Bob", "42")
    student.get_details()
    instructor.get_details()
    # 9
    circle = Circle(2)
    rectangle = Rectangle(2, 2)
    print(circle.get_area())
    print(rectangle.get_area())
