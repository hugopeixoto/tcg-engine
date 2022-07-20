#![feature(derive_default_enum)]

extern crate rand;
use crate::rand::Rng;

type Card = String;

#[derive(Clone)]
pub enum DeckSlice {
    Shuffled(Vec<Card>),
    Ordered(Vec<Card>),
}

#[derive(Default, Clone)]
pub struct Deck {
    slices: Vec<DeckSlice>,
}

impl Deck {
    pub fn new(cards: &[Card]) -> Self {
        Self {
            slices: vec![DeckSlice::Ordered(cards.into())],
        }
    }

    pub fn shuffle(&self) -> Self {
        Self {
            slices: vec![DeckSlice::Shuffled(
                self.slices.iter().flat_map(|x| match x { DeckSlice::Shuffled(x) => x, DeckSlice::Ordered(x) => x }).cloned().collect()
            )],
        }
    }

    pub fn draw(&self, dm: &mut dyn DecisionMaker) -> (Self, Option<Card>) {
        if self.is_empty() {
            (Self::default(), None)
        } else {
            match &self.slices[0] {
                DeckSlice::Ordered(x) => {
                    (
                        Deck {
                            slices: if x.len() > 1 {
                                let mut poop = vec![DeckSlice::Ordered(x[1..].iter().cloned().collect())];
                                poop.extend(self.slices[1..].iter().cloned());
                                poop
                            } else {
                                self.slices[1..].iter().cloned().collect()
                            }
                        },
                        Some(x[0].clone())
                    )
                },
                DeckSlice::Shuffled(x) => {
                    let index = dm.random_card(x.len());
                    let mut x = x.clone();
                    let card = x.remove(index);

                    (Deck {
                        slices: if !x.is_empty() {
                            let mut slices = vec![DeckSlice::Shuffled(x)];
                            slices.extend(self.slices[1..].iter().cloned());
                            slices
                        } else {
                            self.slices[1..].iter().cloned().collect()
                        }
                    }, Some(card))
                },
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.slices.is_empty()
    }

    pub fn len(&self) -> usize {
        self.slices.iter().map(|s| match s { DeckSlice::Ordered(x) => x.len(), DeckSlice::Shuffled(x) => x.len() }).sum()
    }

    pub fn put_on_top(&self, card: Card) -> Self {
        if self.is_empty() {
            Self {
                slices: vec![DeckSlice::Ordered(vec![card])],
            }
        } else {
            match &self.slices[0] {
                DeckSlice::Ordered(x) => {
                    let mut y = x.clone();
                    y.insert(0, card);
                    let mut slices = vec![DeckSlice::Ordered(y)];
                    slices.extend(self.slices[1..].iter().cloned());
                    Deck { slices }
                },
                DeckSlice::Shuffled(_) => {
                    let mut slices = vec![DeckSlice::Ordered(vec![card])];
                    slices.extend(self.slices.iter().cloned());
                    Deck { slices }
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub enum RotationalStatus {
    #[default]
    None,
    Asleep,
    Confused,
    Paralyzed,
}

#[derive(Clone)]
pub enum FaceCard {
    Up(Card),
    Down(Card),
}

impl FaceCard {
    pub fn up(&self) -> Self {
        match self {
            Self::Up(c) => Self::Up(c.clone()),
            Self::Down(c) => Self::Up(c.clone()),
        }
    }
}

#[derive(Default, Clone)]
pub struct InPlayCard {
    stack: Vec<FaceCard>,
    attached: Vec<FaceCard>,
    damage_counters: u32,
    rotational_status: RotationalStatus,
    poisoned: bool,
    burned: bool,
}

#[derive(Default)]
struct CLIDrawTarget {
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

trait CLIDrawable {
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
        self.stack[0].draw(x, y, target)
    }
}

#[derive(Default, Clone)]
struct PlayerSide {
    deck: Deck,
    hand: Vec<Card>,
    discard: Vec<Card>,
    lost_zone: Vec<Card>,
    prizes: Vec<FaceCard>,
    gx_available: bool,
    vstar_available: bool,
    active: Vec<InPlayCard>,
    bench: Vec<InPlayCard>,
    stadium: Option<Card>,
    supporter: Option<Card>,
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

#[derive(Default, Clone)]
enum GameStage {
    #[default]
    Uninitialized,
    Turn(Player),
}

#[derive(Default, Clone)]
struct GameState {
    p1: PlayerSide,
    p2: PlayerSide,
    // player effects, in play effects, etc

    // whose turn is it, what stage of the turn are we, etc
    stage: GameStage,
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
            GameStage::Turn(Player::One) => {
                target.draw_line("v", 3, 25);
            },
            GameStage::Turn(Player::Two) => {
                target.draw_line("^", 3, 24);
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
    }
}

impl GameState {
    pub fn initial(a: &[&str], b: &[&str]) -> Self {
        Self {
            p1: PlayerSide {
                deck: Deck::new(&a.iter().map(|x| x.to_string()).collect::<Vec<_>>()),
                ..Default::default()
            },
            p2: PlayerSide {
                deck: Deck::new(&b.iter().map(|x| x.to_string()).collect::<Vec<_>>()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn side(&self, player: Player) -> &PlayerSide {
        match player {
            Player::One => &self.p1,
            Player::Two => &self.p2,
        }
    }

    fn side_mut(&mut self, player: Player) -> &mut PlayerSide {
        match player {
            Player::One => &mut self.p1,
            Player::Two => &mut self.p2,
        }
    }

    pub fn with_player_side(&self, player: Player, side: PlayerSide) -> Self {
        match player {
            Player::One => Self { p1: side, ..self.clone() },
            Player::Two => Self { p2: side, ..self.clone() },
        }
    }

    pub fn put_on_top_of_deck(&self, player: Player, card: Card) -> Self {
        let side = self.side(player);

        self.with_player_side(player, PlayerSide {
            deck: side.deck.put_on_top(card),
            ..side.clone()
        })
    }

    pub fn shuffle_hand_into_deck(&self, player: Player) -> Self {
        let mut state = self.clone();
        while !state.side(player).hand.is_empty() {
            let card = state.side_mut(player).hand.pop().unwrap();
            state = state.put_on_top_of_deck(player, card.clone());
        }
        state.shuffle_deck(player)
    }

    pub fn shuffle_deck(&self, player: Player) -> Self {
        let side = self.side(player);

        self.with_player_side(player, PlayerSide {
            deck: side.deck.shuffle(),
            ..side.clone()
        })
    }

    pub fn draw_to_hand(&self, player: Player, dm: &mut dyn DecisionMaker) -> Self {
        let side = self.side(player);

        let (deck, card) = side.deck.draw(dm);
        let mut hand = side.hand.clone();
        if let Some(card) = card { hand.push(card); }

        self.with_player_side(player, PlayerSide { deck, hand, ..side.clone() })
    }

    pub fn draw_to_prizes(&self, player: Player, dm: &mut dyn DecisionMaker) -> Self {
        let side = self.side(player);

        let (deck, card) = side.deck.draw(dm);
        let mut prizes = side.prizes.clone();
        if let Some(card) = card { prizes.push(FaceCard::Down(card)); }

        self.with_player_side(player, PlayerSide { deck, prizes, ..side.clone() })
    }

    pub fn draw_n_to_hand(&self, player: Player, n: usize, dm: &mut dyn DecisionMaker) -> Self {
        if n == 0 {
            self.clone()
        } else {
            self.draw_to_hand(player, dm).draw_n_to_hand(player, n - 1, dm)
        }
    }

    pub fn play_from_hand_to_active_face_down(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.active.push(InPlayCard {
            stack: vec![FaceCard::Down(card.clone())],
            ..Default::default()
        });

        self.with_player_side(player, side)
    }

    pub fn play_from_hand_to_bench_face_down(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.bench.push(InPlayCard {
            stack: vec![FaceCard::Down(card.clone())],
            ..Default::default()
        });

        self.with_player_side(player, side)
    }

    pub fn reveal_pokemon(&self, player: Player) -> Self {
        let mut side = self.side(player).clone();

        for active in side.active.iter_mut() {
            active.stack[0] = active.stack[0].up();
        }

        for benched in side.bench.iter_mut() {
            benched.stack[0] = benched.stack[0].up();
        }

        self.with_player_side(player, side)
    }
}

struct GameEngine {
    state: GameState,
}

pub trait DecisionMaker {
    fn random_card(&mut self, n: usize) -> usize;
    fn confirm_setup_mulligan(&mut self, p: Player);
    fn confirm_setup_active_or_mulligan(&mut self, p: Player, maybe: &Vec<Card>) -> SetupActiveSelection;
    fn confirm_setup_active(&mut self, p: Player, yes: &Vec<Card>, maybe: &Vec<Card>) -> Card;
    fn confirm_mulligan_draw(&mut self, p: Player, upto: usize) -> usize;
    fn confirm_setup_bench_selection(&mut self, p: Player, cards: &Vec<Card>) -> Vec<Card>;
}

struct CLI {
}

impl DecisionMaker for CLI {
    fn random_card(&mut self, n: usize) -> usize {
        rand::thread_rng().gen_range(0..n)
    }

    fn confirm_setup_mulligan(&mut self, _p: Player) {
    }

    fn confirm_setup_active_or_mulligan(&mut self, _p: Player, _maybe: &Vec<Card>) -> SetupActiveSelection {
        SetupActiveSelection::Mulligan
    }

    fn confirm_setup_active(&mut self, _p: Player, yes: &Vec<Card>, _maybe: &Vec<Card>) -> Card {
        yes[0].clone()
    }

    fn confirm_mulligan_draw(&mut self, _p: Player, upto: usize) -> usize {
        upto
    }

    fn confirm_setup_bench_selection(&mut self, p: Player, cards: &Vec<Card>) -> Vec<Card> {
        cards.clone()
    }
}

#[derive(PartialEq, Eq)]
enum Maybe {
    Yes,
    No,
    Maybe,
}

#[derive(PartialEq, Eq)]
enum Stage {
    // Baby,
    Basic,
    // Break,
    // Legend,
    // LevelUp,
    // Mega,
    // Restored,
    Stage1,
    Stage2,
    // VStar,
    // VUnion,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SetupActiveSelection {
    Mulligan,
    Place(Card),
}

#[derive(Debug, Clone)]
pub enum Action {
}

impl GameEngine {
    pub fn play(&mut self, dm: &mut dyn DecisionMaker) {
        loop {
            CLIDrawTarget::print(&self.state);
            match self.state.stage {
                GameStage::Uninitialized => { self.setup(dm); },
                GameStage::Turn(player) => {
                    self.state = self.state.draw_to_hand(player, dm);

                    println!("{:?}", self.available_actions(player));
                    break;
                }
            }
        }
    }

    pub fn available_actions(&self, player: Player) -> Vec<Action> {
        // retreat
        // use trainer from hand
        // attach energy from hand
        // use ability actions
        // attack

        for card in self.state.side(player).hand.iter() {
            // can action?
            println!("can play {}? {}", card, self.can_be_played_from_hand(player, card));
        }

        vec![]
    }

    pub fn can_be_played_from_hand(&self, player: Player, card: &Card) -> bool {
        let opp = player.opponent();

        let my_deck = &self.state.side(player).deck;
        let my_bench = &self.state.side(player).bench;

        match card.as_str() {
            "Clefairy Doll (BS 70)" => self.can_bench(player, card),
            "Computer Search (BS 71)" => self.can_discard_other(player, card, 2),
            // "Devolution Spray (BS 72)" => mine.in_play.any(is_evolution),
            "Impostor Professor Oak (BS 73)" => true,
            "Item Finder (BS 74)" => self.can_discard_other(player, card, 2) && self.discard_pile_has_trainer(player, card),
            "Lass (BS 75)" => true,
            "Pokemon Breeder (BS 76)" => true, // TODO: bunch checks
            "Pokemon Trader (BS 77)" => true, // TODO: pokecomm
            "Scoop Up (BS 78)" => true,
            //"Super Energy Removal (BS 79)" => mine.in_play.any(energy_attached(1..)) && opp.in_play.any(energy_attached(1..)),
            "Defender (BS 80)" => true,
            "Energy Retrieval (BS 81)" => self.can_discard_other(player, card, 1) && self.discard_pile_has_basic_energy(player, card),
            //"Full Heal (BS 82)" => self.active.any(asleep|confused|paralyzed|poisoned),
            "Maintenance (BS 83)" => self.can_discard_other(player, card, 2), // not discard but shuffle
            "PlusPower (BS 84)" => true,
            //"Pokemon Center (BS 85)" => mine.in_play.any((has_damage_counters|energy_attached(1..))&can_damage_counters_be_removed),
            //"Pokemon Flute (BS 86)" => self.discard_pile_has_basic_pokemon(opp) && !opp.can_bench
            "Pokedex (BS 87)" => !my_deck.is_empty(),
            "Professor Oak (BS 88)" => !my_deck.is_empty(),
            //"Revive (BS 89)" => same as pokeflute but for ourselves,
            //"Super Potion (BS 90)" => mine.in_play.any(has_damage_counters&energy_attached(1..)),
            "Bill (BS 91)" => my_deck.len() > 0,
            //"Energy Removal (BS 92)" => opp.in_play.any(energy_attached(1..))
            "Gust of Wind (BS 93)" => self.state.side(opp).bench.len() > 0,
            //"Potion (BS 94)" => mine.in_play.any(has_damage_counters),
            "Switch (BS 95)" => !my_bench.is_empty(),

            _ => false
        }
    }

    pub fn can_bench(&self, player: Player, _card: &Card) -> bool {
        self.state.side(player).bench.len() < self.bench_size(player)
    }

    pub fn bench_size(&self, _player: Player) -> usize {
        5
    }

    pub fn can_discard_other(&self, player: Player, card: &Card, n: usize) -> bool {
        let this_card = self.state.side(player).hand.iter().filter(|c| *c == card).count();
        let other_cards = self.state.side(player).hand.iter().filter(|c| *c != card).count();

        this_card - 1 + other_cards >= n
    }

    pub fn discard_pile_has_trainer(&self, player: Player, _card: &Card) -> bool {
        self.state.side(player).discard.iter().find(|c| self.is_trainer(c)) != None
    }

    pub fn discard_pile_has_basic_energy(&self, player: Player, _card: &Card) -> bool {
        self.state.side(player).discard.iter().find(|c| self.is_basic_energy(c)) != None
    }

    pub fn setup(&mut self, dm: &mut dyn DecisionMaker) {
        let mut p1selection = SetupActiveSelection::Mulligan;
        let mut p2selection = SetupActiveSelection::Mulligan;

        while p1selection == SetupActiveSelection::Mulligan && p2selection == SetupActiveSelection::Mulligan {
            // 1. each player shuffles their deck
            self.state = self.state.shuffle_hand_into_deck(Player::One).shuffle_hand_into_deck(Player::Two);

            // 2. each player draws 7 cards
            self.state = self.state.draw_n_to_hand(Player::One, 7, dm).draw_n_to_hand(Player::Two, 7, dm);

            // 3. players pick a card to be their active pokemon (face down)
            p1selection = self.confirm_setup_selection(Player::One, dm);
            p2selection = self.confirm_setup_selection(Player::Two, dm);
        }

        // place selections
        if let SetupActiveSelection::Place(card) = &p1selection {
            self.state = self.state.play_from_hand_to_active_face_down(Player::One, card);
        }

        if let SetupActiveSelection::Place(card) = &p2selection {
            self.state = self.state.play_from_hand_to_active_face_down(Player::Two, card);
        }

        while p2selection == SetupActiveSelection::Mulligan {
            // p2 shuffles, draws 7, selects again
            self.state = self.state.shuffle_hand_into_deck(Player::Two).draw_n_to_hand(Player::Two, 7, dm);
            p2selection = self.confirm_setup_selection(Player::Two, dm);

            // p1 is asked to draw 0,1,2 cards
            let n = dm.confirm_mulligan_draw(Player::One, 2);
            self.state = self.state.draw_n_to_hand(Player::One, n, dm);
        }

        while p1selection == SetupActiveSelection::Mulligan {
            // p1 shuffles, draws 7, selects again
            self.state = self.state.shuffle_hand_into_deck(Player::One).draw_n_to_hand(Player::One, 7, dm);
            p1selection = self.confirm_setup_selection(Player::One, dm);

            // p2 is asked to draw 0,1,2 cards
            let n = dm.confirm_mulligan_draw(Player::Two, 2);
            self.state = self.state.draw_n_to_hand(Player::Two, n, dm);
        }

        if let SetupActiveSelection::Place(card) = &p1selection {
            if self.state.p1.active.is_empty() {
                self.state = self.state.play_from_hand_to_active_face_down(Player::One, card);
            }
        }

        if let SetupActiveSelection::Place(card) = &p2selection {
            if self.state.p2.active.is_empty() {
                self.state = self.state.play_from_hand_to_active_face_down(Player::Two, card);
            }
        }

        self.setup_bench(dm);
        self.setup_prizes(dm);
        // TODO: flip coin to decide who goes first, or check for First Ticket DRV 19.
        self.setup_reveal_pokemon();
        // TODO: check for abilities that activate on reveal, like Sableye SF 48

        self.state.stage = GameStage::Turn(Player::One);

        println!("Hand sizes: {}, {}", self.state.p1.hand.len(), self.state.p2.hand.len());
    }

    pub fn setup_bench(&mut self, dm: &mut dyn DecisionMaker) {
        let p1bench = self.confirm_bench_selection(Player::One, dm);
        let p2bench = self.confirm_bench_selection(Player::Two, dm);

        for card in p1bench {
            self.state = self.state.play_from_hand_to_bench_face_down(Player::One, &card);
        }
        for card in p2bench {
            self.state = self.state.play_from_hand_to_bench_face_down(Player::Two, &card);
        }
    }

    pub fn setup_prizes(&mut self, dm: &mut dyn DecisionMaker) {
        for _ in 0..6 {
            self.state = self.state.draw_to_prizes(Player::One, dm);
        }

        for _ in 0..6 {
            self.state = self.state.draw_to_prizes(Player::Two, dm);
        }
    }

    pub fn setup_reveal_pokemon(&mut self) {
        self.state = self.state.reveal_pokemon(Player::One);
        self.state = self.state.reveal_pokemon(Player::Two);
    }

    pub fn confirm_bench_selection(&self, player: Player, dm: &mut dyn DecisionMaker) -> Vec<Card> {
        let side = self.state.side(player);

        let benchable = side.hand.iter().filter(|c| self.placeable_as_benched_during_setup(c)).cloned().collect::<Vec<_>>();

        dm.confirm_setup_bench_selection(player, &benchable)
    }

    pub fn confirm_setup_selection(&self, player: Player, dm: &mut dyn DecisionMaker) -> SetupActiveSelection {
        let yes = self.state.side(player).hand.iter().filter(|c| self.placeable_as_active_during_setup(c) == Maybe::Yes).cloned().collect::<Vec<_>>();
        let maybe = self.state.side(player).hand.iter().filter(|c| self.placeable_as_active_during_setup(c) == Maybe::Maybe).cloned().collect::<Vec<_>>();

        println!("Player {:?}: {:?}", player, self.state.side(player).hand);
        println!("Player {:?}: Pick from {:?}, {:?}", player, yes, maybe);

        let selection = if yes.is_empty() && maybe.is_empty() {
            dm.confirm_setup_mulligan(player);
            SetupActiveSelection::Mulligan
        } else if yes.is_empty() {
            dm.confirm_setup_active_or_mulligan(player, &maybe)
        } else {
            SetupActiveSelection::Place(dm.confirm_setup_active(player, &yes, &maybe))
        };

        println!("Player {:?}: selected {:?}", player, selection);
        selection
    }

    pub fn placeable_as_active_during_setup(&self, card: &Card) -> Maybe {
        if card == "Mysterious Fossil (FO 62)" {
            Maybe::Maybe
        } else if self.stage(card) == Some(Stage::Basic) {
            Maybe::Yes
        } else {
            Maybe::No
        }
    }

    pub fn placeable_as_benched_during_setup(&self, card: &Card) -> bool {
        if card == "Mysterious Fossil (FO 62)" {
            true
        } else if self.stage(card) == Some(Stage::Basic) {
            true
        } else {
            false
        }
    }

    pub fn is_trainer(&self, card: &Card) -> bool {
        false
    }

    pub fn is_basic_energy(&self, card: &Card) -> bool {
        match card.as_str() {
            "Psychic Energy (BS  101)" => true,
            "Water Energy (BS  102)" => true,
            _ => false,
        }
    }

    pub fn stage(&self, card: &Card) -> Option<Stage> {
        match card.as_str() {
            "Psyduck (FO 53)" | "Voltorb (BS 67)" | "Growlithe (BS 28)" | "Gastly (FO 33)" => Some(Stage::Basic),
            "Squirtle (BS 63)" | "Articuno (FO 17)" => Some(Stage::Basic),
            "Electrode (BS 21)" | "Arcanine (BS 23)" | "Wartortle (BS 42)" => Some(Stage::Stage1),
            "Blastoise (BS 2)" => Some(Stage::Stage2),
            _ => None,
        }
    }
}


fn main() {
    let state = GameState::initial(
        &[
            "Psyduck (FO 53)", "Psyduck (FO 53)", "Psyduck (FO 53)", "Psyduck (FO 53)",
            "Voltorb (BS 67)", "Voltorb (BS 67)", "Voltorb (BS 67)", "Voltorb (BS 67)",
            "Electrode (BS 21)", "Electrode (BS 21)", "Electrode (BS 21)", "Electrode (BS 21)",
            "Growlithe (BS 28)", "Growlithe (BS 28)", "Growlithe (BS 28)",
            "Arcanine (BS 23)", "Arcanine (BS 23)", "Arcanine (BS 23)",
            "Gastly (FO 33)",
            "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)",
            "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)",
            "Item Finder (BS 74)", "Item Finder (BS 74)", "Item Finder (BS 74)", "Item Finder (BS 74)",
            "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)",
            "Lass (BS 75)", "Lass (BS 75)", "Lass (BS 75)",
            "Switch (BS 95)", "Switch (BS 95)",
            "Pokemon Trader (BS 77)", "Pokemon Trader (BS 77)",
            "Maintenance (BS 83)", "Maintenance (BS 83)",
            "Defender (BS 80)", "Defender (BS 80)",
            "Energy Removal (BS 92)", "Energy Removal (BS 92)",
            "PlusPower (BS 84)",
            "Gust of Wind (BS 93)",
            "Psychic Energy (BS 101)", "Psychic Energy (BS 101)", "Psychic Energy (BS 101)", "Psychic Energy (BS 101)",
            "Psychic Energy (BS 101)", "Psychic Energy (BS 101)",
            "Double Colorless Energy (BS 96)", "Double Colorless Energy (BS 96)", "Double Colorless Energy (BS 96)", "Double Colorless Energy (BS 96)",
        ],
        &[
            "Squirtle (BS 63)", "Squirtle (BS 63)", "Squirtle (BS 63)", "Squirtle (BS 63)",
            "Wartortle (BS 42)",
            "Blastoise (BS 2)", "Blastoise (BS 2)", "Blastoise (BS 2)",
            "Articuno (FO 17)", "Articuno (FO 17)", "Articuno (FO 17)", "Articuno (FO 17)",
            "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)",
            "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)",
            "Energy Retrieval (BS 81)", "Energy Retrieval (BS 81)", "Energy Retrieval (BS 81)", "Energy Retrieval (BS 81)",
            "Pokemon Breeder (BS 76)", "Pokemon Breeder (BS 76)", "Pokemon Breeder (BS 76)", "Pokemon Breeder (BS 76)",
            "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)",
            "Item Finder (BS 74)", "Item Finder (BS 74)", "Item Finder (BS 74)",
            "Maintenance (BS 83)", "Maintenance (BS 83)",
            "Super Energy Removal (BS 79)", "Super Energy Removal (BS 79)",
            "Super Potion (BS 90)", "Super Potion (BS 90)",
            "Switch (BS 95)", "Switch (BS 95)",
            "PlusPower (BS 84)",
            "Gust of Wind (BS 93)",
            "Lass (BS 75)",
            "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)",
            "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)",
            "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)",
            "Water Energy (BS 102)", "Water Energy (BS 102)",
        ],
    );

    let mut game = GameEngine { state };
    let mut cli = CLI {};

    game.play(&mut cli);
}
