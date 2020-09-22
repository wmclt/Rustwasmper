use rand::Rng;
use std::collections::HashMap;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
const SIZE: u32 = 20;
const NR_OF_BOMBS: u32 = 50;

#[derive(Debug, Copy, Clone)]
pub enum TileType {
    Bomb,
    NotBomb(u32),
}

#[derive(Debug, Copy, Clone)]
pub enum TileState {
    Flagged,
    QuestionMark,
    Visible,
    Hidden,
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    tile_type: TileType,
    state: TileState,
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
            tiles.insert(
                (column, row),
                Tile {
                    tile_type: TileType::NotBomb(0),
                    state: TileState::Hidden,
                },
            );
        }
    }

    for _ in 0..NR_OF_BOMBS {
        let mut location;
        loop {
            location = random_bomb_location();
            match tiles[&location].tile_type {
                TileType::Bomb => continue,
                _ => {
                    break;
                }
            }
        }

        tiles.insert(
            location,
            Tile {
                tile_type: TileType::Bomb,
                state: TileState::Hidden,
            },
        );

        let increment_neighs = |coord: &(u32, u32), tiles: &mut HashMap<(u32, u32), Tile>| {
            let tile = tiles.get(coord).unwrap();
            match tile.tile_type {
                TileType::NotBomb(nr) => Tile {
                    tile_type: TileType::NotBomb(nr + 1),
                    state: TileState::Hidden,
                },
                _ => *tile,
            }
        };

        apply_to_neighs(location, &mut tiles, increment_neighs);
    }
    tiles
}

fn apply_to_neighs(
    (col, row): (u32, u32),
    tiles: &mut HashMap<(u32, u32), Tile>,
    func: fn(&(u32, u32), &mut HashMap<(u32, u32), Tile>) -> Tile,
) {
    for col_neigh in left(col)..=right(col) {
        for row_neigh in above(row)..=below(row) {
            let updated_tile = func(&(col_neigh, row_neigh), tiles);
            tiles.insert((col_neigh, row_neigh), updated_tile);
        }
    }
}

fn random_bomb_location() -> (u32, u32) {
    let mut generator = rand::thread_rng();
    let x_rand = generator.gen_range(0, SIZE);
    let y_rand = generator.gen_range(0, SIZE);
    (x_rand, y_rand)
}

pub enum Msg {
    Select(u32, u32),
    RightClick(u32, u32),
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
                self.click(col, row);
            }
            Msg::RightClick(col, row) => {
                self.right_click(col, row);
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

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

impl Board {
    fn click(&mut self, col: u32, row: u32) {
        let tile_type = self.tiles[&(col, row)].tile_type;
        self.tiles.insert(
            (col, row),
            Tile {
                tile_type,
                state: TileState::Visible,
            },
        );
        match tile_type {
            TileType::Bomb => {
                self.game_over = true;
            }
            TileType::NotBomb(0) => {
                for col_neigh in left(col)..=right(col) {
                    for row_neigh in above(row)..=below(row) {
                        let neigh_tile = self.tiles[&(col_neigh, row_neigh)];
                        match neigh_tile.state {
                            TileState::Hidden => {
                                self.click(col_neigh, row_neigh);
                            }
                            _ => {}
                        }
                    }
                }
            }
            TileType::NotBomb(_) => {}
        };
    }

    fn right_click(&mut self, col: u32, row: u32) {
        let tile = self.tiles[&(col, row)];
        let tile_type = tile.tile_type;
        let state = match tile.state {
            TileState::Hidden => TileState::Flagged,
            TileState::Flagged => TileState::QuestionMark,
            TileState::QuestionMark => TileState::Hidden,
            TileState::Visible => TileState::Visible,
        };
        self.tiles.insert((col, row), Tile { tile_type, state });
    }

    fn view_square(&self, row: u32, column: u32) -> Html {
        html! {
            <td class=self.square_class((column, row))
                onclick=self.link.callback(move |_| Msg::Select(column, row))
                oncontextmenu=self.link.callback(move |_| Msg::RightClick(column, row))>
                <div> { self.get_tile_symbol(column, row) } </div>
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

    fn square_class(&self, coord: (u32, u32)) -> &'static str {
        if self.game_over {
            match self.tiles[&coord].tile_type {
                TileType::Bomb => {
                    return "bomb";
                }
                _ => {}
            }
        }
        match self.tiles[&coord].state {
            TileState::Visible => "visible",
            TileState::Flagged => "flag",
            TileState::QuestionMark => "question-mark",
            _ => "hidden",
        }
    }

    fn get_tile_symbol(&self, col: u32, row: u32) -> String {
        match self.tiles[&(col, row)].state {
            TileState::Flagged => "🚩".to_string(),
            TileState::QuestionMark => "❓".to_string(),
            TileState::Visible => match self.tiles[&(col, row)].tile_type {
                TileType::NotBomb(0) => "".to_string(),
                TileType::NotBomb(nr) => format!("{}", nr),
                _ => "".to_string(),
            },
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
