
player_names = ['Jerryd Bayless', 'Devin Booker', 'Eric Bledsoe',
           'Carmelo Anthony', 'DeMar Derozan', 'Vince Carter']
player_numbers = [0, 1, 2, 7, 10, 15]
player_teams = ['PHI', 'PHX', 'PHX', 'NYK', 'TOR', 'MEM']

def check_jersey_number():
    name = input('What player\'s number do you want to know? ')
    i = 0
    length = len(player_names)
    while (i < length):
        if (player_names[i] == name):
            print(name + ' wears the number ' + str(player_numbers[i]))
            print('and he plays for ' + player_teams[i])
        i += 1
        
    # How could this be changed to look up a name/team, 
    # given a number as input?

def main():
    check_jersey_number()
    
main()