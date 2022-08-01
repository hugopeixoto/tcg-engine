#[derive(Clone, PartialEq, Eq)]
pub struct Card {
    pub owner: Player,
    pub in_game_id: usize,
    pub archetype: String,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {:?}#{}", self.archetype, self.owner, self.in_game_id)
    }
}

pub type InPlayID = usize;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    #[default]
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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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

impl DeckSlice {
    pub fn remove_card(&self, card: &Card) -> (Option<Card>, Self) {
        match self {
            DeckSlice::Shuffled(cards) => {
                let mut cards = cards.clone();
                let p = cards.iter().position(|c| c == card);
                match p {
                    Some(p) => {
                        cards.remove(p);
                        (Some(card.clone()), DeckSlice::Shuffled(cards))
                    },
                    None => {
                        (None, DeckSlice::Shuffled(cards))
                    }
                }
            },
            DeckSlice::Ordered(cards) => {
                let mut cards = cards.clone();
                let p = cards.iter().position(|c| c == card);
                match p {
                    Some(p) => {
                        cards.remove(p);
                        (Some(card.clone()), DeckSlice::Ordered(cards))
                    },
                    None => {
                        (None, DeckSlice::Ordered(cards))
                    }
                }
            }
        }
    }
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

    pub fn cards(&self) -> Vec<Card> {
        self.slices.iter().flat_map(|s| match s { DeckSlice::Ordered(x) => x, DeckSlice::Shuffled(x) => x }).cloned().collect()
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

    pub fn remove_card(&self, card: &Card) -> (Option<Card>, Self) {
        for (i, slice) in self.slices.iter().enumerate() {
            if let (Some(c), slice) = slice.remove_card(card) {
                let mut deck = self.clone();
                deck.slices[i] = slice;
                return (Some(c), deck);
            }
        }

        (None, self.clone())
    }
}

#[derive(Clone, Debug)]
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

