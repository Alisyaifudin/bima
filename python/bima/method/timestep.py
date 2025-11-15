from typing import Union

class _Constant:
    value = 0

    def __init__(self, delta_t: float):
        self.delta_t = delta_t

    def __repr__(self):
        return f"TimestepMethod.Constant({self.delta_t})"

class _Adaptive:
    value = 1
    delta_t = None

    def __repr__(self):
        return "TimestepMethod.Adaptive"

class TimestepMethod:
    Adaptive = _Adaptive()

    @staticmethod
    def Constant(value: float) -> _Constant:
        return _Constant(value)

type TimestepMethodType = Union[_Adaptive, _Constant]
