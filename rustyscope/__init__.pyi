from typing import List, Tuple
import numpy as np
import numpy.typing as npt

LineScan = Tuple[npt.NDArray[np.float64], npt.NDArray[np.float64]]

def load(file_path: str) -> List[LineScan]:
    """Loads a nanoscope AFP file as linescans

    Returns:
        List of (x_array, height_array) tuples with float64 numpy arrays.
    """
    ...
