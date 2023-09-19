from sympy import symbols, pprint
from galgebra.ga import Ga
from galgebra.printer import Format

# Format(Fmode=False, Dmode=True)

s4coords = (x, y, z, w) = symbols("x y z w", real=True)
s4 = Ga("e", g=[1, 1, 1, 1], coords=s4coords)

a_s = s4.mv("a.s", 'scalar')
a_bv = s4.mv("a.bv", 'bivector')
rotor_a = a_s + a_bv

b_s = s4.mv("b.s", 'scalar')
b_bv = s4.mv("b.bv", 'bivector')
rotor_b = b_s + b_bv

v = s4.mv("v", 'vector')

def compose():
    stringed = (rotor_a * rotor_b).Fmt(3)
    print(str(stringed).replace("__", "."))

def rotate():
    stringed = (rotor_a * v * rotor_a.rev()).Fmt(3)
    print(str(stringed).replace("__", "."))


rotate()
