use rand::Rng;
use std::collections::HashMap;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

const SIZE: u32 = 20;
const NR_OF_BOMBS: u32 = 50;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Bomb(bool),
    NotBomb(u8, bool),
}

pub struct Board {
    link: ComponentLink<Self>,
    game_over: bool,
    tiles: HashMap<(u32, u32), Tile>,
}

fn generate_board() -> HashMap<(u32, u32), Tile> {
    let mut tiles: HashMap<(u32, u32), Tile> = HashMap::with_capacity((SIZE * SIZE) as usize);
    for row in 0..SIZE {
        for column in 0..SIZE {
            tiles.insert((column, row), Tile::NotBomb(0, false));
        }
    }

    for _ in 0..NR_OF_BOMBS {
        let mut location;
        loop {
            location = random_bomb_location();
            match tiles[&location] {
                Tile::Bomb(_) => continue,
                _ => {
                    break;
                }
            }
        }

        tiles.insert(location, Tile::Bomb(false));
        let (col, row) = location;
        for col_neigh in left(col)..=right(col) {
            for row_neigh in above(row)..=below(row) {
                let updated_tile;
                match tiles.get(&(col_neigh, row_neigh)).unwrap() {
                    Tile::NotBomb(nr, _) => {
                        updated_tile = Tile::NotBomb(nr + 1, false);
                    }
                    _ => {
                        continue;
                    }
                };
                tiles.insert((col_neigh, row_neigh), updated_tile);
            }
        }
    }
    tiles
}

fn random_bomb_location() -> (u32, u32) {
    let mut generator = rand::thread_rng();
    let x_rand = generator.gen_range(0, SIZE);
    let y_rand = generator.gen_range(0, SIZE);
    (x_rand, y_rand)
}

pub enum Msg {
    Select(u32, u32),
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), link: ComponentLink<Self>) -> Self {
        Board {
            link,
            game_over: false,
            tiles: generate_board(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.game_over {
            return false;
        }
        match msg {
            Msg::Select(col, row) => {
                match self.tiles[&(col, row)] {
                    Tile::Bomb(_) => {
                        self.tiles.insert((col, row), Tile::Bomb(true));
                        self.game_over = true;
                    }
                    Tile::NotBomb(nr, _) => {
                        self.tiles.insert((col, row), Tile::NotBomb(nr, true));
                    }
                };
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <table>
                { (0..SIZE).map(|row| self.view_row(row)).collect::<Html>() }
            </table>
        }
    }
}

impl Board {
    fn view_square(&self, row: u32, column: u32) -> Html {
        html! {
            <td class=self.square_class((column, row))
                onclick=self.link.callback(move |_| Msg::Select(column, row))>
                <div> { self.get_tile_nr(column, row) } </div>
                // <div> { column } { row } </div>
            </td>
        }
    }

    fn view_row(&self, row: u32) -> Html {
        html! {
            <tr>
                {for (0..SIZE).map(|column| {
                    self.view_square(row, column)
                })}
            </tr>
        }
    }

    fn square_class(&self, tile: (u32, u32)) -> &'static str {
        if self.game_over {
            match self.tiles[&tile] {
                Tile::Bomb(_) => {
                    return "bomb";
                }
                _ => {}
            }
        }
        match self.tiles[&tile] {
            Tile::NotBomb(_, true) => "touched_tile",
            _ => "untouched_tile",
        }
    }

    fn get_tile_nr(&self, col: u32, row: u32) -> String {
        match self.tiles[&(col, row)] {
            Tile::NotBomb(nr, true) => format!("{}", nr),
            _ => "".to_string(),
        }
    }
}

fn left(col: u32) -> u32 {
    if col == 0 {
        return 0;
    }
    col - 1
}

fn right(col: u32) -> u32 {
    if col == SIZE - 1 {
        return SIZE - 1;
    }
    col + 1
}

fn above(row: u32) -> u32 {
    if row == 0 {
        return 0;
    }
    row - 1
}

fn below(row: u32) -> u32 {
    if row == SIZE - 1 {
        return SIZE - 1;
    }
    row + 1
}
