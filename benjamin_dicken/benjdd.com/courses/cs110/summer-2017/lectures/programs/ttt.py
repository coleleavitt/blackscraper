b = [[' ', ' ', ' '],
     [' ', ' ', ' '],
     [' ', ' ', ' ']]

def check_won():
    for i in range(0, len(b)):
        if b[0][i] == b[1][i] and b[1][i] == b[2][i] and b[0][i] != ' ':
            print(b[1][i] + ' Won!')
            return True
        if b[i][0] == b[i][1] and b[i][1] == b[i][2] and b[i][0] != ' ':
            print(b[i][1] + ' Won!')
            return True
    if b[0][0] == b[1][1] and b[1][1] == b[2][2] and b[1][1] != ' ':
        print(b[1][1] + ' Won!')
        return True
    elif b[2][0] == b[1][1] and b[1][1] == b[0][2] and b[1][1] != ' ':
        print(b[1][1] + ' Won!')
        return True 
    return False

def print_board():
    print('-------------')
    for i in b:
        print('| ', end='')
        for j in i:
            print(j, end=' | ')
        print()
        print('-------------')
    
def main():
    print('Welcome to TTT')
    
    while True:
        char = input('Which character? ')
        row = int(input('Enter row to place: '))
        col = int(input('Enter col to place: '))
        if (char != 'X' and char != 'O'):
            print('Invalid character')
            break
        if row < 0 or row > 2:
            print('Invalid row')
            break  
        if col < 0 or col > 2:
            print('Invalid row')
            break
        b[row][col] = char
        print_board()
        result = check_won()
        if (result == True):
            print('Game over')
            break

main()
