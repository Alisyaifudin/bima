import bima.simulation
from bima.trajectory import Trajectory
import numpy as np
import bima
from matplotlib import pyplot as plt

v = np.sqrt(2)
b0 = bima.Body.from_vec(1, [-1, 0, 0], [0, -2/3*v, 0])
b1 = bima.Body.from_vec(2, [0.5, 0, 0], [0, 1/3*v, 0])
bodies = [b0, b1]

force = bima.Force.direct(s=0)
integrator = bima.Integrator.leap_frog(dt=0.01)
# integrator = bima.Integrator.euler(dt=0.001)
# integrator = bima.Integrator.rk4(dt=0.001)
# integrator = bima.Integrator.bs(dt=0.001)

sim = bima.Simulation(integrator=integrator, force=force)

record = sim.run(bodies, t_stop=200, progress=True)


def plot(bodies: list[Trajectory]):
    length = len(bodies[0])
    sample_n = np.min([1000, length])
    skip = length//sample_n
    print(length)

    fig, ax = plt.subplots()
    for body in bodies:
        x = body.x[::skip]
        y = body.y[::skip]
        ax.plot(x, y)
        ax.scatter(x[-1], y[-1])
    ax.set_aspect("equal")
    plt.show()


# plot(record)

energy = bima.Energy.from_bodies(record)
e0 = energy.e[0]
# print(energy)
plt.plot(energy.t[1:], (e0-energy.e[1:])/e0)
plt.show()
