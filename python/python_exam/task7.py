# Author: Boris Korotaev
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

if __name__ == "__main__":
    # 7
    udb = UserDatabase()
    udb.add_user("Alice", "123")
    udb.add_user("Alice", "123")

    udb.authenticate("Alice", "123")
    udb.authenticate("Bob", "123")
    udb.authenticate("Eva", "123")
