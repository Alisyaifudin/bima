from bima.trajectory import Trajectory
from bima import _bima
from numpy.typing import NDArray
import numpy as np


class Energy:
    def __init__(self, t: NDArray[np.float64], e: NDArray[np.float64]):
        self.t = t
        self.e = e

    @classmethod
    def from_bodies(cls, bodies: list[Trajectory], n_active: int | None = None, progress=False):
        if n_active is None or n_active > len(bodies):
            n_active = len(bodies)
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
        energy = _bima.calc_energy(objects, masses, n_active, progress)
        ins = cls(energy[0], energy[1])
        return ins

    def __repr__(self) -> str:
        list = np.array([[t, e] for e, t in zip(self.e, self.t)])
        return str(list)
