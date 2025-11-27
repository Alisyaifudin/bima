from bima import _bima

class Force:
    def __init__(self, id: int, v) -> None:
        self.id = id
        self.v = v

    @staticmethod
    def direct(s: float):
        return Direct(s)

    # @staticmethod
    # def octree(cls, s: float) -> Self:
    #     return cls(1, s)


class Direct(Force):
    def __init__(self, s: float):
        v = _bima.Direct(s)
        super().__init__(0, v)
