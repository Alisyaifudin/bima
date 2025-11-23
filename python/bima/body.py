import numpy as np
from numpy.typing import NDArray

class Body:
    def __init__(self, v: list[list[float]], id: int, m: float):
        self.id = id
        self.m = m
        arr = np.array(v)
        shape = arr.shape
        self.t = arr[:,0]
        self.x = arr[:,1]
        self.y = arr[:,2]
        self.z = arr[:,3]
        self.vx = arr[:,4]
        self.vy = arr[:,5]
        self.vz = arr[:,6]
        if shape[1] > 7:
          self.ax = arr[:,7]
          self.ay = arr[:,8]
          self.az = arr[:,9]
        else:
          self.ax = None
          self.ay = None
          self.az = None

    def __len__(self):
        return len(self.t)

    def __repr__(self) -> str:
        return f"Body(id={self.id})"

    def __str__(self) -> str:
        return self.__repr__()
       