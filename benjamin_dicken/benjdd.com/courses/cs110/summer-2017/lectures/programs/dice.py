from random import *

def main():
    while True:
        r1 = randint(1,6)
        r2 = randint(1,6)
        print('The roll was: ' + str(r1) + ' + ' + str(r2))
        if r1 + r2 == 7:
            print('A sum of 7!')
            break

main()