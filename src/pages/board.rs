use rand::Rng;
use std::collections::HashMap;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
const SIZE: u32 = 20;
const NR_OF_BOMBS: u32 = 50;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Bomb(bool, bool),
    NotBomb(u8, bool, bool),
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
            tiles.insert((column, row), Tile::NotBomb(0, false, false));
        }
    }

    for _ in 0..NR_OF_BOMBS {
        let mut location;
        loop {
            location = random_bomb_location();
            match tiles[&location] {
                Tile::Bomb(_, _) => continue,
                _ => {
                    break;
                }
            }
        }

        tiles.insert(location, Tile::Bomb(false, false));

        let increment_neighs = |coord: &(u32, u32), tiles: &mut HashMap<(u32, u32), Tile>| {
            let tile = tiles.get(coord).unwrap();
            match tile {
                Tile::NotBomb(nr, _, _) => Tile::NotBomb(nr + 1, false, false),
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
    RightClick(u32, u32)
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
            },
            Msg::RightClick(col, row) => {
                match self.tiles[&(col, row)] {
                    Tile::Bomb(false, false) => self.tiles.insert((col, row), Tile::Bomb(false, true)),
                    Tile::NotBomb(nr, false, false) => self.tiles.insert((col, row), Tile::NotBomb(nr, false, true)),
                    _ => { }
                }
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

            match self.tiles[&(col, row)] {
                Tile::Bomb(_, false) => {
                    self.tiles.insert((col, row), Tile::Bomb(true, false));
                    self.game_over = true;
                }
                Tile::NotBomb(0, false, false) => {
                    self.tiles.insert((col, row), Tile::NotBomb(0, true, false));

                    for col_neigh in left(col)..=right(col) {
                        for row_neigh in above(row)..=below(row) {
                            match self.tiles[&(col_neigh, row_neigh)] {
                                Tile::NotBomb(_, false, false) => {
                                    self.click(col_neigh, row_neigh)
                                },
                                _ => {}
                            }
                        }
                    }
                }
                Tile::NotBomb(nr, false, false) => {
                    self.tiles.insert((col, row), Tile::NotBomb(nr, true, false));
                },
                _ => {}
            };
    }
    fn view_square(&self, row: u32, column: u32) -> Html {
        html! {
            <td class=self.square_class((column, row))
                onclick=self.link.callback(move |_| Msg::Select(column, row))
                oncontextmenu=self.link.callback(move |_| Msg::RightClick(column, row))>
                <div> { self.get_tile_nr(column, row) } </div>
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
                Tile::Bomb(_, _) => {
                    return "bomb";
                }
                _ => {}
            }
        }
        match self.tiles[&tile] {
            Tile::NotBomb(_, true, false) => "touched_tile",
            Tile::NotBomb(_, false, true) => "flag",
            Tile::Bomb(_, true) => "flag",
            _ => "untouched_tile",
        }
    }

    fn get_tile_nr(&self, col: u32, row: u32) -> String {
        match self.tiles[&(col, row)] {
            Tile::NotBomb(0, true, _) => "".to_string(),
            Tile::NotBomb(nr, true, _) => format!("{}", nr),
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
