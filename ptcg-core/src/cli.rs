use crate::state::*;

#[derive(Default)]
pub struct CLIDrawTarget {
    lines: Vec<Vec<char>>,
}

impl CLIDrawTarget {
    pub fn print(drawable: &dyn CLIDrawable) {
        let mut draw = Self::default();
        drawable.draw(0, 0, &mut draw);
        for i in 0..draw.lines.len() {
            println!("{}", draw.line(i));
        }
    }

    pub fn draw_line(&mut self, text: &str, x: usize, y: usize) {
        while !(y < self.lines.len()) {
            self.lines.push(vec![]);
        }

        while !(x + text.chars().count() < self.lines[y].len()) {
            self.lines[y].push(' ');
        }

        for (i, c) in text.chars().enumerate() {
            self.lines[y][x + i] = c;
        }
    }

    pub fn line(&self, n: usize) -> String {
        self.lines[n].iter().cloned().collect::<String>()
    }
}

pub trait CLIDrawable {
    fn draw(&self, x: usize, y: usize, target: &mut CLIDrawTarget);
}

impl CLIDrawable for FaceCard {
    fn draw(&self, x: usize, y: usize, target: &mut CLIDrawTarget) {
        match &self {
            FaceCard::Down(_) => {
                target.draw_line("|‾‾‾‾‾|", x, y);
                target.draw_line("|  ?  |", x, y + 1);
                target.draw_line("|  ?  |", x, y + 2);
                target.draw_line("|_____|", x, y + 3);
            },
            FaceCard::Up(c) => {
                target.draw_line("|‾‾‾‾‾|", x, y);
                target.draw_line(&format!("| {:3} |", &c[0..3]), x, y + 1);
                target.draw_line(&format!("| {:3} |", &c[3..6]), x, y + 2);
                target.draw_line("|_____|", x, y + 3);
            },
        }
    }
}


impl CLIDrawable for InPlayCard {
    fn draw(&self, x: usize, y: usize, target: &mut CLIDrawTarget) {
        self.stack[0].draw(x, y, target);

        let energies = self.attached
            .iter()
            .filter(|card| match card { FaceCard::Up(_) => true, _ => false })
            .map(|card| match card { FaceCard::Up(card) => match card.as_str() {
                "Psychic Energy (BS 101)" => "[P]",
                "Water Energy (BS 102)" => "[P]",
                "Double Colorless Energy (BS 96)" => "[C][C]",
                _ => "[C]"
            }, _ => "" })
            .collect::<Vec<_>>()
            .join("")
            ;

        target.draw_line(&energies, x, y - 1);
    }
}

