
class Body:
    def __init__(self, w: list[float], active: bool = True):
        if len(w) != 7:
            raise ValueError("w should be of size 7")
        self.m = w[0]
        self.x = w[1]
        self.y = w[2]
        self.z = w[3]
        self.vx = w[4]
        self.vy = w[5]
        self.vz = w[6]
        self.active = active

    @classmethod
    def from_vec(cls, m: float, r: list[float], v: list[float], active: bool = True):
        if (len(r) != 3 and len(v) != 3):
            raise ValueError("Invalid vec size")
        w = [m, r[0], r[1], r[2], v[0], v[1], v[2]]
        return cls(w, active)

    def __repr__(self) -> str:
        return f"body(m={self.m}, r=[{self.x}, {self.y}, {self.z}], v=[{self.vx}, {self.vy}, {self.vz}])"

    def __str__(self) -> str:
        return self.__repr__()
