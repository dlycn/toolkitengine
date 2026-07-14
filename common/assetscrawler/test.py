import sqlite3

# 连接到数据库
connection = sqlite3.connect("test.db")
cursor = connection.cursor()

# 读取二进制文件
with open("test.png", "rb") as file:
    blob_data = file.read()

# 插入 BLOB 数据
cursor.execute("INSERT INTO Person (name) VALUES (?)", ("test_name",))


# 提交事务
connection.commit()

# 关闭连接
connection.close()

import requests

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"
}


