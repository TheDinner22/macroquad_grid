// so a grid is composed of cells?
// this is becuase if we want to write to the grid
// it would be nice if it remembered what it was doing

// simple ass struct, doesn't even have no impl
#[derive(Default)]
pub struct Cell {
    pub color: Option<macroquad::color::Color>,
    pub value: String,
    pub text_color: Option<macroquad::color::Color>
}
