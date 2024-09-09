use freya::prelude::*;
use owlchess::{ Board, Cell as owlchessCell, Color, File, Piece, Rank };

mod cell_component;
use cell_component::Cell;

#[component(no_case_check)]
pub fn ChessBoard(
    #[props(default = "300".to_string())] size: String,
    #[props(default = "rgb(90, 100, 235)".to_string())] background_color: String,
    #[props(default = "#ffdead".to_string())] white_cell_color: String,
    #[props(default = "#cd853f".to_string())] black_cell_color: String,
    #[props(default = "#ffd700".to_string())] coordinates_color: String,
    #[props(default = false)] hide_coordinates: bool,
    #[props(default = false)] reversed_orientation: bool
) -> Element {
    let board_size = size.clone();
    let width = size.clone();
    let height = size.clone();
    let (node_ref, size) = use_node();
    let real_size = size.area.width();
    let cell_size = real_size / 9.0;
    let half_cell_size = cell_size / 2.0;
    let font_size = half_cell_size * 0.8;
    let cell_size_str = format!("{}", cell_size);
    let half_cell_size_str = format!("{}", half_cell_size);
    let font_size_str = format!("{}", font_size);

    let position = use_signal(|| "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let position_object = use_memo(move || Board::from_fen(position()));

    rsx!(rect {
        width: width,
        height: height,
        background: background_color,
        reference: node_ref,
        {
            rsx!(
                // Up coordinates line
                rect {
                    width: board_size.clone(),
                    height: half_cell_size_str.clone(),
                    direction: "horizontal",
                    rect {
                        width:half_cell_size_str.clone(),
                        height: half_cell_size_str.clone(),
                    }
                    if hide_coordinates {
                        rect {
                            width: cell_size_str.clone(),
                            height: half_cell_size_str.clone(),
                        }
                    }
                    else {
                        for col in 0..8 {
                            {
                            let coordinates = ["A", "B", "C", "D", "E", "F", "G", "H"];
                            let coordinate = if reversed_orientation {coordinates[7-col]} else {coordinates[col]};
                            rsx!(
                                    label {
                                        text_align: "center",
                                        font_size: font_size_str.clone(),
                                        font_weight: "bold",
                                        color: coordinates_color.clone(),
                                        width: cell_size_str.clone(),
                                        height: half_cell_size_str.clone(),
                                        "{coordinate}"
                                    }
                                )
                            }
                        }
                    }
                }
                // Central lines
                for row in 0..8 {
                    rect {
                        direction: "horizontal",
                        // Left coordinates
                        if hide_coordinates {
                            rect {
                                width: half_cell_size_str.clone(),
                                height: cell_size_str.clone(),
                            }
                        }
                        else {
                            {
                                let coordinates = ["8", "7", "6", "5", "4", "3", "2", "1"];
                                let coordinate = if reversed_orientation {coordinates[7-row]} else {coordinates[row]};
                                rsx!(
                                    rect {
                                        width: half_cell_size_str.clone(),
                                        height: cell_size_str.clone(),
                                        main_align: "center",
                                        cross_align: "center",    
                                        label {
                                            font_size: font_size_str.clone(),
                                            font_weight: "bold",
                                            color: coordinates_color.clone(),
                                            "{coordinate}"
                                        }
                                    }   
                                )
                            }
                        }
                        // Cells
                        for col in 0..8 {
                            Cell {
                                size: cell_size_str.clone(),
                                background_color: if (row + col) % 2 == 0 { white_cell_color.clone() } 
                                    else {  black_cell_color.clone() },
                                piece_fen: match position_object() {
                                    Ok(board) => owlchess_cell_to_fen_option(board.get2(File::from_index(col), Rank::from_index(row))),
                                _ => None,
                                }
                            }
                        }
                        // Right coordinates
                        if hide_coordinates {
                            rect {
                                width: half_cell_size_str.clone(),
                                height: cell_size_str.clone(),
                            }
                        }
                        else {
                            {
                                let coordinates = ["8", "7", "6", "5", "4", "3", "2", "1"];
                                let coordinate = if reversed_orientation {coordinates[7-row]} else {coordinates[row]};
                                rsx!(
                                    rect {
                                        width: half_cell_size_str.clone(),
                                        height: cell_size_str.clone(),
                                        main_align: "center",
                                        cross_align: "center",    
                                        label {
                                            font_size: font_size_str.clone(),
                                            font_weight: "bold",
                                            color: coordinates_color.clone(),
                                            "{coordinate}"
                                        }
                                    }   
                                )
                            }
                        }
                    }
                }
                // Down coordinates line
                rect {
                    width: board_size.clone(),
                    height: half_cell_size_str.clone(),
                    direction: "horizontal",
                    rect {
                        width:half_cell_size_str.clone(),
                        height: half_cell_size_str.clone(),
                    }
                    if hide_coordinates {
                        rect {
                            width: cell_size_str.clone(),
                            height: half_cell_size_str.clone(),
                        }
                    }
                    else {
                        for col in 0..8 {
                            {
                            let coordinates = ["A", "B", "C", "D", "E", "F", "G", "H"];
                            let coordinate = if reversed_orientation {coordinates[7-col]} else {coordinates[col]};
                            rsx!(
                                    label {
                                        text_align: "center",
                                        font_size: font_size_str.clone(),
                                        font_weight: "bold",
                                        color: coordinates_color.clone(),
                                        width: cell_size_str.clone(),
                                        height: half_cell_size_str.clone(),
                                        "{coordinate}"
                                    }
                                )
                            }
                        }
                    }
                }
            )
        }
    })
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
