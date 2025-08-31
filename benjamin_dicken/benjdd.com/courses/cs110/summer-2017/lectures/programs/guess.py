from random import *

def main():
    num = randint(1, 100)
    print(num)
    guess = -1
    while True:
        guess = int(input('Guess a number: '))
        if guess == num:
            print('Correct!')
            break
        else:
            print('Wrong!')

main()