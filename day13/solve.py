import z3
from z3 import *

def solve_sol(x1, y1, x2, y2, x3, y3):
    a = Int('a')
    b = Int('b')
    solver = Optimize()
    solver.add(And(a * x1 + b * x2 == x3))
    solver.add(And(a * y1 + b * y2 == y3))
    solver.add(And(a >= 0))
    solver.add(And(b >= 0))
    solver.minimize(3 * a + b)
    solver.check()
    res = solver.model()
    if res[a] is None or res[b] is None:
        return []
    return [res[a].as_long(), res[b].as_long()]

# print(solve_sol(94, 34, 22, 67, 8400, 5400))
