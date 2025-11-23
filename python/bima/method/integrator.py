
from enum import IntEnum


class Integrator(IntEnum):
    Euler = 0
    RK4 = 1
    BS = 2
    LeapFrog = 3
