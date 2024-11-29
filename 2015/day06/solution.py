import re
from typing import Sequence


LINE_MATCH = re.compile(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")


class Instruction:
    def __init__(self, top, left, bottom, right):
        self.top = int(top)
        self.left = int(left)
        self.bottom = int(bottom)
        self.right = int(right)

    def operation(self, existing) -> int:
        raise NotImplemented

    def operation2(self, existing) -> int:
        raise NotImplemented

    def apply(self, grid):
        for x in range(self.top, self.bottom + 1):
            for y in range(self.left, self.right + 1):
                grid[x][y] = self.operation(grid[x][y])

    def apply2(self, grid):
        for x in range(self.top, self.bottom + 1):
            for y in range(self.left, self.right + 1):
                grid[x][y] = self.operation2(grid[x][y])


class TurnOn(Instruction):
    def operation(self, existing):
        return 1

    def operation2(self, existing):
        return existing + 1


class TurnOff(Instruction):
    def operation(self, existing):
        return 0

    def operation2(self, existing):
        return max(existing - 1, 0)


class Toggle(Instruction):
    def operation(self, existing):
        return existing * -1 + 1

    def operation2(self, existing):
        return existing + 2


def parse_input(infile_name):
    with open(infile_name) as fh:
        for line in fh.readlines():
            line_match = LINE_MATCH.match(line)
            assert line_match

            (operation, top, left, bottom, right) = line_match.groups()

            match operation:
                case "turn on":
                    operation_class = TurnOn
                case "turn off":
                    operation_class = TurnOff
                case _:
                    operation_class = Toggle

            yield operation_class(top, left, bottom, right)


def part1(input: Sequence[Instruction], width, height):
    grid = []

    for _ in range(0, height):
        grid.append([0] * width)

    for instruction in input:
        instruction.apply(grid)

    count = 0

    for row in grid:
        count += sum(row)

    return count


def part2(input: Sequence[Instruction], width, height):
    grid = []

    for _ in range(0, height):
        grid.append([0] * width)

    for instruction in input:
        instruction.apply2(grid)

    count = 0

    for row in grid:
        count += sum(row)

    return count



def main():
    input = list(parse_input("input.txt"))
    print("Part I:  ", part1(input, 1000, 1000))
    print("Part II: ", part2(input, 1000, 1000))


if __name__ == "__main__":
    main()
