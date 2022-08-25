use crate::state::*;
use crate::engine::*;

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

impl CLIDrawable for PrizeCard {
    fn draw(&self, x: usize, y: usize, target: &mut CLIDrawTarget) {
        self.card.draw(x, y, target)
    }
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
                target.draw_line(&format!("| {:3} |", &c.archetype[0..3]), x, y + 1);
                target.draw_line(&format!("| {:3} |", &c.archetype[3..6]), x, y + 2);
                target.draw_line("|_____|", x, y + 3);
            },
        }
    }
}

pub fn energy_symbol(energy: &Type) -> String {
    match energy {
        Type::Colorless => "[C]",
        Type::Fighting => "[F]",
        Type::Fire => "[R]",
        Type::Grass => "[G]",
        Type::Lightning => "[L]",
        Type::Psychic => "[P]",
        Type::Water => "[W]",
        Type::Dark => "[D]",
        Type::Metal => "[M]",
        Type::Fairy => "[Y]",
        Type::Dragon => "[N]",
        Type::Any => "[*]",
    }.into()
}

impl InPlayCard {
    fn draw(&self, x: usize, y: usize, target: &mut CLIDrawTarget, engine: &GameEngine) {
        self.stack[0].draw(x, y, target);

        let energies = self.attached
            .iter()
            .filter(|card| card.is_up())
            .flat_map(|card| engine.provides(card.card()))
            .map(|energy_type| energy_symbol(&energy_type))
            .collect::<Vec<_>>()
            .join("");

        // "Defender (BS 80)" => "{Defender}",
        target.draw_line(&energies, x, y - 1);

        target.draw_line(&format!("{} HP", engine.remaining_hp(self)), x, y - 2);

        match self.rotational_status {
            RotationalStatus::Paralyzed => { target.draw_line("* Paralyzed", x, y - 3); },
            RotationalStatus::Asleep    => { target.draw_line("* Asleep", x, y - 3); },
            RotationalStatus::Confused  => { target.draw_line("* Confused", x, y - 3); },
            _ => {},
        }
    }
}

impl CLIDrawable for GameEngine {
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

        match self.state.stage {
            GameStage::Uninitialized => {
                target.draw_line("x", 3, 24);
                target.draw_line("x", 3, 25);
            },
            GameStage::StartOfTurn(_) => {},
            GameStage::EndOfTurn(_) => {},
            GameStage::Turn(Player::One) => {
                target.draw_line("v", 3, 25);
            },
            GameStage::Turn(Player::Two) => {
                target.draw_line("^", 3, 24);
            },
            GameStage::PokemonCheckup(_) => {
                target.draw_line("*", 3, 24);
                target.draw_line("*", 3, 25);
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

        target.draw_line(&format!("{:3}", self.state.p1.deck.len()), x + 64, 35);
        target.draw_line(&format!("{:3}", self.state.p1.discard.len()), x + 64, 40);
        if !self.state.p1.active.is_empty() {
            self.state.p1.active[0].draw(x + 35, y + 28, target, self);
        }
        for (i, benched) in self.state.p1.bench.iter().enumerate() {
            benched.draw(x + 19 + i * 8, 38, target, self);
        }
        for (i, prize) in self.state.p1.prizes.iter().rev().enumerate() {
            prize.draw(x + 0 + (i%2) * 8, y + 28 + (i/2) * 5, target);
        }
        for (i, card) in self.state.p1.hand.iter().enumerate() {
            FaceCard::Up(card.clone()).draw(i*8, 43, target);
        }

        target.draw_line(&format!("{:3}", self.state.p2.deck.len()), x + 64, 15);
        target.draw_line(&format!("{:3}", self.state.p2.discard.len()), x + 64, 10);
        if !self.state.p2.active.is_empty() {
            self.state.p2.active[0].draw(x + 35, y + 18, target, self);
        }
        for (i, benched) in self.state.p2.bench.iter().enumerate() {
            benched.draw(x + 19 + i * 8, 8, target, self);
        }
        for (i, prize) in self.state.p2.prizes.iter().enumerate() {
            prize.draw(x + 0 + (i%2) * 8, y + 8 + (i/2) * 5, target);
        }
        for (i, card) in self.state.p2.hand.iter().enumerate() {
            FaceCard::Up(card.clone()).draw(i*8, 1, target);
        }

        target.draw_line(&format!("Player One manual attachments: {}", self.state.p1.manual_attachments_this_turn), x + 80, 8);
        target.draw_line(&format!("Player Two manual attachments: {}", self.state.p2.manual_attachments_this_turn), x + 80, 9);

        for (i, effect) in self.state.effects.iter().enumerate() {
            target.draw_line(&format!("Effect on {:?}", effect.target), x + 80, 10 + i*5 + 0);
            target.draw_line(&format!("  what: {:?}", effect.consequence), x + 80, 10 + i*5 + 1);
            target.draw_line(&format!("  name: {}", effect.name), x + 80, 10 + i*5 + 2);
            target.draw_line(&format!("  source: {:?}", effect.source), x + 80, 10 + i*5 + 3);
            target.draw_line(&format!("  expires: {:?}", effect.expires), x + 80, 10 + i*5 + 4);
        }
    }
}
