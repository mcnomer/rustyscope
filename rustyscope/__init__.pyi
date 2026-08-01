from typing import List, Tuple
import numpy as np
import numpy.typing as npt

LineScan = Tuple[npt.NDArray[np.float64], npt.NDArray[np.float64]]

class AFPFile:
    """
    An interface for the parsed AFP file

    Attributes:
        file_path (str): Path to the file
        data (list of LineScan): Data formatted as a list of linescans - (x, height) numpy arrays
        file_metadata (Metadata):
        scanner_metadata (Metadata):
        get_equipment_metadata (Metadata or None):
        get_hdsc_metadata (Metadata or None):
        get_misc_metadata (Metadata or None):
        get_engage_metadata (Metadata or None):
        get_sweep_metadata (Metadata or None):
    """

    file_path: str

    def __init__(self, file_path: str) -> None:
        """
        Load an nanoscope AFP file

        Args:
            file_path (str): Path to the file

        Returns:
            AFPFile: Parsed representation of the given file


        """
        ...

    @property
    def data(self) -> List[LineScan]: ...
    @property
    def channels(self) -> List[Channel]: ...
    @property
    def file_metadata(self) -> Metadata: ...
    @property
    def scanner_metadata(self) -> Metadata: ...
    @property
    def equipment_metadata(self) -> Metadata | None: ...
    @property
    def hdsc_metadata(self) -> Metadata | None: ...
    @property
    def misc_metadata(self) -> Metadata | None: ...
    @property
    def engage_metadata(self) -> Metadata | None: ...
    @property
    def sweep_metadata(self) -> Metadata | None: ...

class Metadata(dict[str, int | float | str]):
    """Dictionary parsed from a section of the AFP file's header"""

    pass

class Channel:
    """
    AFP data channel

    Attributes:
        name (str): Channel name
        data (list of int): Channel data
    """

    name: str
    data: List[int]

    @property
    def metadata(self) -> Metadata: ...
