# Author: Boris Korotaev
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
        print(f"Студент: {self._name}, ID: {self._id}, Курс: {self._course}")


class Instructor(Person):
    def __init__(self, id: int, name: str, department: str) -> None:
        super().__init__(id, name)
        self._department = department

    def get_details(self) -> None:
        print(f"Преподаватель: {self._name}, ID: {self._id}, Кафедра: {self._department}")

if __name__ == "__main__":
    # 8
    person = Person(1, "John")
    student = Student(1, "Alice", "Cybersecurity")
    instructor = Instructor(2, "Bob", "42")
    person.get_details()
    student.get_details()
    instructor.get_details()
