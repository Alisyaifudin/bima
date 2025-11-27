import bima.simulation
from bima.trajectory import Trajectory
import numpy as np
import bima
from matplotlib import pyplot as plt


# np.random.seed(1234567)
# n = 3
# m = np.random.random(n) + 1
# pos = (np.random.randn(n, 3)-0.5)*5
# vel = (np.random.randn(n, 3)-0.5)*1
# bodies = []
# for i in range(n):
#     b = bima.Body.from_vec(m[i], pos[i], vel[i])
#     bodies.append(b)
v = np.sqrt(1/5)
sin_30 = np.sin(np.pi/6)
cos_30 = np.cos(np.pi/6)
# b0 = bima.Body.from_vec(1, [-1, 0, 0], [0, -2/3*v, 0])
b0 = bima.Body.from_vec(1.0000001, [0, 1, 0], [-v, 0, 0])
b1 = bima.Body.from_vec(0.9999999, [-cos_30, -sin_30, 0], [v*sin_30, -v*cos_30, 0])
b2 = bima.Body.from_vec(1, [cos_30, -sin_30, 0], [v*sin_30, v*cos_30, 0])
bodies = [b0, b1, b2]

force = bima.Force.direct(s=0)
integrator = bima.Integrator.leap_frog(dt=0.001)
# integrator = bima.Integrator.euler(dt=0.001)
# integrator = bima.Integrator.rk4(dt=0.001)
# integrator = bima.Integrator.bs(dt=0.001, tol=1e-2)

sim = bima.Simulation(integrator=integrator, force=force)

record = sim.run(bodies, t_stop=23, progress=True)


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


plot(record)

# energy = bima.Energy.from_bodies(record)
# e0 = energy.e[0]
# plt.plot(energy.t[1:], (e0-energy.e[1:])/e0)
# plt.show()
