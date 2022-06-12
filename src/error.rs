pub enum GameState {
    InvalidMove(String),
    InvalidDestination(String),
    InvalidSource(String),
    KingUnderCheck(String),
    InvalidPromotion(String),
    GameOver(String),
    DrawOffer(String),
    DrawRejected,
    Resignation,
    OK,
}
