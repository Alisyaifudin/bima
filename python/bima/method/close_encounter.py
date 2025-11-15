from typing import Union

class _Truncated:
    value = 0

    def __init__(self, par: float):
        self.par = par

    def __repr__(self):
        return f"CloseEncounter.Truncated({self.par})"

class _Soften:
    value = 1

    def __init__(self, par: float):
        self.par = par

    def __repr__(self):
        return f"CloseEncounter.Soften({self.par})"

class _Regularized:
    value = 2
    par = None

    def __repr__(self):
        return "CloseEncounter.Regularized"

class CloseEncounterMethod:
    Regularized = _Regularized()

    @staticmethod
    def Truncated(value: float) -> _Truncated:
        return _Truncated(value)

    @staticmethod
    def Soften(value: float) -> _Soften:
        return _Soften(value)

type CloseEncounterMethodType = Union[_Regularized, _Truncated, _Soften]