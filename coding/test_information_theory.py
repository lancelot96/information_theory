from information_theory import mutual_information, self_information, entropy


def test_mutual_information():
    assert mutual_information(1 / 8, 1 / 4) == 1.0


def test_self_information():
    assert self_information(0.5) == 1.0
    assert self_information(0) == float("inf")


def test_entropy():
    assert entropy([0]) == 0
    assert entropy([1 / 8 for _ in range(8)]) == 3.0
    assert entropy([1 / 8, 1 / 8, 1 / 4, 1 / 4] + [1 / 16 for _ in range(4)]) == 2.75
