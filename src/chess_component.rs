use freya::prelude::*;

#[component(no_case_check)]
pub fn ChessBoard(
    #[props(default = "300".to_string())]    
    size: String, 
    #[props(default = "rgb(90, 100, 235)".to_string())]
    background_color: String
) -> Element {
    let width = size.clone();
    let height = size.clone();
    rsx!(rect {
        width: width,
        height: height,
        background: background_color,
    })
}