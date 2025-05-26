use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum TimerType {
    CountDownChallenge,
    WaitTime,
    GameStartCountDown,
    PublicTime,
    BehaviorTreeTimer1,
    BehaviorTreeTimer2,
    BehaviorTreeTimer3,
}