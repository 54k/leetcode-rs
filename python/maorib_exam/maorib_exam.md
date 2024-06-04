Для выполнения задания можно использовать следующие утилиты командной строки в ОС Linux:

1. **Проверка наличия файла:**
   ```bash
   [ -e filename ] && echo "Файл существует" || echo "Файл не существует"
   ```

2. **Проверка размера файла:**
   ```bash
   stat -c %s filename
   ```

3. **Проверка неизменности содержимого файла (хеш):**
   ```bash
   md5sum filename | cut -d ' ' -f 1
   ```

4. **Проверка прав файла:**
   ```bash
   stat -c %A filename
   ```

Эти команды можно использовать в скрипте для мониторинга файла. Вот пример такого скрипта на Bash:

```bash
#!/bin/bash

filename="path/to/your/file"
prev_size=""
prev_hash=""
prev_permissions=""

while true; do
  if [ ! -e "$filename" ]; then
    echo "$(date): Файл не существует"
  else
    # Проверка размера файла
    size=$(stat -c %s "$filename")
    if [ "$size" != "$prev_size" ]; then
      echo "$(date): Размер файла изменился: $size"
      prev_size="$size"
    fi

    # Проверка хеша файла
    hash=$(md5sum "$filename" | cut -d ' ' -f 1)
    if [ "$hash" != "$prev_hash" ]; then
      echo "$(date): Хеш файла изменился: $hash"
      prev_hash="$hash"
    fi

    # Проверка прав файла
    permissions=$(stat -c %A "$filename")
    if [ "$permissions" != "$prev_permissions" ]; then
      echo "$(date): Права файла изменились: $permissions"
      prev_permissions="$permissions"
    fi
  fi
  sleep 5
done
```

Для усложненного задания на Python можно использовать следующий скрипт:

```python
import os
import time
import hashlib
from datetime import datetime

filename = "path/to/your/file"
prev_size = None
prev_hash = None
prev_permissions = None

def get_file_size(filepath):
    return os.path.getsize(filepath)

def get_file_hash(filepath):
    hasher = hashlib.md5()
    with open(filepath, 'rb') as f:
        buf = f.read()
        hasher.update(buf)
    return hasher.hexdigest()

def get_file_permissions(filepath):
    return oct(os.stat(filepath).st_mode)[-3:]

while True:
    if not os.path.exists(filename):
        print(f"{datetime.now()}: Файл не существует")
    else:
        # Проверка размера файла
        size = get_file_size(filename)
        if size != prev_size:
            print(f"{datetime.now()}: Размер файла изменился: {size}")
            prev_size = size

        # Проверка хеша файла
        hash = get_file_hash(filename)
        if hash != prev_hash:
            print(f"{datetime.now()}: Хеш файла изменился: {hash}")
            prev_hash = hash

        # Проверка прав файла
        permissions = get_file_permissions(filename)
        if permissions != prev_permissions:
            print(f"{datetime.now()}: Права файла изменились: {permissions}")
            prev_permissions = permissions

    time.sleep(5)
```

Оба скрипта периодически проверяют состояние файла и выводят информацию о его изменениях.