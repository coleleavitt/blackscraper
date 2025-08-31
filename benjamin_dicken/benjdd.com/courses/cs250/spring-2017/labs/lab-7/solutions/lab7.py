
###
### For each key in data, count the number values that match it.
###
def keys_matching_values(data):
    for key in data.keys():
        count = 0
        for value in data.values():
            if key == value:
                count += 1
        print('Key ' + str(key) + ' matches ' + str(count) + ' values')

###
### Count the number of values in data that match name.
###
def value_count(data, name):
    count = 0
    for val in data.values():
        if name == val:
            count += 1
    return count

###
### Returns a sorted list containing both the keys and values from stuff.
###
def sort_all(stuff):
    result = []
    result.extend(stuff.keys())
    result.extend(stuff.values())
    result.sort()
    return result

###
### Returns the top-2 selling items from the sales dctionary
###
def get_top_items(sales):
    first    = ''
    first_c  = 0
    second   = ''
    second_c = 0
    for key,val in sales.items():
        if val >= first_c:
            first   = key
            first_c = val
    for key,val in sales.items():
        if val >= second_c and key != first:
            second   = key
            second_c = val
    print('Best selling item: ' + first + ' (' + str(first_c) + ' sold)')
    print('Second-best selling item: ' + second + ' (' + str(second_c) + ' sold)')

###
### Convert the contents of file_name to be all upper-case
###
def upper_file(file_name):
    f = open(file_name, 'r')
    contents = f.read()
    contents = contents.upper()
    f = open(file_name, 'w')
    f.write(contents)
    f.close()

###
### Returns the top-2 selling items from the sales db file
###
def get_top_items_from_file(file_name):
    first    = ''
    first_c  = 0
    f = open(file_name, 'r')
    for line in f:
        sp = line.split(':')
        key = sp[0].strip()
        val = int(sp[1].strip())
        if val >= first_c:
            first   = key
            first_c = val
    print('Best selling item: ' + first + ' (' + str(first_c) + ' sold)')

