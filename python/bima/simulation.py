from bima.method.close_encounter import CloseEncounterMethodType
from bima.method.force import ForceMethod
from bima.method.solve import SolveMethod
from bima.method.timestep import TimestepMethodType
from bima import _bima
from bima.initial import Initial
from dataclasses import dataclass
from typing import Union


@dataclass
class Config:
    force: ForceMethod
    solve: SolveMethod
    timestep: TimestepMethodType
    close_encounter: CloseEncounterMethodType


class Simulation:
    def __init__(self, initial: Initial, save_acceleration=False) -> None:
        self.initial = initial
        self._sim = _bima.Simulation(initial._initial, save_acceleration)

    def run(self, config: Config, t_stop: float):
        if t_stop <= 0:
            raise ValueError("t_stop must be positive")
        self._sim.run(config.force, config.solve, config.timestep.value, config.close_encounter.value,
                      t_stop, config.timestep.delta_t, config.close_encounter.par)

    def record(self):
        return self._sim.record
