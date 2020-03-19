from typing import Dict
from collections import defaultdict

from helper import Coding


def distribution_from_text(text: str, ignore_case=False, ignore_blank=False) -> Dict[str, Coding]:
    alphabet = defaultdict(Coding)

    for c in text:
        if not ignore_blank or c.isalpha() or c.isnumeric():
            alphabet[c.upper() if ignore_case else c].p += 1

    chars_count = len(text)
    if ignore_blank:
        chars_count = sum(map(lambda c: c.p, alphabet.values()))
    for c in alphabet:
        alphabet[c].p /= chars_count

    return alphabet


if __name__ == "__main__":
    with open("coding/test.txt", "r", encoding="utf8") as file:
        distribution = distribution_from_text(file.read())
        print(distribution)