impl CLIDrawable for GameState {
    fn draw(&self, x: usize, y: usize, target: &mut CLIDrawTarget) {
        target.draw_line("{     } {     }    {     } {     } {     } {     } {     }    |‾‾‾‾‾|", x, y +  8);
        target.draw_line("{  P  } {  P  }    {  B  } {  B  } {  B  } {  B  } {  B  }    |  U  |", x, y +  9);
        target.draw_line("{     } {     }    {     } {     } {     } {     } {     }    |     |", x, y + 10);
        target.draw_line("{     } {     }    {     } {     } {     } {     } {     }    |_____|", x, y + 11);
        target.draw_line("                                                                     ", x, y + 12);
        target.draw_line("{     } {     }                                               |‾‾‾‾‾|", x, y + 13);
        target.draw_line("{  P  } {  P  }                                               |  D  |", x, y + 14);
        target.draw_line("{     } {     }                                               |     |", x, y + 15);
        target.draw_line("{     } {     }                                               |_____|", x, y + 16);
        target.draw_line("                                                                     ", x, y + 17);
        target.draw_line("{     } {     }                    {     }                           ", x, y + 18);
        target.draw_line("{  P  } {  P  }                    {  A  }                           ", x, y + 19);
        target.draw_line("{     } {     }                    {     }                           ", x, y + 20);
        target.draw_line("{     } {     }                    {     }                           ", x, y + 21);
        target.draw_line("                                                                     ", x, y + 22);
        target.draw_line("                                                                     ", x, y + 23);
        target.draw_line("                                                                     ", x, y + 24);
        target.draw_line("                                                                     ", x, y + 25);
        target.draw_line("                                                                     ", x, y + 26);
        target.draw_line("                                                                     ", x, y + 27);
        target.draw_line("{     } {     }                    {     }                           ", x, y + 28);
        target.draw_line("{  P  } {  P  }                    {  A  }                           ", x, y + 29);
        target.draw_line("{     } {     }                    {     }                           ", x, y + 30);
        target.draw_line("{     } {     }                    {     }                           ", x, y + 31);
        target.draw_line("                                                                     ", x, y + 32);
        target.draw_line("{     } {     }                                               |‾‾‾‾‾|", x, y + 33);
        target.draw_line("{  P  } {  P  }                                               |  D  |", x, y + 34);
        target.draw_line("{     } {     }                                               |     |", x, y + 35);
        target.draw_line("{     } {     }                                               |_____|", x, y + 36);
        target.draw_line("                                                                     ", x, y + 37);
        target.draw_line("{     } {     }    {     } {     } {     } {     } {     }    |‾‾‾‾‾|", x, y + 38);
        target.draw_line("{  P  } {  P  }    {  B  } {  B  } {  B  } {  B  } {  B  }    |  U  |", x, y + 39);
        target.draw_line("{     } {     }    {     } {     } {     } {     } {     }    |     |", x, y + 40);
        target.draw_line("{     } {     }    {     } {     } {     } {     } {     }    |_____|", x, y + 41);

        match self.stage {
            GameStage::Uninitialized => {
                target.draw_line("x", 3, 24);
                target.draw_line("x", 3, 25);
            },
            GameStage::StartOfTurn(_) => {},
            GameStage::Turn(Player::One) => {
                target.draw_line("v", 3, 25);
            },
            GameStage::Turn(Player::Two) => {
                target.draw_line("^", 3, 24);
            },
            GameStage::Winner(Player::One) => {
                target.draw_line("Player one wins!", 3, 25);
            },
            GameStage::Winner(Player::Two) => {
                target.draw_line("Player two wins!", 3, 24);
            },
            GameStage::Tie => {
                target.draw_line("It's a tie!", 3, 24);
            },
        }

        target.draw_line(&format!("{:3}", self.p1.deck.len()), x + 64, 35);
        target.draw_line(&format!("{:3}", self.p1.discard.len()), x + 64, 40);
        if !self.p1.active.is_empty() {
            self.p1.active[0].draw(x + 35, y + 28, target);
        }
        for (i, benched) in self.p1.bench.iter().enumerate() {
            benched.draw(x + 19 + i * 8, 38, target);
        }
        for (i, prize) in self.p1.prizes.iter().rev().enumerate() {
            prize.draw(x + 0 + (i%2) * 8, y + 28 + (i/2) * 5, target);
        }
        for (i, card) in self.p1.hand.iter().enumerate() {
            FaceCard::Up(card.clone()).draw(i*8, 43, target);
        }

        target.draw_line(&format!("{:3}", self.p2.deck.len()), x + 64, 15);
        target.draw_line(&format!("{:3}", self.p2.discard.len()), x + 64, 10);
        if !self.p2.active.is_empty() {
            self.p2.active[0].draw(x + 35, y + 18, target);
        }
        for (i, benched) in self.p2.bench.iter().enumerate() {
            benched.draw(x + 19 + i * 8, 8, target);
        }
        for (i, prize) in self.p2.prizes.iter().enumerate() {
            prize.draw(x + 0 + (i%2) * 8, y + 8 + (i/2) * 5, target);
        }
        for (i, card) in self.p2.hand.iter().enumerate() {
            FaceCard::Up(card.clone()).draw(i*8, 3, target);
        }

        for (i, effect) in self.effects.iter().enumerate() {
            target.draw_line(&format!("{:?}", effect), x + 80, 8 + i);
        }
    }
}
