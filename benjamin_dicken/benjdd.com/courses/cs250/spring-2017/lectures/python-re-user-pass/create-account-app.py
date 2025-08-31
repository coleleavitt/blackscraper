import sqlite3
import sys

print('Welcome to the account creation portal.')
print('Enter a username:')
username = sys.stdin.readline().strip()

print('Enter a password:')
password = sys.stdin.readline().strip()

user_pass = (username, password)

conn = sqlite3.connect('application')
conn.execute('INSERT INTO user VALUES (?, ?)', user_pass)
conn.commit()
conn.close()

