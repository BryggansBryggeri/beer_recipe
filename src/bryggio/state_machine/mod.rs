//! State machine for the brewing process
//!
//! Heavily influenced by (https://hoverbear.org/blog/rust-state-machine-pattern/)
//!
//! TODO: Rename this module and place it somewhere better.
//! Perhaps a as a client in bryggio-lib
use crate::bryggio::recipe::Recipe as BryggioRecipe;

// TODO: Rename
enum StateMachineWrapper {
    Planning(StateMachine<Planning>),
    Preparation,
    Mash,
    MashOut,
    Sparge,
    Boil,
    FirstChill,
    Whirlpool,
    FinalChill,
    Fermentation,
    Done(StateMachine<Done>),
}

impl StateMachineWrapper {
    fn step(mut self) -> Self {
        match self {
            StateMachineWrapper::Planning(val) => StateMachineWrapper::Done(val.into()),
            StateMachineWrapper::Done(_val) => panic!("Can't step from state 'Done'"),
            _ => todo!(),
        }
    }
}

// TODO: Rename state machine
struct StateMachine<S> {
    general: BryggioRecipe,
    state: S,
}

impl From<BryggioRecipe> for StateMachine<Planning> {
    fn from(recipe: BryggioRecipe) -> StateMachine<Planning> {
        StateMachine {
            general: recipe,
            state: Planning { temp: 0 },
        }
    }
}

struct Planning {
    temp: usize,
}

impl From<BryggioRecipe> for Planning {
    fn from(_recipe: BryggioRecipe) -> Planning {
        Planning { temp: 0 }
    }
}

struct Done {
    temp: usize,
}
impl From<StateMachine<Planning>> for StateMachine<Done> {
    fn from(val: StateMachine<Planning>) -> StateMachine<Done> {
        StateMachine {
            general: val.general,
            state: Done {
                temp: 2 * val.state.temp,
            },
        }
    }
}
