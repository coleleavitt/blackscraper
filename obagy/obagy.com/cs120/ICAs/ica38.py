#Buggy code for ICA-38

def calculate_bmi(weight, height):
    return weight / (height ** 2)

def first_matches_last(alist):
    match_list = []
    for elem in alist:
        if elem[0] == elem[-1]:
            match_list.append(elem)

    return match_list

class Word:
    def __init__(self, word):
        self._word = word

    def __eq__(self, other):
        if len(self._word) == len(other._word):
            for letter in self._word.lower():
                if letter not in other._word.lower():
                    return False
        return True

def main():
    #Prob 4
    patients = [(70, 1.8), (80, 1.9), (150, 1.7)]
    for patient in patients:
        weight, height = patients[0]
        bmi = calculate_bmi(height, weight)
        print("Patient's BMI is:", bmi)

    #Print 5
    #Call with a list that causes a error
    #print(first_matches_last()

    #Prob 6
    w1 = Word("post")
    w2 = Word("stop")
    print(w1 == w2)

    w1 = Word("keep")
    w2 = Word("peep")
    print(w1 == w2)

    #create two word objects that cause the
    #the call to "==" to return True when
    # it should return False
   

main()
