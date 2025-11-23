from bima.body import Body
from bima import _bima
from numpy.typing import NDArray
import numpy as np


class Energy:
    def __init__(self, t: NDArray[np.float64], e: NDArray[np.float64]):
        self.t = t
        self.e = e

    @classmethod
    def from_bodies(cls, bodies: list[Body]):
        objects = []
        masses = []
        for body in bodies:
            object = []
            for i in range(len(body)):
                t = body.t[i]
                x = body.x[i]
                y = body.y[i]
                z = body.z[i]
                vx = body.vx[i]
                vy = body.vy[i]
                vz = body.vz[i]
                object.append([t, x, y, z, vx, vy, vz])
            objects.append(object)
            masses.append(body.m)
        energy = _bima.calc_energy(objects, masses)
        ins = cls(energy[0], energy[1])
        return ins
