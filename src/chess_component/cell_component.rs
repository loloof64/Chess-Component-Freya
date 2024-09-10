use dioxus_core::AttributeValue;
use freya::prelude::*;
use owlchess::{ Board, Cell as owlchessCell, Color, File, Piece, Rank };

static WP: &[u8] = include_bytes!("./vectors/Chess_plt45.svg");
static WN: &[u8] = include_bytes!("./vectors/Chess_nlt45.svg");
static WB: &[u8] = include_bytes!("./vectors/Chess_blt45.svg");
static WR: &[u8] = include_bytes!("./vectors/Chess_rlt45.svg");
static WQ: &[u8] = include_bytes!("./vectors/Chess_qlt45.svg");
static WK: &[u8] = include_bytes!("./vectors/Chess_klt45.svg");
static BP: &[u8] = include_bytes!("./vectors/Chess_pdt45.svg");
static BN: &[u8] = include_bytes!("./vectors/Chess_ndt45.svg");
static BB: &[u8] = include_bytes!("./vectors/Chess_bdt45.svg");
static BR: &[u8] = include_bytes!("./vectors/Chess_rdt45.svg");
static BQ: &[u8] = include_bytes!("./vectors/Chess_qdt45.svg");
static BK: &[u8] = include_bytes!("./vectors/Chess_kdt45.svg");

#[component(no_case_check)]
pub fn Cell(
    #[props(default = "300".to_string())] size: String,
    #[props(default = "rgb(90, 100, 235)".to_string())] background_color: String,
    board: Memo<Option<Board>>,
    file: u8,
    rank: u8
) -> Element {
    let piece_fen = if let Some(board) = &*board.read() {
        owlchess_cell_to_fen_option(
            board.get2(File::from_index(file as usize), Rank::from_index(rank as usize))
        )
    } else {
        None
    };

    rsx!(
        DropZone {
            ondrop: move |data: String| {
                match &*board.read() {
                    Some(board) => {
                        println!("dropped {}", data);
                    },
                    _ => {}
                }
            },
            rect {
                width: "{size}",
                height: "{size}",
                background: background_color,
                if let Some(piece_fen) = piece_fen {
                    DragZone {
                        data: piece_fen.clone(),
                        drag_element: rsx!(
                            svg {
                                width: "{size}",
                                height: "{size}",
                                svg_data: piece_fen_to_svg(piece_fen.as_str()),
                            },
                        ),
                        svg {
                            width: "100%",
                            height: "100%",
                            svg_data: piece_fen_to_svg(piece_fen.as_str()),
                        },
                    }
                }
         }
    })
}

fn piece_fen_to_svg(piece_fen: &str) -> AttributeValue {
    match piece_fen {
        "P" => static_bytes(WP),
        "N" => static_bytes(WN),
        "B" => static_bytes(WB),
        "R" => static_bytes(WR),
        "Q" => static_bytes(WQ),
        "K" => static_bytes(WK),
        "p" => static_bytes(BP),
        "n" => static_bytes(BN),
        "b" => static_bytes(BB),
        "r" => static_bytes(BR),
        "q" => static_bytes(BQ),
        "k" => static_bytes(BK),
        _ => unreachable!(),
    }
}

fn owlchess_cell_to_fen_option(cell: owlchessCell) -> Option<String> {
    match cell.color() {
        Some(Color::White) =>
            match cell.piece() {
                Some(Piece::Pawn) => Some("P".to_string()),
                Some(Piece::Knight) => Some("N".to_string()),
                Some(Piece::Bishop) => Some("B".to_string()),
                Some(Piece::Rook) => Some("R".to_string()),
                Some(Piece::Queen) => Some("Q".to_string()),
                Some(Piece::King) => Some("K".to_string()),
                _ => None,
            }
        _ =>
            match cell.piece() {
                Some(Piece::Pawn) => Some("p".to_string()),
                Some(Piece::Knight) => Some("n".to_string()),
                Some(Piece::Bishop) => Some("b".to_string()),
                Some(Piece::Rook) => Some("r".to_string()),
                Some(Piece::Queen) => Some("q".to_string()),
                Some(Piece::King) => Some("k".to_string()),
                _ => None,
            }
    }
}
