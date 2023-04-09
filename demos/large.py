import time

from filter.ml import filter_image

if __name__ == "__main__":
    start_time = time.process_time()
    filter_image("../media/image-input-large.jpg", 3)
    end_time = time.process_time()
    print("Execution time:", end_time - start_time)
