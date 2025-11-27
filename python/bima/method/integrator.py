from bima import _bima


class Integrator:
    def __init__(self, id: int, v):
        self.v = v
        self.id = id

    @staticmethod
    def euler(dt: float):
        return Euler(dt)

    @staticmethod
    def rk4(dt: float):
        return RK4(dt)

    @staticmethod
    def bs(dt: float, tol=1e-5, n_try=20):
        return BS(dt, tol, n_try)

    @staticmethod
    def leap_frog(dt: float):
        return LeapFrog(dt)


class Euler(Integrator):
    def __init__(self, dt: float):
        v = _bima.Euler(dt)
        super().__init__(0, v)


class RK4(Integrator):
    def __init__(self, dt: float):
        v = _bima.Rk4(dt)
        super().__init__(1, v)


class BS(Integrator):
    def __init__(self, dt: float, tol: float, n_try: int):
        v = _bima.Bs(dt, tol, n_try)
        super().__init__(2, v)


class LeapFrog(Integrator):
    def __init__(self, dt: float):
        v = _bima.LeapFrog(dt)
        super().__init__(3, v)
