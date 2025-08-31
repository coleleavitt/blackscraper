'''
Lab 11 Solutions
__________________

Problem 1 - terms
An immutable data type is a type whose values cannot be modified. 
For example, strings and tuples are immutable.

A hash function maps data of arbitrary size and type to an
integer in a fixed range.

For linear probing, when a collision occurs when attempting to 
put a key (or key/value pair) in the hash table, you repeatedly
decrement the index until a free slot is found. If the index 
becomes negative, your wrap around to the end of the underlying 
list. You must also use this strategy for all of the hash table 
operations, i.e., get() and contains().

A queue is a linear data structure, which means that the data
has an ordering. Elements are added from one end of the 
data structure and removed from the other end of the 
data structure. This property is called First in, first, out.


Problem 2 - complexity
1.
  a) The function returns a list of tuples where the 
     tuples are all the possible pairs of values in 
     the list "data" that add to 0.

  b) O(nxn) -- this is n squared

2. Binary Search. The list must be sorted.

3.
  a) O(log n)
  b) O(n)

Problem 3 - testing
1. Black box
 a) Error cases
    The file does not exist
    The file is not readable (it's an image file or something else)
    The file contains lines that don't have correct name-age pairs
 
 b) Edge cases
    An empty file
    A file containing one line with a  name-age pair
    A file containing millions of lines with name-age pairs

 c) Normal cases
    A file with several lines consisting of name-age pairs

2. White box (unit testing)
test 1: [], 42 
test 2: [3,3,3,3], 3
test 3: [1,2,3,4,5], 5
test 4: [1,2,3,4,5], "hello"

Problem 4 - hashing with separate chaining
Step 1.
__________________________________________
|  0  |  1  |  2  |  3  |  4  |  5  |  6 |
------------------------------------------
|     |  ll |     |  ll |     |  ll |    |
---------|-----------|-----------|--------
         |           |           |
         8           17          26
                     |           |
                     24          19


Step 3.
        See the code for put() below in the Hashtable class
Step 4.
        The LinkedList class would need a method
        that searches for a value in a linked list
        and returns True if it is there and 
        False otherwise.

'''

'''
Node and Linked classes for Problem 4 - hashing
'''
class Node:
    def __init__(self, value):
        self._value = value
        self._next = None
    
    def __str__(self):
        return str(self._value) + " ->  "
    
    def value(self):
        return self._value
    
    def set_value(self, value):
        self._value = value

    def set_next(self, node):
        self._next = node
    
    def next(self):
        return self._next

    def str(self):
        return str(node._value) + " ->  "
        
class LinkedList:
    def __init__(self):
        self._head = None

    def is_empty(self):
        return self._head == None

    def get_head(self):
        return self._head

    # add a node to the head of the list
    def add(self, node):
        node._next = self._head
        self._head = node

    def __str__(self):
        string = 'LL -> '
        curr_node = self._head
        while curr_node != None:
            string += str(curr_node)
            curr_node = curr_node.next()
        #string += ']'
        return string

'''
Hashtable class for Problem 4

'''
class Hashtable:
    def __init__(self, capacity):
        self._pairs = [None] * capacity 
        self._size = capacity

    def _hash(self, key):
        return key % 7

    # Note: once you have put a key in the 
    # hash table, print out the linked list
    # at that slot (index)
    def put(self, key):
        # your code goes here for Problem 4
        i = self._hash(key)
        if self._pairs[i] == None:
            llist = LinkedList()
            llist.add(Node(key)) 
            self._pairs[i] = llist
        else:
            llist = self._pairs[i]
            llist.add(Node(key))
        # print statement for debugging
        # print the linked list at the 
        # slot that the key hashed to 
        print(self._pairs[i])

def main():

    ht = Hashtable(7)
    ht.put(8)
    ht.put(24)
    ht.put(19)
    ht.put(17)
    ht.put(26)

main()
