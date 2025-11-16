import numpy as np
import bima
from matplotlib import pyplot as plt
from bima import Config

arr = np.array([[1, -0.5, 0, 0, 0, -0.5 * 1.5, 0], [1.5, 0.5, 0, 0, 0, 0.5, 0]])
initial = bima.Initial.from_arr(arr)

config = Config(
    force=bima.ForceMethod.Direct,
    solve=bima.SolveMethod.Euler,
    timestep=bima.TimestepMethod.Constant(0.00001),
    close_encounter=bima.CloseEncounterMethod.Regularized
)

sim = bima.Simulation(initial)

sim.run(config, 3)

record = np.array(sim.record())

x1 = record[0, ::1000, 1]
y1 = record[0, ::1000, 2]
x2 = record[1, ::1000, 1]
y2 = record[1, ::1000, 2]

fig, ax = plt.subplots()
ax.plot(x1, y1)
ax.scatter(x1[-1], y1[-1])
ax.plot(x2, y2)
ax.scatter(x2[-1], y2[-1])
plt.show()
