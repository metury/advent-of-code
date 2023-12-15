#!/usr/bin/env python3

from enum import Enum

EQUALS = "="
DASH = "-"

MULTIPLIER = 17
DIVISOR = 256


class OperationType(Enum):
    ADD = "add"
    REMOVE = "remove"


class Box:
    def __init__(self):
        self.content = {}

    def add_lens(self, label: str, focal_length: int):
        self.content[label] = focal_length

    def remove_lens(self, label: str):
        self.content.pop(label, None)

    def box_power(self) -> int:
        return sum(
            slot * focal_length
            for slot, (_, focal_length) in enumerate(self.content.items(), 1)
        )


class Operation:
    def __init__(self, op: Enum, label: str, focal_length=None):
        self.op = op
        self.label = label
        self.hash = Operation.hash_string(label)
        self.focal_length = focal_length

    @staticmethod
    def hash_string(string: str) -> int:
        value = 0
        for char in string:
            value = (value + ord(char)) * MULTIPLIER % DIVISOR
        return value


class ReflectorDish:
    def __init__(self):
        self.sequence: list[Operation] = []
        self.boxes = [Box() for _ in range(DIVISOR)]

    @staticmethod
    def parse_step(string: str) -> Operation:
        if EQUALS in string:
            label, focal_length_str = string.split(EQUALS)
            return Operation(OperationType.ADD, label, int(focal_length_str))
        elif DASH in string:
            label = string.split(DASH)[0]
            return Operation(OperationType.REMOVE, label)

    def parse_input(self, input_data: list):
        for string in input_data:
            self.sequence.append(ReflectorDish.parse_step(string))

    def initialize_sequence(self):
        for operation in self.sequence:
            if operation.op == OperationType.ADD:
                self.boxes[operation.hash].add_lens(
                    operation.label, operation.focal_length
                )
            else:
                self.boxes[operation.hash].remove_lens(operation.label)

    def calculate_focusing_power(self) -> int:
        for i, box in enumerate(self.boxes, 1):
            print(i * box.box_power())
        return sum(i * box.box_power() for i, box in enumerate(self.boxes, 1))


def part_one(data: list) -> int:
    return sum(Operation.hash_string(string) for string in data)


def main():
    with open("INPUT") as f:
        input_data = f.read().split(",")

    print(part_one(input_data))

    reflector_dish = ReflectorDish()
    reflector_dish.parse_input(input_data)
    reflector_dish.initialize_sequence()
    print(reflector_dish.calculate_focusing_power())


if __name__ == "__main__":
    main()

