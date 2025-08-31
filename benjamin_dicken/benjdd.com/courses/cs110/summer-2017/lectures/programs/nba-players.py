
# This function prints out information about an NBA player.
# The first and only parameter of this function is the name of the player.
# It is expected to be a string.
def print_player_info(name):
    if (name == 'Lebron James'):
        print(name + ' Plays for CLE and wears #23')
    elif (name == 'Kevin Durant'):
        print(name + ' Plays for GSW and wears #35')
    elif (name == 'Tony Parker'):
        number = int(input('I know multiple players named' + name + '. Which number does he wear? '))
        if (number == 18):
            print(name + ' Played for TOR and wore #18')
        elif (number == 9):
            print(name + ' Plays for SAS and wears #9')
        else:
            print('I don\'t know about that player!')
    elif (name == 'Russel Westbrook'):
        print(name + ' Plays for OKC and wears #0')
    else:
        print('I don\'t know about that player!')

def main():
    name = ''
    while name != 'EXIT':
        name = input('What NBA player do you want info about? ')
        print_player_info(name)
    print('Goodbye!')
    
main()
