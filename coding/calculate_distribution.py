from typing import Dict
from collections import defaultdict

from helper import Coding


def distribution_from_text(text: str) -> Dict[str, Coding]:
    alphabet = defaultdict(Coding)

    for c in text:
        alphabet[c].p += 1

    chars_count = len(text)
    for c in alphabet:
        alphabet[c].p /= chars_count

    return alphabet


if __name__ == "__main__":
    with open("coding/test.txt", "r", encoding="utf8") as file:
        distribution = distribution_from_text(file.read())
        print(distribution)
