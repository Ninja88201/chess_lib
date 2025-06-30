use crate::CastlingRights;


#[test]
fn insert() {
    let mut rights = CastlingRights::NONE;
    rights.insert(CastlingRights::WHITE_KINGSIDE);
    assert!(rights.contains(CastlingRights::WHITE_KINGSIDE));
    assert!(!rights.contains(CastlingRights::WHITE_QUEENSIDE));

    rights.insert(CastlingRights::WHITE_QUEENSIDE);
    assert!(rights.contains(CastlingRights::WHITE_QUEENSIDE));

    rights.remove(CastlingRights::WHITE_KINGSIDE);
    assert!(!rights.contains(CastlingRights::WHITE_KINGSIDE));
    assert!(rights.contains(CastlingRights::WHITE_QUEENSIDE));
}
#[test]
fn remove() {
    let mut rights = CastlingRights::ALL;

    rights.remove(CastlingRights::WHITE_KINGSIDE);
    assert!(!rights.contains(CastlingRights::WHITE_KINGSIDE));
    assert!(rights.contains(CastlingRights::WHITE_QUEENSIDE));
    assert!(rights.contains(CastlingRights::BLACK_KINGSIDE));
    assert!(rights.contains(CastlingRights::BLACK_QUEENSIDE));

    rights.remove(CastlingRights::WHITE_QUEENSIDE);
    assert!(!rights.contains(CastlingRights::WHITE_QUEENSIDE));
    assert!(rights.contains(CastlingRights::BLACK_KINGSIDE));
    assert!(rights.contains(CastlingRights::BLACK_QUEENSIDE));

    rights.remove(CastlingRights::BLACK_KINGSIDE);
    assert!(!rights.contains(CastlingRights::BLACK_KINGSIDE));
    assert!(rights.contains(CastlingRights::BLACK_QUEENSIDE));

    rights.remove(CastlingRights::BLACK_QUEENSIDE);
    assert!(!rights.contains(CastlingRights::BLACK_QUEENSIDE));
}

#[test]
fn display() {
    assert_eq!(CastlingRights::WHITE_KINGSIDE.to_fen(), "K");
    assert_eq!(CastlingRights::BLACK_QUEENSIDE.to_fen(), "q");
    assert_eq!(
        (CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_KINGSIDE).to_fen(),
        "Kk"
    );
}
