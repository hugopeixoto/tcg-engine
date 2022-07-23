pub type Card = String;
pub type InPlayID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub enum RotationalStatus {
    #[default]
    None,
    Asleep,
    Confused,
    Paralyzed,
}

pub trait Shuffler {
    fn random_card(&mut self, n: usize) -> usize;
}


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

    pub fn draw(&self, dm: &mut dyn Shuffler) -> (Self, Option<Card>) {
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
    pub id: InPlayID,
    pub stack: Vec<FaceCard>,
    pub attached: Vec<FaceCard>,
    pub damage_counters: u32,
    pub rotational_status: RotationalStatus,
    pub poisoned: bool,
    pub burned: bool,
}

#[derive(Default, Clone)]
pub struct PlayerSide {
    pub deck: Deck,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
    pub lost_zone: Vec<Card>,
    pub prizes: Vec<FaceCard>,
    pub gx_available: bool,
    pub vstar_available: bool,
    pub active: Vec<InPlayCard>,
    pub bench: Vec<InPlayCard>,
    pub stadium: Option<Card>,
    pub supporter: Option<Card>,
}

impl PlayerSide {
    pub fn in_play_mut(&mut self, id: InPlayID) -> Option<&mut InPlayCard> {
        for p in self.active.iter_mut() {
            if p.id == id {
                return Some(p);
            }
        }

        for p in self.bench.iter_mut() {
            if p.id == id {
                return Some(p);
            }
        }

        None
    }
}

#[derive(Default, Clone)]
pub enum GameStage {
    #[default]
    Uninitialized,
    StartOfTurn(Player),
    Turn(Player),
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectTarget {
    Player(Player),
    InPlay(Player, InPlayID),
}

impl EffectTarget {
    pub fn is_player(&self, player: Player) -> bool {
        *(match self {
            Self::Player(p) => p,
            Self::InPlay(p, _) => p,
        }) == player
    }
}

#[derive(Clone, Debug)]
pub enum EffectSource {
    Ability(Player, InPlayID),
    Attack(Player, InPlayID),
    Trainer(Player, Card),
    Energy(Player, Card),
}

#[derive(Clone, Debug)]
pub enum EffectExpiration {
    DefendingPokemon, // ends on: switching, retreating, evolving, zone moving
    RestOfTheGame, // thanks ADP
    EndOfTurn(Player, usize), // 0: this turn; 1: next turn
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectConsequence {
    BlockAttachmentFromHand,
}

#[derive(Clone, Debug)]
pub struct Effect {
    pub source: EffectSource,
    pub target: EffectTarget,
    pub expires: EffectExpiration,
    pub consequence: EffectConsequence,
    pub name: String,
}

impl Effect {
    pub fn energy_attach_for_turn(effect: &Effect) -> bool {
        effect.name == "ENERGY_ATTACH_FOR_TURN"
    }
}

#[derive(Default, Clone)]
pub struct GameState {
    pub p1: PlayerSide,
    pub p2: PlayerSide,
    // player effects, in play effects, etc

    // whose turn is it, what stage of the turn are we, etc
    pub stage: GameStage,

    // effects
    pub effects: Vec<Effect>,
}
