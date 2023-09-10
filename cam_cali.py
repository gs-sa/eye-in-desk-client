import numpy as np;

x = np.float64([1211.25, 534]);
b = np.float64([0.4,0]);

A = b @ x.T / x.T@x
print(x / (x.T@x))