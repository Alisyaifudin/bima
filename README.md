N-body simulation library in Rust with Python bindings.

# Features
- [-] Integrators:
  - [x] Euler
  - [x] Runge-Kutta 4
  - [x] Bulirsch-Stoer
  - [x]Leapfrog (Symplectic)
  - [ ] Wisdom-Holman
- [ ] Adaptive time stepping.
- [ ] Collision detection and handling.
  - [ ] Solid body collisions.
  - [ ] Merging bodies.
- [-] Close encounter detection.
  - [x] Softening.
  - [x] Truncated.
  - [ ] Regularization.
- [ ] Force calculation.
  - [x] Direct summation.
  - [ ] Barnes-Hut tree (octree).
  - [ ] Fast multipole method.
- [ ] Parallelization.
  - [ ] CPU multi-threading.
  - [ ] GPU acceleration.

# Installation
```bash
pip install bima
```

# Quick Start

```python
from bima.body import Body
from bima import Config
import numpy as np
import bima
from matplotlib import pyplot as plt

v = np.sqrt(2)
arr = np.array([[1, -1, 0, 0, 0, -2/3 * v, 0], [2, 0.5, 0, 0, 0, 1/3 * v, 0]])
initial = bima.Initial.from_arr(arr)

config = Config(
    force=bima.ForceMethod.Direct,
    # integrator=bima.Integrator.Euler,
    integrator=bima.Integrator.LeapFrog,
    timestep=bima.TimestepMethod.Constant(0.1),
    # timestep=bima.TimestepMethod.Constant(1),
    close_encounter=bima.CloseEncounterMethod.Regularized,
)

sim = bima.Simulation(initial)

bodies = sim.in_memory.run(config, 100)
# bodies = sim.in_memory.run(config, 1)

# print(len(energy.t))


def plot(bodies: list[Body]):
    body0 = bodies[0]
    length = len(body0)
    sample_n = np.min([1000, length])
    skip = length//sample_n
    x0 = body0.x[::skip]
    y0 = body0.y[::skip]

    body1 = bodies[1]
    length = len(body1)
    skip = length//sample_n
    x1 = body1.x[::skip]
    y1 = body1.y[::skip]

    fig, ax = plt.subplots()
    ax.plot(x0, y0)
    ax.scatter(x0[-1], y0[-1])
    ax.plot(x1, y1)
    ax.scatter(x1[-1], y1[-1])
    ax.set_aspect("equal")
    plt.show()


# plot(bodies)

energy = bima.Energy.from_bodies(bodies)
e0 = energy.e[0]
plt.plot(energy.t, (e0-energy.e)/e0)
plt.show()
```


# License
GNU General Public License v3.0. See LICENSE for more details.

# Documentation

under construction