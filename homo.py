import numpy as np;
import cv2

pts_src = np.float64([
1384.75, 562,1391, 680.5,1022.25, 558.75,1022, 675,
]).reshape(-1,1,2);

pts_dst = np.float64([
150, 150,150, 270,544, 150,544, 270,
]).reshape(-1,1,2)

h, status = cv2.findHomography(pts_src, pts_dst)

print(h)
res = h @ np.float64([1384.5, 562, 1])
print(res[0]/res[2], res[1]/res[2])