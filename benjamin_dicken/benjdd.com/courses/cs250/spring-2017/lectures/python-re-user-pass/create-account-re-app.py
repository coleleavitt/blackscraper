import sqlite3
import sys
import re

print('Welcome to the account creation portal.')
print('Enter a username:')
username = sys.stdin.readline().strip()

def check_regex_rule(input, regex_rule, message):
    pattern = re.compile(regex_rule)
    res = pattern.findall(input)
    if len(res) == 0 :
        print('ERROR: ' + message)
        sys.exit()

check_regex_rule(username, r'.{8,}', 'Length must be 8')
check_regex_rule(username, r'.*[A-Za-z].*', '')
check_regex_rule(username, r'.*[0-9].*', '')

print('Enter a password:')
password = sys.stdin.readline().strip()

check_regex_rule(password, r'.{10,}', 'length')
check_regex_rule(password, r'.*[0-9].*', 'digit')
check_regex_rule(password, r'.*[;:,.!?].*', 'special char')

user_pass = (username, password)

conn = sqlite3.connect('application')
conn.execute('INSERT INTO user VALUES (?, ?)', user_pass)
conn.commit()
conn.close()
