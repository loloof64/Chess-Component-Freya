use freya::prelude::*;
use owlchess::{ Board, Color };

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
    #[props(default = false)] reversed_orientation: bool,
    #[props(
        default = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    )] start_position: String
) -> Element {
    let board_size = size.clone();
    let width = size.clone();
    let height = size.clone();
    let (node_ref, size) = use_node();
    let real_size = size.area.width();
    let cell_size = real_size / 9.0;
    let half_cell_size = cell_size / 2.0;
    let width_without_borders = cell_size * 8.0;
    let font_size = half_cell_size * 0.8;
    let cell_size_str = format!("{}", cell_size);
    let half_cell_size_str = format!("{}", half_cell_size);
    let width_without_borders_str = format!("{}", width_without_borders);
    let font_size_str = format!("{}", font_size);

    let position = use_signal(|| start_position);
    let position_object = use_memo(move || Board::from_fen(position().as_str()));
    let white_turn = use_memo(move || position_object().map(|board| board.side() == Color::White));
    let board_object = use_memo(move || position_object().ok());

    rsx!(rect {
        width: width,
        height: height,
        background: background_color,
        reference: node_ref,
        {
            rsx!(
                DragProvider::<String> {
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
                                width: width_without_borders_str.clone(),
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
                        // player turn if needed
                        match white_turn() {
                            Ok(white_turn) => {
                                if (reversed_orientation && white_turn) || (!white_turn && !reversed_orientation) {
                                    rsx!(
                                        rect {
                                            width:half_cell_size_str.clone(),
                                            height: half_cell_size_str.clone(),
                                            rect {
                                                width: "100%",
                                                height: "100%",
                                                background: if white_turn {"white"} else {"black"},
                                                corner_radius: half_cell_size_str.clone(),
                                            }
                                        }
                                    )
                                }
                                else {
                                    rsx!(
                                        rect {
                                            width:half_cell_size_str.clone(),
                                            height: half_cell_size_str.clone(),
                                        }
                                    )
                                }
                            },
                            _ => rsx!(
                                rect {
                                    width:half_cell_size_str.clone(),
                                    height: half_cell_size_str.clone(),
                                }
                            )
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
                                    board_memo: board_object,
                                    file: if reversed_orientation {7-col as u8} else {col as u8},
                                    rank: if reversed_orientation {row as u8} else {7-row as u8},
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
                                width: width_without_borders_str.clone(),
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
                        // player turn if needed
                        match white_turn() {
                            Ok(white_turn) => {
                                if (reversed_orientation && !white_turn) || (white_turn && !reversed_orientation) {
                                    rsx!(
                                        rect {
                                            width:half_cell_size_str.clone(),
                                            height: half_cell_size_str.clone(),
                                            rect {
                                                width: "100%",
                                                height: "100%",
                                                background: if white_turn {"white"} else {"black"},
                                                corner_radius: half_cell_size_str.clone(),
                                            }
                                        }
                                    )
                                }
                                else {
                                    rsx!(
                                        rect {
                                            width:half_cell_size_str.clone(),
                                            height: half_cell_size_str.clone(),
                                        }
                                    )
                                }
                            },
                            _ => rsx!(
                                rect {
                                    width:half_cell_size_str.clone(),
                                    height: half_cell_size_str.clone(),
                                }
                            )
                        }
                    }
                }
            )
        }
    })
}
