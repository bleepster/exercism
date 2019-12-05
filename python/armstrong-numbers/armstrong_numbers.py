from functools import reduce

def is_armstrong_number(number):
    n = [int(i) for i in str(number)]
    c = len(n)
    if c == 1:
        return number**c == number
    return reduce(lambda x,y: x+y**c, n[1:], n[0]**c) == number
