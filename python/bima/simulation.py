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
    save_acceleration: bool = False


class Simulation:
    def __init__(self, initial: Initial) -> None:
        self.initial = initial
        self._sim = _bima.Simulation(initial._initial)

    def run_memory(self, config: Config, t_stop: float) -> list[list[list[float]]]:
        if t_stop <= 0:
            raise ValueError("t_stop must be positive")
        return self._sim.run_memory(config.force, config.solve, config.timestep.value, config.close_encounter.value,
                                    t_stop, config.timestep.delta_t, config.close_encounter.par, config.save_acceleration)
