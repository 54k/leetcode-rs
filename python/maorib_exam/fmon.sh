#!/bin/bash

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
