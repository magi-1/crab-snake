import snake
import crab

def main():
    print("Executing speed test...")
    num_points, num_steps = 100000, 100
    snake.random_points.run(num_points, num_steps)
    crab.random_points.run(num_points, num_steps)
    
    