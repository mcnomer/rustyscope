from matplotlib import pyplot as plt
from rustyscope import AFPFile

file_path = r"good_test_file.001"
afp_file = AFPFile(file_path)

print(f"AFP image was scanned: {afp_file.file_metadata["date"]}")

for channel in afp_file.channels:
    print(f"Found channel: {channel.name}")

for x, height in afp_file.data:
    plt.scatter(x, height)
plt.show()
