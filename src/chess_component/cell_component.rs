use dioxus_core::AttributeValue;
use freya::prelude::*;

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
    #[props(default = None)] piece_fen: Option<String>
) -> Element {
    let width = size.clone();
    let height = size.clone();
    let image = piece_fen_to_svg(piece_fen);

    rsx!(rect {
        width: width,
        height: height,
        background: background_color,

        if let Some(svg_data) = image {
            svg {
                width: "100%",
                height: "100%",
                svg_data,
            }
        }
    })
}

fn piece_fen_to_svg(piece_fen: Option<String>) -> Option<AttributeValue> {
    if piece_fen.is_none() {
        return None;
    }
    match piece_fen.unwrap().as_str() {
        "P" => Some(static_bytes(WP)),
        "N" => Some(static_bytes(WN)),
        "B" => Some(static_bytes(WB)),
        "R" => Some(static_bytes(WR)),
        "Q" => Some(static_bytes(WQ)),
        "K" => Some(static_bytes(WK)),
        "p" => Some(static_bytes(BP)),
        "n" => Some(static_bytes(BN)),
        "b" => Some(static_bytes(BB)),
        "r" => Some(static_bytes(BR)),
        "q" => Some(static_bytes(BQ)),
        "k" => Some(static_bytes(BK)),
        _ => None,
    }
}
