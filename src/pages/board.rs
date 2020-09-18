use rand::Rng;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

const SIZE: u32 = 20;
const NR_OF_BOMBS:u32 = 80;

pub struct Board {
    link: ComponentLink<Self>,
    selected: Vec<(u32, u32)>,
    bombs: [(u32, u32); NR_OF_BOMBS as usize],
    game_over: bool,
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
        let mut bomb_placements: [(u32, u32); NR_OF_BOMBS as usize] = [(0, 0); NR_OF_BOMBS as usize];

        for index in 0..NR_OF_BOMBS {
            let location = random_bomb_location();
            bomb_placements[index as usize] = location;
        }

        Board {
            link,
            selected: Vec::new(),
            bombs: bomb_placements,
            game_over: false,
        }
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.game_over {
            return false
        }
        match msg {
            Msg::Select(x, y) => {
                self.selected.push((x, y));
                if self.bombs.contains(&(x, y)) {
                    self.game_over = true;
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
}

impl Board {
    fn view_square(&self, row: u32, column: u32) -> Html {
        html! {
            <td class=self.square_class((column, row))
                onclick=self.link.callback(move |_| Msg::Select(column, row))>
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

    fn square_class(&self, this: (u32, u32)) -> &'static str {
        if self.bombs.contains(&this) && self.game_over {
            "bomb"
        } else if self.selected.contains(&this) {
            "square_green"
        } else {
            "untouched_tile"
        }
    }

    // fn number_of_neighbour_bom
}
