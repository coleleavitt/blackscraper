###
### Author: Benjamin Dicken
### Description: A quiz program
###

questions = []
solutions = []

def main():
    limit = int(input('How many questions? '))
    counter = 0
    while counter < limit:
        q = input('Enter question ' + str(counter+!) + ': ')
        s = input('  What\'s the solution? ')
        questions.append(q)
        solutions.append(s)
        counter += 1
        
    correct = 0
    index = 0
    while index < len(questions):
        a = input(questions[index] + " ")
        if (a == solutions[index]):
            print('Correct!')
            correct+=1
        else:
            print('Wrong')
        index += 1
    
    print('Quiz over!')
    print('Your score is ' + str(int((correct / limit) * 100)) + '%')

main()
