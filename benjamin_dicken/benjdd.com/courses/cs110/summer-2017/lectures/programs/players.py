def main():
    f = open('players.txt', 'r')
    lines = f.readlines()
    players = {}
    for line in lines:
        sp = line.split(' | ')
        name = sp[0]
        number = int(sp[1])
        players[name] = number
    
    while True:
        inp = input('Enter a player name: ')
        if inp == 'exit':
            break
        elif inp == 'show all players':
            for key, value in players.items():
                print(key + ' wears ' + str(value))
        else:
            name = inp
            if name in players:
                number = players[name]
                print(name + ' wears number ' + str(number))
            else:
                print('I dont know that player')
    
main()