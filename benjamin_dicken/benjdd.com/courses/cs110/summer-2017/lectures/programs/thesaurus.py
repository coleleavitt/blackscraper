thes = {}

def add_word(add_str):
    sp = add_str.split(' : ')
    word = sp[0]
    sim = sp[1]
    sim_list = sim.split(' ')
    thes[word] = sim_list    
    
def main():
    f = open('thesaurus.txt', 'r')
    lines = f.readlines()
    for line in lines:
        add_word(line)
    
    while True:
        text = input('> ')
        if text == 'exit':
            print('Bye!')
            break
        elif text.startswith('ADD'):
            line = text[4:]
            add_word(line)
            print('Word added!')
        else:
            if text in thes:
                print('Words similar to ' + text + ' are:')
                for word in thes[text]:
                    print('  ' + word)
            else:
                print('I dont know that word.')

main()