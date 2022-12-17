use std::time::SystemTime;

use color_eyre::Result;
use context_attribute::context;
use framework::MainOutput;
use spl_network_messages::GameControllerStateMessage;
use types::{CycleTime, GameControllerState, SensorData};

pub struct GameControllerFilter {
    game_controller_state: Option<GameControllerState>,
    last_game_state_change: Option<SystemTime>,
}

#[context]
pub struct CreationContext {}

#[context]
pub struct CycleContext {
    pub sensor_data: Input<SensorData, "sensor_data">,
    pub cycle_time: Input<CycleTime, "cycle_time">,
    // TODO:
    // pub game_controller_state_message:
    //     PerceptionInput<GameControllerStateMessage, "SplNetwork", "game_controller_state_message">,
}

#[context]
#[derive(Default)]
pub struct MainOutputs {
    pub game_controller_state: MainOutput<Option<GameControllerState>>,
}

impl GameControllerFilter {
    pub fn new(_context: CreationContext) -> Result<Self> {
        Ok(Self {
            game_controller_state: None,
            last_game_state_change: None,
        })
    }

    pub fn cycle(&mut self, context: CycleContext) -> Result<MainOutputs> {
        // TODO:
        // for game_controller_state_message in context
        //     .game_controller_state_message
        //     .persistent
        //     .values()
        //     .flatten()
        for game_controller_state_message in &Vec::<GameControllerStateMessage>::new() {
            let game_state_changed = match &self.game_controller_state {
                Some(game_controller_state) => {
                    game_controller_state.game_state != game_controller_state_message.game_state
                }
                None => true,
            };
            if game_state_changed {
                self.last_game_state_change = Some(context.cycle_time.start_time);
            }
            self.game_controller_state = Some(GameControllerState {
                game_state: game_controller_state_message.game_state,
                game_phase: game_controller_state_message.game_phase,
                kicking_team: game_controller_state_message.kicking_team,
                last_game_state_change: self.last_game_state_change.unwrap(),
                penalties: game_controller_state_message.hulks_team.clone().into(),
                remaining_amount_of_messages: game_controller_state_message
                    .hulks_team
                    .remaining_amount_of_messages,
                set_play: game_controller_state_message.set_play,
            });
        }
        Ok(MainOutputs {
            game_controller_state: self.game_controller_state.into(),
        })
    }
}
