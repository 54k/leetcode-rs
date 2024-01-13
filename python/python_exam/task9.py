# Author: Boris Korotaev
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
    # 9
    circle = Circle(2)
    rectangle = Rectangle(2, 2)
    print(circle.get_area())
    print(rectangle.get_area())
