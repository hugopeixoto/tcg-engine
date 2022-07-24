use crate::state::*;
use crate::cli::CLIDrawTarget;

pub trait CardArchetype {
    // probably want to add the Zone of the card
    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action>;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState;
    fn stage(&self) -> Option<Stage>;
}

pub trait CardDB {
    fn archetype(&self) -> Box<dyn CardArchetype>;
}

pub struct GameEngine {
    pub state: GameState,
}

pub trait DecisionMaker: Shuffler {
    fn confirm_setup_mulligan(&mut self, p: Player);
    fn confirm_setup_active_or_mulligan(&mut self, p: Player, maybe: &Vec<Card>) -> SetupActiveSelection;
    fn confirm_setup_active(&mut self, p: Player, yes: &Vec<Card>, maybe: &Vec<Card>) -> Card;
    fn confirm_mulligan_draw(&mut self, p: Player, upto: usize) -> usize;
    fn confirm_setup_bench_selection(&mut self, p: Player, cards: &Vec<Card>) -> Vec<Card>;
    fn pick_action<'a>(&mut self, p: Player, actions: &'a Vec<Action>) -> &'a Action;
    fn pick_target<'a>(&mut self, p: Player, actions: &'a Vec<InPlayID>) -> &'a InPlayID;
    fn pick_from_hand<'a>(&mut self, p: Player, whose: Player, how_many: usize, hand: &'a Vec<Card>) -> Vec<&'a Card>;
    fn pick_from_discard<'a>(&mut self, p: Player, whose: Player, how_many: usize, discard: &Vec<Card>, searchable: &'a Vec<Card>) -> Vec<&'a Card>;
    fn pick_in_play<'a>(&mut self, p: Player, how_many: usize, searchable: &'a Vec<InPlayCard>) -> Vec<&'a InPlayCard>;
    fn search_deck<'a>(&mut self, p: Player, whose: Player, how_many: usize, deck: &'a Vec<Card>) -> Vec<&'a Card>;
}

#[derive(PartialEq, Eq)]
pub enum Maybe {
    Yes,
    No,
    Maybe,
}

#[derive(PartialEq, Eq)]
pub enum Stage {
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

#[derive(PartialEq, Eq)]
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
}

#[derive(PartialEq, Eq, Debug)]
pub enum SetupActiveSelection {
    Mulligan,
    Place(Card),
}

#[derive(Debug, Clone)]
pub enum Action {
    Pass,
    TrainerFromHand(Card),
    AttachFromHand(Card),
    BenchFromHand(Card),
}

impl GameEngine {
    pub fn play(&mut self, dm: &mut dyn DecisionMaker) {
        loop {
            CLIDrawTarget::print(&self.state);
            match self.state.stage {
                GameStage::Uninitialized => { self.setup(dm); },
                GameStage::Winner(_) => { break; },
                GameStage::Tie => { break; },
                GameStage::StartOfTurn(player) => {
                    if self.state.side(player).deck.is_empty() {
                        self.state = self.state.with_stage(GameStage::Winner(player.opponent()));
                    } else {
                        self.state = self.state.draw_to_hand(player, dm);
                        self.state = self.state.with_stage(GameStage::Turn(player));
                    }
                },
                GameStage::Turn(player) => {
                    println!("available actions for {:?}:", player);
                    let actions = self.available_actions(player);
                    for (i, action) in actions.iter().enumerate() {
                        println!(" {}. {:?}", i + 1, action);
                    }

                    let action = dm.pick_action(player, &actions);

                    match action {
                        Action::Pass => {
                            // count down end of turn effects
                            self.state = self.state.with_stage(GameStage::StartOfTurn(player.opponent()));

                            self.state.effects.retain(|e| match e.expires {
                                EffectExpiration::EndOfTurn(p, 0) => p != player,
                                _ => true,
                            });
                            for effect in self.state.effects.iter_mut() {
                                match effect.expires {
                                    EffectExpiration::EndOfTurn(p, t) => {
                                        if p == player {
                                            effect.expires = EffectExpiration::EndOfTurn(p, t - 1)
                                        }
                                    },
                                    _ => {},
                                }
                            }
                        },
                        Action::TrainerFromHand(card) => {
                            self.state = card.archetype().execute(player, card, self, dm);
                        },
                        Action::AttachFromHand(card) => {
                            self.state = card.archetype().execute(player, card, self, dm);
                        },
                        Action::BenchFromHand(card) => {
                            self.state = self.state.bench_from_hand(player, card);
                        },
                    }
                }
            }

            // Q. When both active Pokémon are knocked out, who places a new active first?
            // A. The player whose turn would be next.
            // https://compendium.pokegym.net/ruling/818/
            let who_first = match self.state.stage {
                GameStage::Turn(player) => player.opponent(),
                GameStage::StartOfTurn(player) => player.opponent(),
                _ => Player::One,
            };

            for who in [who_first, who_first.opponent()] {
                // TODO: 2v2 games
                while self.state.side(who).active.len() < 1 && !self.state.side(who).bench.is_empty() {
                    let chosen = dm.pick_in_play(who, 1, &self.state.side(who).bench);
                    self.state = self.state.promote(chosen[0]);
                }
            }

            // Well, if one of you has a Benched Pokémon to replace your Active Pokémon and
            // the other player doesn't, then the person who can replace his or her Active
            // Pokémon wins. Otherwise, you play Sudden Death. This is explained in the
            // Pokémon rules in the Expert Rules section under "What Happens if Both Players
            // Win at the Same Time?"
            // https://compendium.pokegym.net/ruling/882/
            let [a, b] = [Player::One, Player::Two].map(|player| {
                let prize_done = self.state.side(player).prizes.is_empty();
                let no_active = self.state.side(player.opponent()).active.is_empty();

                (if prize_done { 1 } else { 0 }) + (if no_active { 1 } else { 0 })
            });

            if a > 0 && a > b {
                self.state.stage = GameStage::Winner(Player::One);
            } else if b > 0 && b > a {
                self.state.stage = GameStage::Winner(Player::Two);
            } else if b > 0 && b == a {
                self.state.stage = GameStage::Tie;
            }
        }
    }

