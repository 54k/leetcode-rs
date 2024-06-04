#!/usr/bin/python3

import sys
import os
import time
import hashlib
from datetime import datetime

filename = sys.argv[1]
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
