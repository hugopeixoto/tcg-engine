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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Fighting,
    Fire,
    Grass,
    Lightning,
    Psychic,
    Water,
    Dark,
    Metal,
    Fairy,
    Dragon,
    Colorless,
    Any,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zone {
    Unknown,
    Hand(Player),
    Deck(Player),
    Prize(Player),
    Discard(Player),
    InPlay(Player),
    LostZone(Player),
    WorkingArea(Player),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poison {
    pub counters: usize,
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

    pub fn contains(&self, card: &Card) -> bool {
        match self {
            DeckSlice::Shuffled(cards) => cards.contains(card),
            DeckSlice::Ordered(cards) => cards.contains(card),
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

    pub fn contains(&self, card: &Card) -> bool {
        self.slices.iter().any(|slice| slice.contains(card))
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn is_up(&self) -> bool {
        match self {
            Self::Up(_c) => true,
            Self::Down(_c) => false,
        }
    }

    pub fn is_down(&self) -> bool {
        match self {
            Self::Up(_c) => false,
            Self::Down(_c) => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttachedCard {
    pub card: FaceCard,
    pub attached_turn: usize,
}

impl AttachedCard {
    pub fn card(&self) -> &Card {
        self.card.card()
    }

    pub fn is_up(&self) -> bool {
        self.card.is_up()
    }

    pub fn is_down(&self) -> bool {
        self.card.is_down()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct InPlayCard {
    pub id: InPlayID,
    pub owner: Player,
    pub stack: Vec<FaceCard>,
    pub attached: Vec<AttachedCard>,
    pub damage_counters: usize,
    pub rotational_status: RotationalStatus,
    pub poisoned: Option<Poison>,
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

    pub fn is_asleep(&self) -> bool {
        match self.rotational_status {
            RotationalStatus::Asleep => true,
            _ => false,
        }
    }
}

type PrizeCardID = usize;

#[derive(Clone)]
pub struct PrizeCard {
    pub id: PrizeCardID,
    pub card: FaceCard,
}

#[derive(Clone)]
pub struct PlayerSide {
    pub owner: Player,
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
    pub working_area: Vec<Card>,
    pub manual_attachments_this_turn: usize,
}

impl PlayerSide {
    pub fn new(cards: &[String], player: Player, base_id: usize) -> Self {
        Self {
            deck: Deck::new(&cards.iter().enumerate().map(|(n, x)| Card { owner: player, in_game_id: base_id + n, archetype: x.clone() }).collect::<Vec<_>>()),
            owner: player,
            hand: vec![],
            discard: vec![],
            lost_zone: vec![],
            prizes: vec![],
            gx_available: true,
            vstar_available: true,
            active: vec![],
            bench: vec![],
            stadium: None,
            supporter: None,
            working_area: vec![],
            manual_attachments_this_turn: 0,
        }
    }

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

    pub fn all_cards(&self) -> Vec<Card> {
        let mut cards = vec![];

        cards.extend(self.deck.cards());
        cards.extend(self.hand.clone());
        cards.extend(self.discard.clone());
        cards.extend(self.lost_zone.clone());
        cards.extend(self.working_area.clone());
        cards.extend(self.prizes.iter().map(|c| c.card.card()).cloned());
        if let Some(card) = &self.supporter { cards.push(card.clone()); }
        if let Some(card) = &self.stadium { cards.push(card.clone()); }

        for in_play in self.active.iter() {
            cards.extend(in_play.stack.iter().map(|c| c.card()).cloned());
            cards.extend(in_play.attached.iter().map(|c| c.card()).cloned());
        }

        for in_play in self.bench.iter() {
            cards.extend(in_play.stack.iter().map(|c| c.card()).cloned());
            cards.extend(in_play.attached.iter().map(|c| c.card()).cloned());
        }

        cards
    }

    pub fn zone(&self, card: &Card) -> Zone {
        if self.discard.contains(card) {
            Zone::Discard(self.owner)
        } else if self.hand.contains(card) {
            Zone::Hand(self.owner)
        } else if self.lost_zone.contains(card) {
            Zone::LostZone(self.owner)
        } else if self.prizes.iter().any(|p| p.card.card() == card) {
            Zone::Prize(self.owner)
        } else if self.deck.contains(card) {
            Zone::Deck(self.owner)
        } else if self.stadium.as_ref() == Some(card) {
            Zone::InPlay(self.owner)
        } else if self.supporter.as_ref() == Some(card) {
            Zone::InPlay(self.owner)
        } else if self.active.iter().any(|p| p.stack.iter().any(|c| c.card() == card) || p.attached.iter().any(|c| c.card() == card)) {
            Zone::InPlay(self.owner)
        } else if self.bench.iter().any(|p| p.stack.iter().any(|c| c.card() == card) || p.attached.iter().any(|c| c.card() == card)) {
            Zone::InPlay(self.owner)
        } else if self.working_area.contains(card) {
            Zone::WorkingArea(self.owner)
        } else {
            Zone::Unknown
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameStage {
    Uninitialized,
    StartOfTurn(Player),
    Turn(Player),
    EndOfTurn(Player),
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

    pub fn is_in_play(&self, in_play: &InPlayCard) -> bool {
        match self {
            Self::InPlay(_, ip) => *ip == in_play.id,
            _ => false,
        }
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

pub type EffectConsequence = String;

#[derive(Clone, Debug)]
pub enum EffectParameter {
    Type(Type),
    String(String),
}

impl EffectParameter {
    pub fn get_type(&self) -> Option<Type> {
        match self {
            Self::Type(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(v) => Some(v.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Effect {
    pub source: EffectSource,
    pub target: EffectTarget,
    pub expires: EffectExpiration,
    pub consequence: EffectConsequence,
    pub parameters: Vec<EffectParameter>,
    pub name: String,
}

impl Effect {
    pub fn get_parameter_type(&self, index: usize) -> Option<Type> {
        self
            .parameters
            .iter()
            .flat_map(|p| p.get_type())
            .nth(index)
    }

    pub fn get_parameter_string(&self, index: usize) -> Option<String> {
        self
            .parameters
            .iter()
            .flat_map(|p| p.get_string())
            .nth(index)
    }
}

#[derive(Clone)]
pub struct GameState {
    pub p1: PlayerSide,
    pub p2: PlayerSide,

    // whose turn is it, what stage of the turn are we, etc
    pub stage: GameStage,
    pub turn: usize,
    pub turns: Vec<Player>,

    // effects
    pub effects: Vec<Effect>,
}

impl GameState {
    pub fn initial(a: &[String], b: &[String]) -> Self {
        Self {
            p1: PlayerSide::new(a, Player::One, 0),
            p2: PlayerSide::new(b, Player::Two, a.len()),
            stage: GameStage::Uninitialized,
            turn: 0,
            turns: vec![],
            effects: vec![],
        }
    }

    fn next_play_id(&self) -> InPlayID {
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

    fn with_player_side(&self, side: PlayerSide) -> Self {
        match side.owner {
            Player::One => Self { p1: side, ..self.clone() },
            Player::Two => Self { p2: side, ..self.clone() },
        }
    }

    fn put_on_top_of_deck(&self, player: Player, card: Card) -> Self {
        let side = self.side(player);

        self.with_player_side(PlayerSide {
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

        self.with_player_side(side).put_on_top_of_deck(player, card.clone()).shuffle_deck(player)
    }

    pub fn shuffle_deck(&self, player: Player) -> Self {
        let side = self.side(player);

        self.with_player_side(PlayerSide {
            deck: side.deck.shuffle(),
            ..side.clone()
        })
    }

    pub fn draw_to_hand(&self, player: Player, dm: &mut dyn Shuffler) -> Self {
        let side = self.side(player);

        let (deck, card) = side.deck.draw(dm);
        let mut hand = side.hand.clone();
        if let Some(card) = card { hand.push(card); }

        self.with_player_side(PlayerSide { deck, hand, ..side.clone() })
    }

    pub fn draw_to_working_area(&self, player: Player, dm: &mut dyn Shuffler) -> Self {
        let mut side = self.side(player).clone();
        let (deck, card) = side.deck.draw(dm);

        if let Some(card) = card {
            side.working_area.push(card);
            side.deck = deck;
        }

        self.with_player_side(side)
    }

    pub fn put_working_area_on_top_of_deck(&self, player: Player) -> Self {
        let mut side = self.side(player).clone();

        while let Some(card) = side.working_area.pop() {
            side.deck = side.deck.put_on_top(card);
        }

        self.with_player_side(side)
    }

    pub fn rearrange_working_area(&self, player: Player, cards: &Vec<&Card>) -> Self {
        let mut side = self.side(player).clone();

        side.working_area = cards.iter().cloned().cloned().collect();

        self.with_player_side(side)
    }

    pub fn draw_to_prizes(&self, player: Player, dm: &mut dyn Shuffler) -> Self {
        let side = self.side(player);

        let (deck, card) = side.deck.draw(dm);
        let mut prizes = side.prizes.clone();
        if let Some(card) = card { prizes.push(PrizeCard { id: prizes.len() + 1, card: FaceCard::Down(card) }); }

        self.with_player_side(PlayerSide { deck, prizes, ..side.clone() })
    }

    pub fn prize_to_hand(&self, player: Player, prize: &PrizeCard) -> Self {
        let mut side = self.side(player).clone();

        let p = side.prizes.iter().position(|c| c.id == prize.id).unwrap();
        let prize = side.prizes.remove(p);
        side.hand.push(prize.card.card().clone());

        self.with_player_side(side)
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
            put_in_play_turn: 0,
            ..Default::default()
        });

        self.with_player_side(side)
    }

    pub fn play_from_hand_to_bench_face_down(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.bench.push(InPlayCard {
            id: self.next_play_id(),
            owner: player,
            stack: vec![FaceCard::Down(card.clone())],
            put_in_play_turn: 0,
            ..Default::default()
        });

        self.with_player_side(side)
    }

    pub fn manual_attach_from_hand(&self, player: Player, card: &Card, target: &InPlayID) -> Self {
        let mut side = self.side(player).clone();
        side.manual_attachments_this_turn += 1;

        self
            .with_player_side(side)
            .attach_from_hand(player, card, target)
    }

    pub fn attach_from_hand(&self, player: Player, card: &Card, target: &InPlayID) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.in_play_mut(target).unwrap().attached.push(AttachedCard { card: FaceCard::Up(card.clone()), attached_turn: self.turn });

        self.with_player_side(side)
    }

    pub fn evolve_from_hand(&self, player: Player, card: &Card, target: &InPlayID) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.in_play_mut(target).unwrap().stack.insert(0, FaceCard::Up(card.clone()));
        side.in_play_mut(target).unwrap().put_in_play_turn = self.turn;

        self.with_player_side(side)
    }

    pub fn bench_from_hand(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.hand.iter().position(|c| c == card).unwrap();
        side.hand.remove(p);

        side.bench.push(InPlayCard {
            id: self.next_play_id(),
            owner: player,
            stack: vec![FaceCard::Up(card.clone())],
            put_in_play_turn: self.turn,
            ..Default::default()
        });

        self.with_player_side(side)
    }

    pub fn bench_from_discard(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        let p = side.discard.iter().position(|c| c == card).unwrap();
        side.discard.remove(p);

        side.bench.push(InPlayCard {
            id: self.next_play_id(),
            owner: player,
            stack: vec![FaceCard::Up(card.clone())],
            put_in_play_turn: self.turn,
            ..Default::default()
        });

        self.with_player_side(side)
    }

    fn without_card(&self, card: &Card) -> Self {
        let mut state = self.clone();

        state.p1.discard.retain(|c| c != card);
        state.p1.hand.retain(|c| c != card);
        state.p1.lost_zone.retain(|c| c != card);
        state.p1.working_area.retain(|c| c != card);
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
        state.p2.working_area.retain(|c| c != card);
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

    pub fn zone(&self, card: &Card) -> Zone {
        let zone = self.p1.zone(card);

        if zone != Zone::Unknown {
            zone
        } else {
            self.p2.zone(card)
        }
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

        self.with_player_side(side)
    }

    pub fn tutor_to_hand(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        if let (Some(c), deck) = side.deck.remove_card(card) {
            side.hand.push(c);
            side.deck = deck;
        }

        self.with_player_side(side)
    }

    pub fn discard_to_hand(&self, player: Player, card: &Card) -> Self {
        let mut side = self.side(player).clone();

        if let Some(p) = side.discard.iter().position(|c| c == card) {
            side.discard.remove(p);
            side.hand.push(card.clone());
        }

        self.with_player_side(side)
    }

    pub fn switch_active_with(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        let benching = side.active.pop().unwrap();
        side.bench.push(benching);

        side.bench.retain(|x| x.id != in_play.id);
        side.active.push(in_play.clone());

        self.with_player_side(side)
    }

    pub fn add_damage_counters(&self, in_play: &InPlayCard, counters: usize) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().damage_counters += counters;

        self.with_player_side(side)
    }

    pub fn remove_damage_counters(&self, in_play: &InPlayCard, counters: usize) -> Self {
        let mut side = self.side(in_play.owner).clone();

        let c = &mut side.in_play_mut(&in_play.id).unwrap().damage_counters;
        *c = c.saturating_sub(counters);

        self.with_player_side(side)
    }

    pub fn remove_all_damage_counters(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().damage_counters = 0;

        self.with_player_side(side)
    }

    pub fn confuse(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().rotational_status = RotationalStatus::Confused;

        self.with_player_side(side)
    }

    pub fn paralyze(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().rotational_status = RotationalStatus::Paralyzed;

        self.with_player_side(side)
    }

    pub fn poison(&self, in_play: &InPlayCard, counters: usize) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().poisoned = Some(Poison { counters });

        self.with_player_side(side)
    }

    pub fn asleep(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.in_play_mut(&in_play.id).unwrap().rotational_status = RotationalStatus::Asleep;

        self.with_player_side(side)
    }

    pub fn promote(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        side.bench.retain(|x| x.id != in_play.id);
        side.active.push(in_play.clone());

        self.with_player_side(side)
    }

    pub fn reveal_pokemon(&self, player: Player) -> Self {
        let mut side = self.side(player).clone();

        for active in side.active.iter_mut() {
            active.stack[0] = active.stack[0].up();
        }

        for benched in side.bench.iter_mut() {
            benched.stack[0] = benched.stack[0].up();
        }

        self.with_player_side(side)
    }

    pub fn remove_special_conditions(&self, in_play: &InPlayCard) -> Self {
        let mut side = self.side(in_play.owner).clone();

        let mut in_play = side.in_play_mut(&in_play.id).unwrap();
        in_play.rotational_status = RotationalStatus::None;
        in_play.poisoned = None;
        in_play.burned = false;

        self.with_player_side(side)
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

    pub fn attached_card(&self, card: &Card) -> Option<&AttachedCard> {
        for in_play in self.all_in_play() {
            for attached in in_play.attached.iter() {
                if attached.card() == card {
                    return Some(attached);
                }
            }
        }

        None
    }

    pub fn all_in_play(&self) -> Vec<&InPlayCard> {
        let mut mons = vec![];

        mons.extend(self.p1.all_in_play());
        mons.extend(self.p2.all_in_play());

        mons
    }

    pub fn all_cards(&self) -> Vec<Card> {
        let mut cards = vec![];

        cards.extend(self.p1.all_cards());
        cards.extend(self.p2.all_cards());

        cards
    }

    pub fn next_turn(&self, player: Player) -> Self {
        let mut turns = self.turns.clone();
        turns.push(player);

        let mut p1 = self.side(Player::One).clone();
        let mut p2 = self.side(Player::Two).clone();

        p1.manual_attachments_this_turn = 0;
        p2.manual_attachments_this_turn = 0;

        Self {
            p1,
            p2,
            turn: self.turn + 1,
            turns,
            ..self.clone()
        }
    }
}
