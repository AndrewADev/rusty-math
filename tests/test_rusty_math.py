import math
import pytest
from rusty_math import Vec3


def test_dot_perpendicular():
    assert Vec3(1, 0, 0).dot(Vec3(0, 1, 0)) == 0.0

def test_dot_parallel():
    assert Vec3(1, 0, 0).dot(Vec3(1, 0, 0)) == 1.0

def test_cross_unit_axes():
    result = Vec3(1, 0, 0).cross(Vec3(0, 1, 0))
    assert (result.x, result.y, result.z) == (0.0, 0.0, 1.0)

def test_cross_anticommutative():
    a, b = Vec3(1, 2, 3), Vec3(4, 5, 6)
    ab = a.cross(b)
    ba = b.cross(a)
    assert (ab.x, ab.y, ab.z) == (-ba.x, -ba.y, -ba.z)

def test_magnitude():
    assert Vec3(3, 4, 0).magnitude() == 5.0

def test_normalize_unit_length():
    n = Vec3(3, 4, 0).normalize()
    assert math.isclose(n.magnitude(), 1.0)

def test_normalize_zero_raises():
    with pytest.raises(ValueError, match="zero vector"):
        Vec3(0, 0, 0).normalize()

def test_angle_perpendicular():
    assert math.isclose(Vec3(1, 0, 0).angle(Vec3(0, 1, 0)), math.pi / 2)

def test_angle_parallel():
    assert math.isclose(Vec3(1, 0, 0).angle(Vec3(1, 0, 0)), 0.0)

def test_angle_zero_raises():
    with pytest.raises(ValueError, match="zero vector"):
        Vec3(0, 0, 0).angle(Vec3(1, 0, 0))

def test_add():
    r = Vec3(1, 2, 3) + Vec3(4, 5, 6)
    assert (r.x, r.y, r.z) == (5.0, 7.0, 9.0)

def test_sub():
    r = Vec3(4, 5, 6) - Vec3(1, 2, 3)
    assert (r.x, r.y, r.z) == (3.0, 3.0, 3.0)

def test_mul_scalar():
    r = Vec3(1, 2, 3) * 2
    assert (r.x, r.y, r.z) == (2.0, 4.0, 6.0)
