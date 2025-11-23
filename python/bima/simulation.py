from bima.body import Body
from bima.disk import Disk
from bima.method.close_encounter import CloseEncounterMethodType
from bima.method.force import ForceMethod
from bima.method.integrator import Integrator
from bima.method.timestep import TimestepMethodType
from bima import _bima
from bima.initial import Initial
from dataclasses import dataclass


@dataclass
class Config:
    force: ForceMethod
    integrator: Integrator
    timestep: TimestepMethodType
    close_encounter: CloseEncounterMethodType
    save_acceleration: bool = False


class Simulation:
    def __init__(self, initial: Initial) -> None:
        self.initial = initial
        self._sim = _bima.Simulation(initial._initial)
        self.in_memory = InMemory(self)

    def in_disk(self, dir_path: str, replace=False):
        return InDisk(self, dir_path, replace)


class InMemory:
    def __init__(self, simulation: Simulation):
        self.simulation = simulation

    def run(self, config: Config, t_stop: float) -> list[Body]:
        if t_stop <= 0:
            raise ValueError("t_stop must be positive")
        record: list[list[list[float]]] = self.simulation._sim.run_memory(config.force, config.integrator, config.timestep.value, config.close_encounter.value,
                                                                          t_stop, config.timestep.delta_t, config.close_encounter.par, config.save_acceleration)
        # print("raw\n", record[0])
        bodies: list[Body] = []
        for i, body in enumerate(record):
            bodies.append(Body(body, i, self.simulation.initial.m[i]))
        return bodies


class InDisk:
    def __init__(self, simulation: Simulation, dir_path: str, replace=False):
        self.simulation = simulation
        self.dir_path = dir_path
        self.replace = replace

    def run(self, config: Config, t_stop: float) -> Disk:
        if t_stop <= 0:
            raise ValueError("t_stop must be positive")
        path = self.simulation._sim.run_disk(self.dir_path, config.force, config.integrator, config.timestep.value, config.close_encounter.value,
                                             t_stop, config.timestep.delta_t, config.close_encounter.par, config.save_acceleration, self.replace)
        return Disk(path)
