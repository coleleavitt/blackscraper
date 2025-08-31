#
# Author: Benjamin Dicken
# Description: A very simple quiz-giving program
#

def main():

    TOTAL = 3
    correct = 0

    question_1 = input('Enter question 1: ')
    solution_1 = input('  What\'s the solution? ')
    question_2 = input('Enter question 3: ')
    solution_2 = input('  What\'s the solution? ')
    question_3 = input('Enter question 3: ')
    solution_3 = input('  What\'s the solution? ')

    print('----------')
    print('Let the quiz begin!')
    print('----------')

    answer_1 = input(question_1 + " ")
    if (answer_1 == solution_1):
        print('Correct!')
        correct+=1
    else:
        print('Wrong')
    answer_2 = input(question_2 + " ")
    if (answer_2 == solution_2):
        print('Correct!')
        correct+=1
    else:
        print('Wrong')
    answer_3 = input(question_3 + " ")
    if (answer_3 == solution_3):
        print('Correct!')
        correct+=1
    else:
        print('Wrong')

    print('----------')
    print('You scored: ' + str(int((correct/TOTAL) * 100)) + "%")
    print('----------')
    
main()

