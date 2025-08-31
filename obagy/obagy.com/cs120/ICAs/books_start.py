
class BookData:
    
    def __init__(self, author, title, rating):
        self._author = author
        self._title = title
        self._rating = rating

    def get_title():
        
    def get_author():

    def get_rating():

    def __str__():


def main():
    book_list = []
    answer = 'yes'
    while answer != 'no':
        title = input("Book: " )
        author = input("Author: ")
        rating = int(input("Rating: "))
        b = BookData(author, title, rating)
        book_list.append(b)
        answer = input('Enter another book? Answer yes or no: ')

    # your code goes here

main()
