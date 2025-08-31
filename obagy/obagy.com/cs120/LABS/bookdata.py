# Lab 4 Problem 1- exercise on using a class

class BookData:
    def __init__(self, author, title, rating):
        self._author = author
        self._title = title
        self._rating = rating

    def get_title(self):
        return self._title

    def get_author(self):
        return self._author

    def get_rating(self):
        return self._rating

    def __str__(self):
        return self._title + " - " + self._author + " - " + str(self._rating)

def process_data(filename):
    # Step 2: your code goes here

def main():
    filename = input("Enter filename: ")
    
    # Step 3: call your process_data function here
    #         and print the resulting dictionary

    prompt = ''
    while prompt != 'done':
        title = input("Book title: ")
        # Step 4: print the rating of the book here 
        #         or a message if not there

        prompt = input('Enter "done" if finished: ')
 
main()
