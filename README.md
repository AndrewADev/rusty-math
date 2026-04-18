# rusty-math

Experimenting with a Python package written in Rust.

## Prerequisites

- [Rust](https://rustup.rs/)
- [uv](https://docs.astral.sh/uv/)

## Development

Build and run tests:

```bash
task build
task test
```

## Installation

Find the wheel URL for your platform and Python version on the [releases page](https://github.com/AndrewADev/rusty-math/releases/latest), then:

```bash
uv add "rusty-math @ https://github.com/AndrewADev/rusty-math/releases/download/v0.1.0/rusty_math-0.1.0-cp312-cp312-macosx_11_0_arm64.whl"
```

## Usage

```python
from rusty_math import Vec3

a = Vec3(1, 0, 0)
b = Vec3(0, 1, 0)

# Dot and cross product
a.dot(b)    # 0.0
a.cross(b)  # Vec3(0, 0, 1)

# Magnitude and normalization
Vec3(3, 4, 0).magnitude()   # 5.0
Vec3(3, 4, 0).normalize()   # Vec3(0.6, 0.8, 0.0)

# Angle between vectors (radians)
a.angle(b)  # 1.5707963267948966  (π/2)

# Arithmetic
a + b      # Vec3(1, 1, 0)
a - b      # Vec3(1, -1, 0)
a * 3      # Vec3(3, 0, 0)
```
