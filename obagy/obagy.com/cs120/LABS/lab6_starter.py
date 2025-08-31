#Lab 6
import string

#Node class
class Node:
    def __init__(self, value):
        self._value = value
        self._next = None
    
    def value(self):
        return self._value
    
    def next(self):
        return self._next

    def __str__(self): 
       if self._next is None:
           ending = ""
       else:
           ending = " -> "
       return str(self._value) + ending
        
#Linked list class
class LinkedList:
    def __init__(self):
        self._head = None

    def is_empty(self):
        return self._head == None

    # add a node to the head of the list
    def add(self, node):
        node._next = self._head
        self._head = node

    # remove a node from the head of the list and return the node
    def remove(self):
        _node = self._head
        self._head = _node._next
        _node._next = None
        return _node
    
    def __str__(self):
        string = 'List -> '
        curr_node = self._head
        while curr_node != None:
            string += str(curr_node)
            curr_node = curr_node.next()
        return string

#Problem 1, Step 2
class Stack :
    # the top is the first element of the LinkedList
    def __init__(self):
        # your code goes here


    # push adds item to the LinkedList
    def push(self, item):
        # your code goes here


    # pop removes item from the LinkedList and returns it
    def pop(self):
        # your code goes here



    def is_empty(self):
        # your code goes here


    
    def __str__(self):
        # your code goes here

#Problem 2, Step 2
def remove_punc(title):
    pass

#Problem 3, Step 6
def string_comp():
    pass

def main():

    # Problem 1, Step 3
    # create a stack


    # push the numbers 2, 4, 6, and 8 on the stack






    #print the stack 



    # on the lab handout, draw a diagram of the underlying
    # LinkedList as it is at this point 
    # (draw it under Problem 1, Step 3 on the lab handout)


    # pop an element off the stack and print it
    # to verify that it is a number and not a Node




    # print the result of calling is_empty() on your stack

    #Problem 2, Step 2
    #Ucomment the lines below once you have implemented the remove_punc()
    #print(repr(remove_punc("It's a dog's life, full of treats!")))
    #print(repr(remove_punc("It's amazing!! --How'd you do that?!!")))


    #Problem 3, Step 4
    #Uncomment the line below after you have filled out the code for string_comp()
    #string_comp()
    print("hello")

main()
