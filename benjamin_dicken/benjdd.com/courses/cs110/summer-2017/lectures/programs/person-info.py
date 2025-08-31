def summarize_person(name, age, bday, bio):
    print('Name:     ' + name)
    print('Age:      ' + str(age))
    print('Birthday: ' + bday)
    print('Bio:      ' + bio)
    
def get_name():
    name = input('Enter your name: ')
    return name

def get_age():
    age = int(input('Enter your age: '))
    if age < 16:
        print('Remember, don\'t drive!')
    return age

def get_birthday():
    bday = input('Enter your birthday (MM/DD/YYYY): ')
    return bday

def get_bio():
    bio = input('Tell me a little about yourself: ')
    return bio

def main():
    n = get_name()
    a = get_age()
    bd = get_birthday()
    bio = get_bio()
    summarize_person(n, a, bd, bio)
    
    # Could also just do this:
    # summarize_person(get_name(), get_age(), get_birthday(), get_bio())
    
main()