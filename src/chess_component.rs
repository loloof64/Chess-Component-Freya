use freya::prelude::*;

mod cell_component;
use cell_component::Cell;

#[component(no_case_check)]
pub fn ChessBoard(
    #[props(default = "300".to_string())] size: String,
    #[props(default = "rgb(90, 100, 235)".to_string())] background_color: String,
    #[props(default = "#ffdead".to_string())] white_cell_color: String,
    #[props(default = "#cd853f".to_string())] black_cell_color: String,
) -> Element {
    let board_size = size.clone();
    let width = size.clone();
    let height = size.clone();
    let (node_ref, size) = use_node();
    let real_size = size.area.width();
    let cell_size = real_size / 9.0;
    let half_cell_size = cell_size / 2.0;
    let cell_size_str = format!("{}", cell_size);
    let half_cell_size_str = format!("{}", half_cell_size);

    rsx!(rect {
            width: width,
            height: height,
            background: background_color,
            reference: node_ref,
            {
                rsx!(
                    rect {
                        width: board_size.clone(),
                        height: half_cell_size_str.clone(),
                    }
                    for row in 0..8 {
                        rect {
                            direction: "horizontal",
                            rect {
                                width: half_cell_size_str.clone(),
                                height: cell_size_str.clone(),
                            }
                            for col in 0..8 {
                                Cell {
                                    size: cell_size,
                                    background_color: if (row + col) % 2 == 0 { white_cell_color.clone() } else {  black_cell_color.clone() },
                                }
                            }
                        }
                    }
                )
            }
        })
}
