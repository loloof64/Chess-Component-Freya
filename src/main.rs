#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
mod chess_component;

use chess_component::ChessBoard;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "white",
            main_align: "center",
            cross_align: "center",
            ChessBoard {
                background_color: "#808000",
                size: "200",
                white_cell_color: "#ffd700",
                black_cell_color: "#a52a2a",
                coordinates_color: "#0022ff",
                hide_coordinates: true,
                reversed_orientation: true,
                start_position: "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
            }
            ChessBoard {
            
            }
        }
    )
}
