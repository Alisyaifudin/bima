from bima.body import Body
import numpy as np
import bima
from matplotlib import pyplot as plt
from bima import Config

c_vx = 0.02

# arr = np.array([[1, -1, 0, 0, 0 + c_vx, -1, 0], [2, 0.5, 0, 0,
#                0 + c_vx, 0.5, 0], [0.1, 0, 10, 0, -0.6 + c_vx, 0, 0]])
# arr = np.array([[1, -1, 0, 0, 0.5, -1, 0], [2, 0.5, 0, 0,
#                -0.2, 0.5, 0], [0.1, 0, 10, 0, -0.2, 0.2, 0]])
np.random.seed(123402)
n = 3
m = np.random.random(n) + 1
pos = (np.random.random((n, 3))-0.5)*10
vel = (np.random.random((n, 3))-0.5)*1
arr = np.column_stack((m, pos, vel))
initial = bima.Initial.from_arr(arr)

config = Config(
    force=bima.ForceMethod.Direct,
    # solve=bima.SolveMethod.Euler,
    # solve=bima.SolveMethod.RK4,
    integrator=bima.SolveMethod.BS,
    # timestep=bima.TimestepMethod.Constant(1),
    timestep=bima.TimestepMethod.Constant(0.001),
    close_encounter=bima.CloseEncounterMethod.Regularized,
)

sim = bima.Simulation(initial)

bodies = sim.in_memory.run(config, 35)


def plot(bodies: list[Body]):
    sample_n = 1000
    fig, ax = plt.subplots()
    for body in bodies:
        length = len(body)
        skip = length//sample_n
        x = body.x[::skip]
        y = body.y[::skip]
        ax.plot(x, y)
        ax.scatter(x[-1], y[-1])
    plt.show()


plot(bodies)

# energy = bima.Energy.from_bodies(bodies)
# e0 = energy.e[0]
# plt.plot(energy.t, (e0-energy.e)/e0)
# plt.show()
