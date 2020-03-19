import math
from typing import List 


ERROR = 0.00000001


def is_valid_probability(x: float) -> bool:
    return 0 - ERROR <= x <= 1 + ERROR


def is_valid_distribution(p: List[float]) -> bool:
    return all(map(is_valid_probability, p)) and abs(sum(p) - 1) < ERROR


def total_probability(x_given_y: List[float], y: List[float]) -> float:
    return sum(map(lambda x_given_y, y: x_given_y * y, x_given_y, y))


def bayesian_probability(y: float, x_given_y, x) -> float:
    return y * x_given_y / x


def self_information(p: float, base: int=2) -> float:
    if p == 0:
        return float("inf")
    return -math.log(p, base)


def mutual_information(x: float, x_given_y: float, base: int=2) -> float:
    return math.log(x_given_y / x, base)


def entropy(p: List[float], base: int=2) -> float:
    p_non_zero = filter(lambda p: p != 0, p)
    return sum(map(lambda p: p * self_information(p, base), p_non_zero))


def var(p: List[float], H: float) -> float:
    return sum(map(lambda p: p * (self_information(p) - H)**2, p))


if __name__ == "__main__":
    # priori = [1 / 8, 1 / 4, 1 / 8, 1 / 4] + [1 / 16 for _ in range(4)]
    # condition = [0, 0, 1, 1] + [0 for _ in range(4)]

    # x_priori = total_probability(condition, priori)
    # y_given_x = bayesian_probability(1 / 4, 1, x_priori)
    # print(y_given_x, 2 / 3)

    # print(self_information(6 / 36))
    # print(self_information(1 / 36))

    # X = {i: 1 / 6 for i in range(1, 7)}
    # Y = {
    #     2: 1 / 36, 3: 2 / 36, 4: 3 / 36, 5: 4 / 36, 6: 5 / 36, 7: 6 / 36, 8: 5 / 36,
    #     9: 4 / 36, 10: 3 / 36, 11: 2 / 36, 12: 1 / 36,
    # }
    # Z = [
    #     1 / 216, 3 / 216, 6 / 216, 10 / 216, 15 / 216, 21 / 216, 25 / 216, 27 / 216,
    #     27 / 216, 25 / 216, 21 / 216, 15 / 216, 10 / 216, 6 / 216, 3 / 216, 1 / 216,
    # ]

    # assert is_valid_distribution(X.values())
    # assert is_valid_distribution(Y.values())
    # assert is_valid_distribution(Z)

    # H_X = entropy(X.values())
    # H_Y = entropy(Y.values())
    # H_Z = entropy(Z)

    # print(H_Z - H_X, H_Z - H_Y, H_Y - H_X)

    H_U = entropy([1 / 4, 3 / 4])
    sigma_squared = var([1 / 4, 3 / 4], H_U)
    print(H_U, sigma_squared)

    epsilon = 10**(-8) # and 0.1
    delta = 10**(-3) # and 0.05

    L_0 = math.ceil(sigma_squared / (epsilon * delta**2 ))
    print(L_0)
    upper_bound = L_0 * (H_U + delta)
    lower_bound = L_0 * (H_U - delta)
    print(upper_bound)
    print(lower_bound)
