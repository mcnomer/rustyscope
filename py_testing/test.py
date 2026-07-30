from matplotlib import pyplot as plt
import rustyscope

file_path = r"C:\Users\omerk\Documents\GitHub\mINE\rust\2026\nanoscope\QCJCD_W3_SiteSite1_Die_X0_Die_Y-4_23_A_22_20260708_010638.001"
# file_path = r"C:\Users\omerk\Documents\GitHub\mINE\rust\2026\nanoscope\QCJCD_W3_SiteSite1_Die_X0_Die_Y-4_23_A_22_20260708_010638 copy.001"

data = rustyscope.load(file_path)
for x, height in data:
    plt.scatter(x, height)
plt.show()
