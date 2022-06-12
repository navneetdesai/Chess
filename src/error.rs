pub enum Error {
    InvalidMove(String),
    InvalidDestination(String),
    InvalidSource(String),
    KingUnderCheck(String),
    Checkmate(String),
    InvalidPromotion(String),
    GameOver(String),
    DrawOffer(String),
    DrawRejected,
    Dummy,
}
