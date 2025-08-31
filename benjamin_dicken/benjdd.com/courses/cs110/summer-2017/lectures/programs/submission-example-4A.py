def every_other_string(strings):
    result = ''
    i = 0
    while i < len(strings):
        result = result + strings[i] + ' '
        i += 2
    result = result.strip(' ')
    return result

def find_longest_string(strings):
    longest = ''
    for s in strings:
        if len(s) > len(longest):
            longest = s
    print('The longest string is: ' + longest)

input = ['These', 'Are', 'Some', 'Strings']
r = every_other_string(input)
print(r)

input = ['It', 'was', 'the', 'best', 'of', 'times']
r = every_other_string(input)
print(r)

input = ['AA', 'BBBB', 'C', 'DDD', 'EEEEEEE']
find_longest_string(input)

input = ['one', 'two', 'three', 'four', 'five', 'six', 'seven']
find_longest_string(input)