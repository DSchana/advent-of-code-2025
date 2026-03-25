import numpy as np
from scipy.signal import convolve2d

def count_neighbours(A):
  K = np.array([
    [1,1,1],
    [1,0,1],
    [1,1,1]])
  return convolve2d(A, K, mode='same', boundary='fill', fillvalue=0)

if __name__ == '__main__':
  with open('day4-input') as f:
    lines = f.read().strip().split('\n')
    A = np.array([[1 if c == '@' else 0 for c in line] for line in lines])
    total = 0
    while True:
      N = count_neighbours(A)
      a = np.sum((A == 1) & (N < 4))
      if a == 0:
        break
      A = (A & (N >= 4)).astype(int)
      total += a
    print(total)
