use fastmetrics_derive::StateSetValue;

#[derive(PartialEq, StateSetValue)]
enum ServiceState {
    Available,
    Degraded,
    Down,
}

fn main() {
    // This just verifies compilation succeeds
    let _state = ServiceState::Available;
}
