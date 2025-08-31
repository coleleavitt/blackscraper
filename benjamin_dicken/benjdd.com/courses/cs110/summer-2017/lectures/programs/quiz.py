questions = ['What color is my backpack?',
             'What is my office number?',
             'What time does cs 110 start at on Tu/Th/Fr?']
answers = ['green',
           '826',
           '8:00am']

def do_quiz():
    print('Welcome to the quiz!')
    print('---')
    index = 0
    correct = 0
    for q in questions:
        answer = input(q + ' ')
        if answer == answers[index]:
            print('    Correct!')
            correct += 1
        else:
            print('    Wrong, the answer is: ' + answers[index])
        index += 1
    
    num_questions = len(questions)
    grade = (correct / num_questions) * 100
    grade = round(grade, 1)
    print('---')
    print('Your final grade is: ' + str(grade))
    
def main():
    do_quiz()

main()