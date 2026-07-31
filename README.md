# rustyscope

A small library for parsing .001 files from 5th Generation Bruker AFPs. Written in Rust.

## Installation

```bash
$ pip install --index-url https://test.pypi.org/simple/ rustyscope
```

## Example Code

```python

from matplotlib import pyplot as plt

import rustyscope



file_path = "example.001"



data = rustyscope.load(file_path)

for x, height in data:

plt.scatter(x, height)

plt.show()

```

## Credits

This project was made into a python package using [maturin](https://github.com/PyO3/maturin).

Thank you to Shea McLaughlin for help with file parsing.

[![Developed by a Human, Not by AI](https://notbyai.fyi/img/developed-by-human-not-by-ai-white.svg)](https://notbyai.fyi/)
