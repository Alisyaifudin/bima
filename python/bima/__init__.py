"""Bima - A Python library for running n-body simulation powered by Rust backend"""

__version__ = "0.5.0"

# Import the private Rust module
from bima import _bima

# Re-export only what you want public
from bima.initial import Initial
from bima.method.close_encounter import CloseEncounterMethod
from bima.method.force import ForceMethod
from bima.method.integrator import Integrator
from bima.method.timestep import TimestepMethod
from bima.simulation import Simulation
from bima.simulation import Config
from bima.energy import Energy
from bima.body import Body

# (Optional) Clean up namespace
__all__ = ["Initial", "CloseEncounterMethod", "ForceMethod",
           "Integrator", "TimestepMethod", "Simulation", 
           "Config", "Energy", "Body" "__version__"]
