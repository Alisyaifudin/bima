from typing import Self
from attr import dataclass
from numpy.typing import NDArray
import numpy as np
from bima import _bima


class Initial:
    _initial: list[_bima.Initial]

    def __init__(self, _initial) -> None:
        self._initial = _initial

    @classmethod
    def from_arr(cls, data: NDArray[np.float64]) -> Self:
        """
        set the initial position of the celestial bodies

        Args:
            data: List of floating point numbers

        Returns:
            Initial instance
        Raises:
            ValueError: Incorrect dimension
        """
        shape = data.shape
        if len(shape) != 2 or shape[1] != 7:
            raise ValueError(
                f"Incorrect dimension, should be (n, 7). shape = {shape}")
        m = data[:, 0].tolist()
        x = data[:, 1].tolist()
        y = data[:, 2].tolist()
        z = data[:, 3].tolist()
        vx = data[:, 4].tolist()
        vy = data[:, 5].tolist()
        vz = data[:, 6].tolist()

        initial = cls(_bima.set_initial(m, x, y, z, vx, vy, vz))
        return initial

    def __repr__(self) -> str:
        return self._initial.__repr__()

    def __str__(self) -> str:
        return self.__repr__()
