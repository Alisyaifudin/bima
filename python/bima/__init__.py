"""Bima - A Python library for running n-body simulation powered by Rust backend"""

__version__ = "0.6.0"

# Import the private Rust module
from bima import _bima

# Re-export only what you want public
from bima.method.force import Force
from bima.method.integrator import Integrator
from bima.simulation import Simulation
from bima.energy import Energy
from bima.trajectory import Trajectory
from bima.body import Body


# (Optional) Clean up namespace
__all__ = ["Force", "Trajectory",
           "Integrator",  "Simulation", "Body",
           "Energy", "__version__"]
