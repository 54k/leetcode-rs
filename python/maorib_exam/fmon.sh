#!/bin/bash

# Функция для вывода помощи и описания кода
function help {
  echo "Usage: $0 <filename>"
  echo
  echo "Этот скрипт мониторит изменения файла, включая его размер, хеш и права доступа."
  echo
  echo "Аргументы:"
  echo "  filename      Имя файла для мониторинга"
  echo
  echo "Описание:"
  echo "  Скрипт проверяет наличие файла, его размер, хеш (MD5) и права доступа каждые 5 секунд."
  echo "  При обнаружении изменений, выводится сообщение с новой информацией и текущей датой и временем."
  echo
  echo "Пример использования:"
  echo "  $0 /path/to/your/file"
}

# Проверка на наличие аргумента
if [ "$#" -ne 1 ]; then
  echo "Ошибка: Не указано имя файла."
  help
  exit 1
fi

filename=$1
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
