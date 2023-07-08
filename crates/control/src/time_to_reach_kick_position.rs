use std::{f32::consts::PI, time::Duration};

use color_eyre::Result;
use framework::AdditionalOutput;
use linear_algebra::Vector2;
use serde::{Deserialize, Serialize};
use types::{parameters::BehaviorParameters, planned_path::PathSegment};

#[derive(Deserialize, Serialize)]
pub struct TimeToReachKickPosition {}

use context_attribute::context;
#[context]
pub struct CycleContext {
    dribble_path: Input<Option<Vec<PathSegment>>, "dribble_path?">,

    time_to_reach_kick_position_output:
        AdditionalOutput<Option<Duration>, "time_to_reach_kick_position_output">,

    time_to_reach_kick_position: CyclerState<Duration, "time_to_reach_kick_position">,

    configuration: Parameter<BehaviorParameters, "behavior">,

    stand_up_back_estimated_remaining_duration:
        Input<Option<Duration>, "stand_up_back_estimated_remaining_duration?">,
    stand_up_front_estimated_remaining_duration:
        Input<Option<Duration>, "stand_up_front_estimated_remaining_duration?">,
}

#[context]
pub struct CreationContext {}

#[context]
pub struct MainOutputs {}

impl TimeToReachKickPosition {
    pub fn new(_: CreationContext) -> Result<Self> {
        Ok(Self {})
    }

    pub fn cycle(&mut self, mut context: CycleContext) -> Result<MainOutputs> {
        let walk_time = context
            .dribble_path
            .as_ref()
            .map(|path| {
                path.iter()
                    .map(|segment: &PathSegment| {
                        let length = segment.length();
                        match segment {
                            PathSegment::LineSegment(_) => {
                                length / context.configuration.path_planning.line_walking_speed
                            }
                            PathSegment::Arc(_, _) => {
                                length / context.configuration.path_planning.arc_walking_speed
                            }
                        }
                    })
                    .sum()
            })
            .map(Duration::from_secs_f32);
        let first_segment_angle = match context.dribble_path {
            Some(path) => match path.first() {
                Some(PathSegment::LineSegment(linesegment)) => {
                    Some(linesegment.1.coords().angle(Vector2::x_axis()).abs())
                }
                _ => None,
            },
            _ => None,
        };
        let time_to_turn = Duration::from_secs_f32(match first_segment_angle {
            Some(angle) => angle / PI * context.configuration.path_planning.half_turning_time,
            None => 0.0f32,
        });
        let time_to_reach_kick_position = walk_time.map(|walk_time| {
            [
                walk_time,
                *context
                    .stand_up_back_estimated_remaining_duration
                    .unwrap_or(&Duration::ZERO),
                *context
                    .stand_up_front_estimated_remaining_duration
                    .unwrap_or(&Duration::ZERO),
                time_to_turn,
            ]
            .into_iter()
            .fold(Duration::ZERO, Duration::saturating_add)
        });

        context
            .time_to_reach_kick_position_output
            .fill_if_subscribed(|| time_to_reach_kick_position);
        // 1800 seconds is 30 minutes, which is essentially maximum as it pertains to game time.
        // Prevents Duration::MAX from breaking the behavior simulator.
        *context.time_to_reach_kick_position = time_to_reach_kick_position
            .unwrap_or(Duration::MAX)
            .min(Duration::from_secs(1800));

        Ok(MainOutputs {})
    }
}
