import re


class Block:
    MATCHER = re.compile(r"^$")

    @classmethod
    def match(cls, line):
        if match := cls.MATCHER.match(line):
            return cls(*match.groups())


class BinaryBlock(Block):
    def __init__(self, left, right, output):
        self.left = left
        self.right = right
        self.output = output
        self._value: int | None = None

    def to_graphviz(self):
        return [(self.left, self.output), (self.right, self.output)]

    def op(self, l: int, r: int) -> int:
        raise NotImplementedError

    def apply(self, graph) -> int:
        if self._value is None:
            if self.left.isdigit():
                l = int(self.left)
            else:
                l = graph[self.left].apply(graph)

            if self.right.isdigit():
                r = int(self.right)
            else:
                r = graph[self.right].apply(graph)

            self._value = self.op(l, r)
        return self._value


class And(BinaryBlock):
    MATCHER = re.compile(r"([a-z]+|\d+) AND ([a-z]+|\d+) -> ([a-z]+)")

    def to_graphviz(self):
        return [(self.left, self.output), (self.right, self.output)]

    def op(self, l, r):
        return l & r


class Or(BinaryBlock):
    MATCHER = re.compile(r"([a-z]+|\d+) OR ([a-z]+|\d+) -> ([a-z]+)")

    def op(self, l, r):
        return l | r


class UnaryBlock(Block):
    def __init__(self, input, output):
        self.input = input
        self.output = output
        self._value: int | None = None

    def to_graphviz(self):
        return [(self.input, self.output)]

    def op(self, n: int) -> int:
        raise NotImplementedError

    def apply(self, graph) -> int:
        if not hasattr(self, "_value"):
            import pdb; pdb.set_trace()
            raise AttributeError
        if self._value is None:
            if self.input.isdigit():
                self._value = self.op(int(self.input))
            else:
                self._value = self.op(graph[self.input].apply(graph))

        return self._value


class Pass(UnaryBlock):
    MATCHER = re.compile(r"([a-z]+|\d+) -> ([a-z]+)")

    def op(self, n: int) -> int:
        return n


class Not(UnaryBlock):
    MATCHER = re.compile(r"NOT ([a-z]+|\d+) -> ([a-z]+)")

    def op(self, n: int) -> int:
        return n ^ 65535


class LeftShift(BinaryBlock):
    MATCHER = re.compile(r"([a-z]+|\d+) LSHIFT (\d+) -> ([a-z]+)")

    def op(self, l, r):
        return l << r


class RightShift(BinaryBlock):
    MATCHER = re.compile(r"([a-z]+|\d+) RSHIFT (\d+) -> ([a-z]+)")

    def op(self, l, r):
        return l >> r


def parse_input(infile_name):
    with open(infile_name) as fh:
        for line in fh.readlines():
            for block_class in [And, Or, Not, LeftShift, RightShift, Pass]:
                if block := block_class.match(line):
                    yield block


def part1(graph):
    return graph["a"].apply(graph)


def part2(graph):
    b_input = part1(graph)

    # Reset memoized values in graph
    for _, v in graph.items():
        v._value = None

    graph["b"] = Pass(str(b_input), "b")

    return part1(graph)


def graphviz(input):
    print("strict digraph {")
    for block in input:
        for f, t in block.to_graphviz():
            print(f"  {f} -> {t}")
    print("}")


def main():
    input = list(parse_input("input.txt"))

    graph = {}

    for edge in input:
        graph[edge.output] = edge

    print("Part I:  ", part1(graph))
    print("Part II: ", part2(graph))


if __name__ == "__main__":
    main()
