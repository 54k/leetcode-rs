### **Полный разбор задачи "1931. Painting a Grid With Three Different Colors"**

**Задача:**  
Дана сетка `m x n`. Нужно раскрасить каждую клетку в один из 3 цветов (R, G, B) так, чтобы **никакие две соседние клетки** (по горизонтали или вертикали) не были одинакового цвета. Вернуть количество способов по модулю `10^9 + 7`.

---

## **1. Генерация всех валидных строк**
Для строки из `m` клеток генерируем все возможные раскраски, где соседние клетки разного цвета.

```python
def generate_rows():
    from itertools import product
    colors = ['R', 'G', 'B']
    rows = []
    for p in product(colors, repeat=m):  # Все возможные комбинации цветов
        valid = True
        for i in range(m - 1):
            if p[i] == p[i + 1]:  # Проверяем соседние клетки
                valid = False
                break
        if valid:
            rows.append(p)  # Добавляем только валидные раскраски
    return rows
```
**Пример для `m=2`:**  
Допустимые раскраски: `['RG', 'RB', 'GR', 'GB', 'BR', 'BG']` (6 вариантов).

---

## **2. Построение матрицы переходов**
Матрица `A` размера `K x K` (где `K` — число валидных строк).  
`A[i][j] = 1`, если строку `j` можно поставить **под** строку `i` (т.е. цвета в столбцах разные).

```python
K = len(rows)
A = [[0] * K for _ in range(K)]  # Инициализируем нулевую матрицу

for i in range(K):
    for j in range(K):
        valid = True
        for k in range(m):  # Проверяем все столбцы
            if rows[i][k] == rows[j][k]:  # Если цвета совпадают — переход невозможен
                valid = False
                break
        if valid:
            A[i][j] = 1
```
**Пример для `m=2`:**  
Если `rows[i] = "RG"`, то `rows[j]` может быть `"GB"`, `"BR"`, но не `"GR"` (т.к. `G` совпадает во втором столбце).

---

## **3. Быстрое возведение матрицы в степень**
Используем метод **бинарного возведения в степень**, чтобы вычислить `A^(n-1)` за `O(log n)` операций.

```python
def matrix_pow(mat, power):
    # Инициализируем единичную матрицу (аналог 1 для умножения)
    result = [[1 if i == j else 0 for j in range(K)] for i in range(K)]
    
    while power > 0:
        if power % 2 == 1:  # Если степень нечётная
            result = matrix_mult(result, mat)  # Умножаем результат на mat
        mat = matrix_mult(mat, mat)  # Возводим mat в квадрат
        power //= 2  # Уменьшаем степень вдвое
    return result

def matrix_mult(a, b):
    res = [[0] * K for _ in range(K)]
    for i in range(K):
        for k in range(K):
            if a[i][k]:  # Оптимизация: умножаем только ненулевые элементы
                for j in range(K):
                    res[i][j] = (res[i][j] + a[i][k] * b[k][j]) % MOD
    return res
```
**Почему единичная матрица?**  
Потому что `A^0 = E` (как `x^0 = 1` для чисел).

---

## **4. Подсчёт общего количества способов**
После вычисления `A^(n-1)` суммируем все элементы — это и есть ответ.

```python
mat = matrix_pow(A, n - 1)  # Возводим матрицу в степень (n-1)
total = 0
for i in range(K):
    for j in range(K):
        total = (total + mat[i][j]) % MOD  # Суммируем все переходы
return total
```
**Пояснение:**  
Каждый элемент `mat[i][j]` — количество способов перейти из строки `i` в строку `j` за `(n-1)` шагов.

---

## **5. Пример для `m=2, n=2`**
1. **Валидные строки:** `['RG', 'RB', 'GR', 'GB', 'BR', 'BG']` (6 шт.).
2. **Матрица переходов `A`:**  
   - Например, `A["RG"]["GB"] = 1` (можно),  
   - `A["RG"]["GR"] = 0` (нельзя, т.к. `G` совпадает во втором столбце).
3. **`A^(2-1) = A`:**  
   Сумма всех элементов = 24 (что верно: 6 стартовых строк × 4 варианта для второй строки).

---

## **6. Оптимизации**
- **Для `m=5`** валидных строк всего 48 (а не `3^5 = 243`), поэтому матрица 48×48 — небольшая.
- **Быстрое умножение** матриц за `O(K^3)`.
- **Бинарное возведение в степень** за `O(log n)`.

---

## **Итоговый код**
```python
MOD = 10**9 + 7

class Solution:
    def colorTheGrid(self, m: int, n: int) -> int:
        # 1. Генерация всех валидных строк
        def generate_rows():
            from itertools import product
            colors = ['R', 'G', 'B']
            rows = []
            for p in product(colors, repeat=m):
                valid = True
                for i in range(m - 1):
                    if p[i] == p[i + 1]:
                        valid = False
                        break
                if valid:
                    rows.append(p)
            return rows

        rows = generate_rows()
        K = len(rows)
        if K == 0:
            return 0

        # 2. Построение матрицы переходов
        A = [[0] * K for _ in range(K)]
        for i in range(K):
            for j in range(K):
                valid = True
                for k in range(m):
                    if rows[i][k] == rows[j][k]:
                        valid = False
                        break
                if valid:
                    A[i][j] = 1

        # 3. Быстрое возведение в степень
        def matrix_mult(a, b):
            res = [[0] * K for _ in range(K)]
            for i in range(K):
                for k in range(K):
                    if a[i][k]:
                        for j in range(K):
                            res[i][j] = (res[i][j] + a[i][k] * b[k][j]) % MOD
            return res

        def matrix_pow(mat, power):
            result = [[1 if i == j else 0 for j in range(K)] for i in range(K)]
            while power > 0:
                if power % 2 == 1:
                    result = matrix_mult(result, mat)
                mat = matrix_mult(mat, mat)
                power //= 2
            return result

        mat = matrix_pow(A, n - 1)
        total = 0
        for i in range(K):
            for j in range(K):
                total = (total + mat[i][j]) % MOD
        return total
```

---

## **Ключевые моменты**
1. **Графовая интерпретация:**  
   - Вершины — валидные раскраски строк.  
   - Рёбра — разрешённые переходы между строками.  
2. **Матрица переходов** — это матрица смежности графа.  
3. **Ответ** — сумма всех путей длины `(n-1)` в графе.  

Теперь ты можешь сохранить этот разбор и код как шпаргалку! 😎