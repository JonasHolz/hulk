from dataclasses import dataclass
from enum import Enum
from typing import Self

from transforms import Pose2


@dataclass
class Parameters:
    sole_pressure_threshold: float
    walk_height: float
    min_step_duration: float
    step_duration: float
    foot_lift_apex: float
    foot_offset_left: float
    foot_offset_right: float


class Side(Enum):
    LEFT = 0
    RIGHT = 1


@dataclass
class Feet:
    support_sole: Pose2
    swing_sole: Pose2

    @staticmethod
    def from_support_side(
        left_sole: Pose2,
        right_sole: Pose2,
        support_side: Side,
    ) -> Self:
        if support_side == Side.LEFT:
            return Feet(support_sole=left_sole, swing_sole=right_sole)
        return Feet(support_sole=right_sole, swing_sole=left_sole)

    def switch(self) -> Self:
        return Feet(support_sole=self.swing_sole, swing_sole=self.support_sole)


@dataclass
class State:
    t: float
    support_side: Side
    start_feet: Feet
    end_feet: Feet


@dataclass
class Measurements:
    pressure_left: float
    pressure_right: float


@dataclass
class Control:
    forward: float
    left: float
    turn: float