    pub fn available_actions(&self, player: Player) -> Vec<Action> {
        let mut actions = vec![];
        // retreat
        // use trainer from hand
        // attach energy from hand
        // use ability actions
        // attack

        for card in self.state.side(player).hand.iter() {
            actions.extend(self.card_actions(player, card));
        }

        actions.push(Action::Pass);

        actions
    }

    pub fn card_actions(&self, player: Player, card: &Card) -> Vec<Action> {
        if self.is_trainer(card) {
            return card.archetype().card_actions(player, card, self);
        } else if self.is_energy(card) {
            return card.archetype().card_actions(player, card, self);
        }

        if self.can_bench_from_hand(card) {
            return vec![Action::BenchFromHand(card.clone())]
        }

        vec![]
    }

    pub fn attachment_from_hand_targets(&self, player: Player, _card: &Card) -> Vec<InPlayID> {
        let blocks = self.state.effects.iter()
            .filter(|e| e.consequence == EffectConsequence::BlockAttachmentFromHand)
            .filter(|e| e.target.is_player(player))
            .collect::<Vec<_>>();

        let mut targets = vec![];

        for active in self.state.side(player).active.iter() {
            targets.push(active.id.clone());
        }

        for benched in self.state.side(player).bench.iter() {
            targets.push(benched.id.clone());
        }

        for block in blocks {
            match &block.target {
                EffectTarget::Player(_) => {
                    return vec![];
                },
                EffectTarget::InPlay(_, id) => {
                    targets.retain(|x| x != id);
                },
            }
        }

        targets
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
        if card.archetype == "Mysterious Fossil (FO 62)" {
            Maybe::Maybe
        } else if self.stage(card) == Some(Stage::Basic) {
            Maybe::Yes
        } else {
            Maybe::No
        }
    }

    pub fn placeable_as_benched_during_setup(&self, card: &Card) -> bool {
        if card.archetype == "Mysterious Fossil (FO 62)" {
            true
        } else if self.stage(card) == Some(Stage::Basic) {
            true
        } else {
            false
        }
    }

    pub fn can_bench_from_hand(&self, card: &Card) -> bool {
        if card.archetype == "Mysterious Fossil (FO 62)" {
            true
        } else if self.stage(card) == Some(Stage::Basic) {
            true
        } else {
            false
        }
    }

    pub fn is_trainer(&self, card: &Card) -> bool {
        match card.archetype.as_str() {
            "Clefairy Doll (BS 70)" => true,
            "Computer Search (BS 71)" => true,
            "Devolution Spray (BS 72)" => true,
            "Impostor Professor Oak (BS 73)" => true,
            "Item Finder (BS 74)" => true,
            "Lass (BS 75)" => true,
            "Pokemon Breeder (BS 76)" => true,
            "Pokemon Trader (BS 77)" => true,
            "Scoop Up (BS 78)" => true,
            "Super Energy Removal (BS 79)" => true,
            "Defender (BS 80)" => true,
            "Energy Retrieval (BS 81)" => true,
            "Full Heal (BS 82)" => true,
            "Maintenance (BS 83)" => true,
            "PlusPower (BS 84)" => true,
            "Pokemon Center (BS 85)" => true,
            "Pokemon Flute (BS 86)" => true,
            "Pokedex (BS 87)" => true,
            "Professor Oak (BS 88)" => true,
            "Revive (BS 89)" => true,
            "Super Potion (BS 90)" => true,
            "Bill (BS 91)" => true,
            "Energy Removal (BS 92)" => true,
            "Gust of Wind (BS 93)" => true,
            "Potion (BS 94)" => true,
            "Switch (BS 95)" => true,
            _ => false
        }
    }

    pub fn is_energy(&self, card: &Card) -> bool {
        self.is_basic_energy(card) || card.archetype == "Double Colorless Energy (BS 96)"
    }

    pub fn is_basic_energy(&self, card: &Card) -> bool {
        match card.archetype.as_str() {
            "Psychic Energy (BS 101)" => true,
            "Water Energy (BS 102)" => true,
            _ => false,
        }
    }

    pub fn stage(&self, card: &Card) -> Option<Stage> {
        match card.archetype.as_str() {
            "Psyduck (FO 53)" | "Voltorb (BS 67)" | "Growlithe (BS 28)" | "Gastly (FO 33)" => Some(Stage::Basic),
            "Squirtle (BS 63)" | "Articuno (FO 17)" => Some(Stage::Basic),
            "Electrode (BS 21)" | "Arcanine (BS 23)" | "Wartortle (BS 42)" => Some(Stage::Stage1),
            "Blastoise (BS 2)" => Some(Stage::Stage2),
            _ => None,
        }
    }
}
