pub enum Error {
    InvalidMove(String),
    InvalidDestination(String),
    InvalidSource(String),
    KingUnderCheck(String),
    Checkmate(String),
    InvalidPromotion(String),
    Dummy
}