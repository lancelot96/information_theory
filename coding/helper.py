import heapq

from typing import List, Dict


class PriorityQueue:
    def __init__(self, container: List=None):
        if container:
            heapq.heapify(container)
        self.heap = container or []

    def __len__(self) -> int:
        return len(self.heap)

    def __repr__(self) -> str:
        return str(self.heap)

    def enqueue(self, p: float) -> None:
        heapq.heappush(self.heap, p)

    def dequeue(self) -> float:
        return heapq.heappop(self.heap)


class TreeNode:
    def __init__(self, label=None, p: float=0, parent=None, children=None):
        self.label = label or ""
        self.p = p
        self.coding = ""
        self.parent = parent
        self.children = children or []

    def __len__(self) -> int:
        return len(self.children)

    def __lt__(self, o) -> bool:
        return self.p < o.p

    def __repr__(self) -> str:
        return f"('{self.label}': {self.p}, {self.children})"

    def degree(self) -> int:
        return len(self)

    def add_child(self, node):
        self.p += node.p
        node.parent = self
        self.children.append(node)


class Coding:
    def __init__(self, p: float=0, length: int=0, coding: str=""):
        self.p = p
        self.length = length
        self.coding = coding

    def __repr__(self) -> str:
        return f"{self.p, self.length, self.coding}"


def average_coding_length(distribution: Dict[str, Coding]) -> float:
    return sum(map(lambda c: c.p * c.length, distribution.values()))
