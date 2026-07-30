from matplotlib import pyplot as plt
import rustyscope

file_path = r"good_test_file.001"

data = rustyscope.load(file_path)
for x, height in data:
    plt.scatter(x, height)
plt.show()
