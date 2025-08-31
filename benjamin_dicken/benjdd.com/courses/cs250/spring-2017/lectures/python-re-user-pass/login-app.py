import sqlite3
import sys

print('Welcome to the application! Login Now.')
print('Enter your username:')
username = sys.stdin.readline().strip()
print('Enter your password:')
password = sys.stdin.readline().strip()

user_pass = (username, password)

conn = sqlite3.connect('application')
result = conn.execute('''SELECT * FROM user
                         WHERE username = ? 
                         AND password = ?''',
                         user_pass)
first_result = result.fetchone()
if first_result is not None:
    print('You have logged in successfully!')
    print('Unfortunately, this application is rather boring....')
    print('Bye!')
else:
    print('Wrong username/password! Go Away!')

