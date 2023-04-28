import time
import random


class Point:
    def __init__(self):
        self.x = 0.0
        self.y = 0.0

    def move(self) -> None:
        self.x += random.gauss(0.0, 1.0)
        self.y += random.gauss(0.0, 1.0)


def run(num_points: int, num_steps: int) -> None:
    start_time = time.time()
    points = [Point() for _ in range(num_points)]

    for _ in range(num_steps):
        for p in points:
            p.move()

    print(f"(python) Executed in {time.time()-start_time:.2f} seconds")
