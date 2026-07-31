# rustyscope

A small library for parsing .001 files from 5th Generation Bruker AFPs. Written in Rust.

## Installation

```bash
$ pip install maturin
$ pip install --index-url https://test.pypi.org/simple/ rustyscope
```

## Example Code

```python

from matplotlib import pyplot as plt
from rustyscope import AFPFile

file_path = r"good_test_file.001"
afp_file = AFPFile(file_path)

print(f"AFP image was scanned: {afp_file.file_metadata["date"]}")

for x, height in afp_file.data:
    plt.scatter(x, height)
plt.show()

```

## Credits

This project was made into a python package using [maturin](https://github.com/PyO3/maturin).

Thank you to Shea McLaughlin for help with file parsing.

[![Developed by a Human, Not by AI](https://notbyai.fyi/img/developed-by-human-not-by-ai-white.svg)](https://notbyai.fyi/)
