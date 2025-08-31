actions = []
players = []
teams = []
times = []

def main():
    
    while True:
        command = input('Enter command: ')
        if command == 'event':
            event = input('enter ACTION.PLAYER.TEAM.TIME: ')
            es = event.split('.')
            actions.append(es[0])
            players.append(es[1])
            teams.append(es[2])
            times.append(es[3])
        elif command == 'log':
            for i in range(len(players)):
                print('At ' + times[i] + ': player ' + players[i] + ' on ' + teams[i] + ' did ' + actions[i])
        else:
            print('command not recognized')
    
main()