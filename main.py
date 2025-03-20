import time

from ml import filter_image

if __name__ == "__main__":
    start_time = time.process_time()
    filter_image("demo.jpg", "demo-out.jpg", 3)
    end_time = time.process_time()
    print("Execution time:", end_time - start_time)
