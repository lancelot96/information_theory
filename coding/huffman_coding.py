import math
from queue import Queue
from typing import List, Dict

from information_theory import entropy
from calculate_distribution import distribution_from_text
from helper import TreeNode, PriorityQueue, Coding, average_coding_length


def first_to_merge(alphabet_size, base) -> int:
    return (alphabet_size - 2) % (base - 1) + 2


def huffman_coding(distribution: Dict[str, Coding], base: int=2):
    assert len(distribution) > 1

    nodes = list(map(
        lambda label, c: TreeNode(label, c.p),
        distribution.keys(), distribution.values(),
    ))
    p_queue = PriorityQueue(nodes)

    while len(p_queue) != 1:
        new_node = TreeNode()
        for _ in range(first_to_merge(len(distribution), base) \
            if len(p_queue) == len(distribution) else base):
            node = p_queue.dequeue()
            new_node.add_child(node)
        p_queue.enqueue(new_node)

    tree_root = p_queue.dequeue()
    print(tree_root)

    depth = 0
    queue = Queue()
    queue.put(tree_root)

    i = 0
    parent = None
    while not queue.empty():
        width = queue.qsize()

        for _ in range(width):
            node = queue.get()

            if node.parent != parent:
                parent = node.parent
                i = 0
            if node.parent != None:
                node.coding = node.parent.coding + str(i)
                i += 1

            if not node.children:
                distribution[node.label].length = depth
                distribution[node.label].coding = node.coding
                continue

            for child in node.children:
                queue.put(child)
        depth += 1


if __name__ == "__main__":
    # file = open("coding/alice in wonderland.txt", "r", encoding="utf8")
    # distribution = distribution_from_text(file.read(), ignore_case=True, ignore_blank=True)

    # distribution = {
    #     "a": Coding(9/16), "b": Coding(3/16), "c": Coding(3/16), "d": Coding(1/16),
    # }

    # distribution = {
    #     "a": Coding(0.2), "b": Coding(0.03), "c": Coding(0.04), "d": Coding(0.16),
    #     "e": Coding(0.05), "f": Coding(0.1), "g": Coding(0.12), "h": Coding(0.3),
    # }

    # distribution = {
    #     "a_1": Coding(0.16), "a_2": Coding(0.14), "a_3": Coding(0.13), "a_4": Coding(0.12),
    #     "a_5": Coding(0.10), "a_6": Coding(0.09), "a_7": Coding(0.08), "a_8": Coding(0.07),
    #     "a_9": Coding(0.06), "a_10": Coding(0.05),
    # }

    # distribution = {
    #     "a_1": Coding(0.5), "a_2": Coding(0.3), "a_3": Coding(0.2),
    # }

    distribution = {
        "a_11": Coding(0.25), "a_12": Coding(0.15), "a_13": Coding(0.1),
        "a_21": Coding(0.15), "a_22": Coding(0.09), "a_23": Coding(0.06),
        "a_31": Coding(0.1), "a_32": Coding(0.06), "a_33": Coding(0.04),
    }

    distribution = {
        "a_111": Coding(0.125), "a_112": Coding(0.075), "a_113": Coding(0.05),
        "a_121": Coding(0.075), "a_122": Coding(0.045), "a_123": Coding(0.03),
        "a_131": Coding(0.05), "a_132": Coding(0.03), "a_133": Coding(0.02),
        "a_211": Coding(0.075), "a_212": Coding(0.045), "a_213": Coding(0.03),
        "a_221": Coding(0.045), "a_222": Coding(0.027), "a_223": Coding(0.018),
        "a_231": Coding(0.03), "a_232": Coding(0.018), "a_233": Coding(0.012),
        "a_311": Coding(0.05), "a_312": Coding(0.03), "a_313": Coding(0.02),
        "a_321": Coding(0.03), "a_322": Coding(0.018), "a_323": Coding(0.012),
        "a_331": Coding(0.02), "a_332": Coding(0.012), "a_333": Coding(0.008),
    }

    prob = sum(map(lambda c: c.p, distribution.values()))
    print(prob)

    base = 2

    huffman_coding(distribution, base)
    print(distribution)
    print(len(distribution))

    lower_bound = entropy(map(lambda c: c.p, distribution.values()), base)
    length = average_coding_length(distribution)

    print(lower_bound)
    print(length)
    print(lower_bound / length)