    pub fn card(&self) -> &Card {
        match self {
            Self::Up(c) => c,
            Self::Down(c) => c,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct InPlayCard {
    pub id: InPlayID,
    pub owner: Player,
    pub stack: Vec<FaceCard>,
    pub attached: Vec<FaceCard>,
    pub damage_counters: usize,
    pub rotational_status: RotationalStatus,
    pub poisoned: bool,
    pub burned: bool,
    pub put_in_play_turn: usize,
}

impl InPlayCard {
    pub fn cards(&self) -> Vec<&Card> {
        let mut cards = vec![];

        cards.extend(self.stack.iter().map(|c| c.card()));
        cards.extend(self.attached.iter().map(|c| c.card()));

        cards
    }
}

type PrizeCardID = usize;

#[derive(Clone)]
pub struct PrizeCard {
    pub id: PrizeCardID,
    pub card: FaceCard,
}

#[derive(Default, Clone)]
pub struct PlayerSide {
    pub deck: Deck,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
    pub lost_zone: Vec<Card>,
    pub prizes: Vec<PrizeCard>,
    pub gx_available: bool,
    pub vstar_available: bool,
    pub active: Vec<InPlayCard>,
    pub bench: Vec<InPlayCard>,
    pub stadium: Option<Card>,
    pub supporter: Option<Card>,
}

impl PlayerSide {
    pub fn in_play(&self, id: &InPlayID) -> Option<&InPlayCard> {
        for p in self.active.iter() {
            if p.id == *id {
                return Some(p);
            }
        }

        for p in self.bench.iter() {
            if p.id == *id {
                return Some(p);
            }
        }

        None
    }

    pub fn in_play_mut(&mut self, id: &InPlayID) -> Option<&mut InPlayCard> {
        for p in self.active.iter_mut() {
            if p.id == *id {
                return Some(p);
            }
        }

        for p in self.bench.iter_mut() {
            if p.id == *id {
                return Some(p);
            }
        }

        None
    }

    pub fn all_in_play(&self) -> Vec<&InPlayCard> {
        let mut cards = vec![];

        cards.extend(self.active.iter());
        cards.extend(self.bench.iter());

        cards
    }
}

#[derive(Debug, Default, Clone)]
pub enum GameStage {
    #[default]
    Uninitialized,
    StartOfTurn(Player),
    Turn(Player),
    PokemonCheckup(Player),
    Winner(Player),
    Tie,
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
    BlockTrainerFromHand,
    BlockDamage,
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
    pub turn: usize,

    // effects
    pub effects: Vec<Effect>,
}

impl GameState {
    pub fn initial(a: &[&str], b: &[&str]) -> Self {
        Self {
            p1: PlayerSide {
                deck: Deck::new(&a.iter().enumerate().map(|(n, x)| Card { owner: Player::One, in_game_id: n, archetype: x.to_string() }).collect::<Vec<_>>()),
                ..Default::default()
            },
            p2: PlayerSide {
                deck: Deck::new(&b.iter().enumerate().map(|(n, x)| Card { owner: Player::Two, in_game_id: a.len() + n, archetype: x.to_string() }).collect::<Vec<_>>()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn next_play_id(&self) -> InPlayID {
        let mut max = 0;

        for in_play in self.p1.active.iter() {
            max = max.max(in_play.id);
        }
        for in_play in self.p2.active.iter() {
            max = max.max(in_play.id);
        }
        for in_play in self.p1.bench.iter() {
            max = max.max(in_play.id);
        }
        for in_play in self.p2.bench.iter() {
            max = max.max(in_play.id);
        }

        max + 1
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

    fn put_on_top_of_deck(&self, player: Player, card: Card) -> Self {
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

    pub fn shuffle_from_hand_into_deck(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        self.with_player_side(player, side).put_on_top_of_deck(player, card.clone()).shuffle_deck(player)
    }

    pub fn shuffle_deck(&self, player: Player) -> Self {
        let side = self.side(player);

        self.with_player_side(player, PlayerSide {
            deck: side.deck.shuffle(),
            ..side.clone()
        })
    }

    pub fn draw_to_hand(&self, player: Player, dm: &mut dyn Shuffler) -> Self {
        let side = self.side(player);

        let (deck, card) = side.deck.draw(dm);
        let mut hand = side.hand.clone();
        if let Some(card) = card { hand.push(card); }

        self.with_player_side(player, PlayerSide { deck, hand, ..side.clone() })
    }

    pub fn draw_to_prizes(&self, player: Player, dm: &mut dyn Shuffler) -> Self {
        let side = self.side(player);

        let (deck, card) = side.deck.draw(dm);
        let mut prizes = side.prizes.clone();
        if let Some(card) = card { prizes.push(PrizeCard { id: prizes.len() + 1, card: FaceCard::Down(card) }); }

        self.with_player_side(player, PlayerSide { deck, prizes, ..side.clone() })
    }

    pub fn prize_to_hand(&self, player: Player, prize: &PrizeCard) -> Self {
        let mut side = self.side(player).clone();

        let p = side.prizes.iter().position(|c| c.id == prize.id).unwrap();
        let prize = side.prizes.remove(p);
        side.hand.push(prize.card.card().clone());

        self.with_player_side(player, side)
    }

    pub fn draw_n_to_hand(&self, player: Player, n: usize, dm: &mut dyn Shuffler) -> Self {
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
            id: self.next_play_id(),
            owner: player,
            stack: vec![FaceCard::Down(card.clone())],
            put_in_play_turn: 1.max(self.turn),
            ..Default::default()
        });

        self.with_player_side(player, side)
    }

    pub fn play_from_hand_to_bench_face_down(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.bench.push(InPlayCard {
            id: self.next_play_id(),
            owner: player,
            stack: vec![FaceCard::Down(card.clone())],
            put_in_play_turn: 1.max(self.turn),
            ..Default::default()
        });

        self.with_player_side(player, side)
    }

    pub fn attach_from_hand(&self, player: Player, card: &Card, target: &InPlayID) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.in_play_mut(target).unwrap().attached.push(FaceCard::Up(card.clone()));

        self.with_player_side(player, side)
    }

    pub fn evolve_from_hand(&self, player: Player, card: &Card, target: &InPlayID) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.in_play_mut(target).unwrap().stack.insert(0, FaceCard::Up(card.clone()));
        side.in_play_mut(target).unwrap().put_in_play_turn = self.turn;

        self.with_player_side(player, side)
    }

    pub fn bench_from_hand(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.bench.push(InPlayCard {
            id: self.next_play_id(),
            owner: player,
            stack: vec![FaceCard::Up(card.clone())],
            put_in_play_turn: 1.max(self.turn),
            ..Default::default()
        });

        self.with_player_side(player, side)
    }

    fn without_card(&self, card: &Card) -> Self {
        let mut state = self.clone();

        state.p1.discard.retain(|c| c != card);
        state.p1.hand.retain(|c| c != card);
        state.p1.lost_zone.retain(|c| c != card);
        state.p1.prizes.retain(|c| c.card.card() != card);
        state.p1.deck = state.p1.deck.remove_card(card).1;
        if state.p1.stadium == Some(card.clone()) {
            state.p1.stadium = None;
        }
        if state.p1.supporter == Some(card.clone()) {
            state.p1.supporter = None;
        }
        for active in state.p1.active.iter_mut() {
            active.stack.retain(|c| c.card() != card);
            active.attached.retain(|c| c.card() != card);
        }
        for benched in state.p1.bench.iter_mut() {
            benched.stack.retain(|c| c.card() != card);
            benched.attached.retain(|c| c.card() != card);
        }
        state.p1.active.retain(|c| !c.stack.is_empty());
        state.p1.bench.retain(|c| !c.stack.is_empty());

        state.p2.discard.retain(|c| c != card);
        state.p2.hand.retain(|c| c != card);
        state.p2.lost_zone.retain(|c| c != card);
        state.p2.prizes.retain(|c| c.card.card() != card);
        state.p2.deck = state.p2.deck.remove_card(card).1;
        if state.p2.stadium == Some(card.clone()) {
            state.p2.stadium = None;
        }
        if state.p2.supporter == Some(card.clone()) {
            state.p2.supporter = None;
        }
        for active in state.p2.active.iter_mut() {
            active.stack.retain(|c| c.card() != card);
            active.attached.retain(|c| c.card() != card);
        }
        for benched in state.p2.bench.iter_mut() {
            benched.stack.retain(|c| c.card() != card);
            benched.attached.retain(|c| c.card() != card);
        }
        state.p2.active.retain(|c| !c.stack.is_empty());
        state.p2.bench.retain(|c| !c.stack.is_empty());

        state
    }

    pub fn move_card_to_discard(&self, card: &Card) -> Self {
        let mut state = self.without_card(card); // broken_state

        state.side_mut(card.owner).discard.push(card.clone());

        state
    }

    pub fn move_card_to_hand(&self, card: &Card) -> Self {
        let mut state = self.without_card(card); // broken_state

        state.side_mut(card.owner).hand.push(card.clone());

        state
    }

    pub fn discard_from_hand(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        if let Some(p) = side.hand.iter().position(|c| c == card) {
            side.hand.remove(p);
            side.discard.push(card.clone());
        }

        self.with_player_side(player, side)
    }

    pub fn tutor_to_hand(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        if let (Some(c), deck) = side.deck.remove_card(card) {
            side.hand.push(c);
            side.deck = deck;
        }

        self.with_player_side(player, side)
    }

    pub fn discard_to_hand(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        if let Some(p) = side.discard.iter().position(|c| c == card) {
            side.discard.remove(p);
            side.hand.push(card.clone());
        }

        self.with_player_side(player, side)
    }

    pub fn switch_active_with(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        let benching = side.active.pop().unwrap();
        side.bench.push(benching);

        side.bench.retain(|x| x.id != in_play.id);
        side.active.push(in_play.clone());

        self.with_player_side(in_play.owner, side)
    }

    pub fn add_damage_counters(&self, in_play: &InPlayCard, counters: usize) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().damage_counters += counters;

        self.with_player_side(in_play.owner, side)
    }

    pub fn confuse(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().rotational_status = RotationalStatus::Confused;

        self.with_player_side(in_play.owner, side)
    }

    pub fn paralyze(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().rotational_status = RotationalStatus::Paralyzed;

        self.with_player_side(in_play.owner, side)
    }

    pub fn promote(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.bench.retain(|x| x.id != in_play.id);
        side.active.push(in_play.clone());

        self.with_player_side(in_play.owner, side)
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

    pub fn with_stage(&self, stage: GameStage) -> Self {
        Self {
            stage,
            ..self.clone()
        }
    }

    pub fn in_play(&self, id: &InPlayID) -> Option<&InPlayCard> {
        self.p1.in_play(id).or(self.p2.in_play(id))
    }

    pub fn all_in_play(&self) -> Vec<&InPlayCard> {
        let mut mons = vec![];

        mons.extend(self.p1.all_in_play());
        mons.extend(self.p2.all_in_play());

        mons
    }

    pub fn next_turn(&self) -> Self {
        Self {
            turn: self.turn + 1,
            ..self.clone()
        }
    }
}
