
def main():
    primary = int(input('How many numbers do you want me to process? (or say 0 to exit) '))
    while (primary != 0):
        index = 0
        num_sum = 0
        num_prod = 1
        num_max = 0
        while (index < primary):
            number = int(input('number: '))
            num_sum = num_sum + number
            num_prod = num_prod * number
            if (number > num_max):
                num_max = number
            index = index + 1
        print('-------')
        print('SUM  = ' + str(num_sum))
        print('PROD = ' + str(num_prod))
        print('MAX = ' + str(num_max))
        primary = int(input('How many numbers do you want me to process? (or say 0 to exit) '))
    
main()