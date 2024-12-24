
from z3 import *

vars = [2, 4,1,5,7,5,1,6,4,2,5,5,0,3,3,0]
a = BitVec('a', 64)

s = Optimize()
s.minimize(a)
for i in range(0, len(vars)):
    ai = a >> (3 * i)
    b = ai % 8
    b = b ^ 5
    c = ai >> b
    b = b ^ 6
    b = b ^ c
    s.add((b % 8 == vars[i]))
print(s.check())
print(s.model()[a])
