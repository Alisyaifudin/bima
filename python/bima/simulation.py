from bima.body import Body
from bima.trajectory import Trajectory
from bima.method.integrator import Integrator
from bima.method.force import Force
from bima import _bima

# @dataclass
# class Config:
#     force: ForceMethod
#     integrator: Integrator
#     timestep: TimestepMethodType
#     close_encounter: CloseEncounterMethodType
#     save_acceleration: bool = False

sims = {
    (0, 0): _bima.Direct_Euler,
    (0, 1): _bima.Direct_Rk4,
    (0, 2): _bima.Direct_Bs,
    (0, 3): _bima.Direct_LeapFrog,
}


class Simulation:
    def __init__(self, integrator: Integrator, force: Force) -> None:
        self.integrator = integrator
        self.force = force
        try:
            sim = sims[(force.id, integrator.id)]
            self.sim = sim(integrator.v, force.v)
        except KeyError as e:
            raise KeyError(
                f"The integrator and/or force selected are not available\n {e}")

    def run(self, bodies: list[Body], t_stop: float, n_active: float | None = None,
            save_acc=False, progress=False):
        n_active = len(bodies) if n_active is None else n_active
        body_list = to_list(bodies)
        record: list[list[list[float]]] = self.sim.run(
            body_list, n_active, t_stop, save_acc, progress)
        b: list[Trajectory] = []
        b.clear()
        for i, body in enumerate(record):
            m = bodies[i].m
            b.append(Trajectory(body, i, m))
        return b


def to_list(bodies: list[Body]) -> list[list[float]]:
    b = []
    for body in bodies:
        arr = [body.m, body.x, body.y, body.z, body.vx, body.vy, body.z]
        b.append(arr)
    return b
