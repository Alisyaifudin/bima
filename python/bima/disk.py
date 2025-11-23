from typing import Optional
from bima.body import Body
import numpy as np
from numpy.typing import NDArray
import h5py


class BodyLazy:
    def __init__(self, group: h5py.Group):
        self.group = group
        self.m: float = group["m"][0]
        self.cache: dict[str, Optional[NDArray[np.float64]]] = dict(
            t=None, x=None, y=None, z=None, vx=None, vy=None, vz=None, ax=None, ay=None, az=None)

    def _read(self, name: str) -> NDArray[np.float64]:
        if self.cache[name] is not None:
            return self.cache[name]
        try:
            g = self.group[name]
        except KeyError:
            raise KeyError(f"The key {name} does not exist")
        v = []
        for chunk in g.values():
            data = chunk[:]
            v.extend(data)
        v = np.array(v).flatten()
        self.cache[name] = v
        return v

    def __len__(self):
        t = self.t()
        return len(t)

    def t(self):
        return self._read("t")

    def x(self):
        return self._read("x")

    def y(self):
        return self._read("y")

    def z(self):
        return self._read("z")

    def vx(self):
        return self._read("vx")

    def vy(self):
        return self._read("vy")

    def vz(self):
        return self._read("vz")

    def ax(self):
        return self._read("ax")

    def ay(self):
        return self._read("ay")

    def az(self):
        return self._read("az")

    def __repr__(self) -> str:
        return f"Body(id={self.group.name})"

    def __str__(self) -> str:
        return self.__repr__()


class DiskFile:
    file: Optional[h5py.File] = None

    def __init__(self, path: str, n: int) -> None:
        self.path = path
        self.n = n

    def __enter__(self):
        file = h5py.File(self.path)
        self.file = file
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        # Handle exceptions if needed
        if exc_type is not None:
            print(f"An exception occurred: {exc_type.__name__}: {exc_value}")
        if self.file is not None:
            self.file.close()
            self.file = None
        # Return False to let exceptions propagate
        return False

    def get(self, i: int) -> BodyLazy:
        if self.file is None:
            raise ValueError("No file")
        if i < 0:
            raise ValueError("index cannot be negative")
        if i >= self.n:
            raise ValueError(
                f"index cannot be larger than the total member: {self.n}")
        bodies = self.file['objects']
        return  BodyLazy(bodies[f"{i}"])
        


class Disk:
    _t: Optional[NDArray[np.float64]]

    def __init__(self, path: str):
        self.path = path
        with h5py.File(self.path) as f:
            self.n = len(f['objects'])

    def open(self):
        return DiskFile(self.path, self.n)

    def __repr__(self) -> str:
        return f"Disk(path={self.path})"

    def __str__(self) -> str:
        return self.__repr__()
