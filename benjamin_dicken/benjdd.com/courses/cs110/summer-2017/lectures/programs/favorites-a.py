questions = ['food', 'drink', 'car', 'city']
answers = ['Chinese', 'Coffee', 'Ford F150', 'Tucson']

def process_favorites():
    
    request = ''
    while request != 'EXIT':
        
        request = input('> ')
        # Handle the 'what is my favorite X?' command
        if request.startswith('what is my favorite'):
            sp = request.split(' ')
            question = sp[4]
            question = question[:len(question)-1]
            answer_index = -1
            for i in range(0,len(questions)):
                if questions[i] == question:
                    answer_index = i
                    break
            if answer_index != -1:
                print('Your favorite ' + questions[answer_index] + ' is ' + answers[answer_index])
            else:
                print('Not sure!')
        # Handle all unknown commands
        else:
            print('Huh?')
        
def main():
    process_favorites()

main()

