#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }
    pub fn crowned(p: GamePiece) -> GamePiece {
        GamePiece{
            color: p.color,
            crowned: true,
        }
    }
}
