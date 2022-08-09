#!/usr/bin/env ruby

# Converts a pokemontcg.io json file representing a full TCG set into a rust file.

require 'pp'
require 'json'
require 'erb'

PRELUDE = <<~EOF
use crate::*;
use crate::carddb::Attacks;

EOF

TEMPLATE = <<~EOF
#[derive(Default)]
pub struct <%= card.struct_name %> {}
impl CardArchetype for <%= card.struct_name %> {
    card_name!("<%= card.display_name %>");
    <%= card.stage %>;
    hp!(<%= card.hp %>);
    color!(<%= card.types.join(', ') %>);
    <%= card.weakness %>;
    <%= card.resistance %>;
    retreat!(<%= card.retreat %>);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
<% card.attacks.each do |attack| -%>
            .register("<%= attack.name %>", &[<%= attack.cost.map{|c| "Type::\#{c}"}.join(", ") %>], Self::<%= attack.fn_name %>)
<% end -%>
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl <%= card.struct_name %> {
<% card.attacks.each do |attack| -%>
    pub fn <%= attack.fn_name %>(<%= attack.impl.match?(/\\bengine\\b/) ? "engine" : "_engine"  %>: &GameEngine, <%= attack.impl.match?(/\\bdm\\b/) ? "dm" : "_dm" %>: &mut dyn DecisionMaker) -> GameEngine {
        <%= attack.impl %>
    }
<% end -%>
}

EOF

def status_to_verb(status)
  {
    'asleep' => 'asleep',
    'confused' => 'confuse',
    'paralyzed' => 'paralyze',
    'poisoned' => 'poison',
  }[status]
end

class Builder
  def initialize(engine_name="engine")
    @text = []
    @is_engining = false
    @engine_name = engine_name
  end

  def self.unimplemented
    Builder.new.unimplemented
  end

  def unimplemented
    @text << "unimplemented!();"
    self
  end

  def self.engine
    Builder.new.engine
  end

  def self.flip_a_coin
    Builder.new.flip_a_coin
  end

  def self.flip_coins(n)
    Builder.new.flip_coins(n)
  end

  def engine
    @text << @engine_name
    @is_engining = true
    self
  end

  def inject_engine!
    engine if !@is_engining
    self
  end

  def damage(amount)
    if !amount.to_s.empty?
      inject_engine!
      @text << ".damage(#{amount})"
    end
    self
  end

  def flip_a_coin
    @text << "let heads = dm.flip(1).heads() == 1;"
    self
  end

  def flip_coins(n)
    @text << "let flip_results = dm.flip(#{n});"
    self
  end

  def inflict(affliction)
    inject_engine!
    @text << ".#{affliction}()"
    self
  end

  def severe_poison(counters)
    inject_engine!
    @text << ".severe_poison(#{counters})"
    self
  end

  def if_heads(&what)
    inject_engine!
    @text << ".then_if(heads, |e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def if_tails(&what)
    inject_engine!
    @text << ".then_if(!heads, |e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def damage_itself(damage)
    inject_engine!
    @text << ".damage_self(#{damage})"
    self
  end

  def damage_per_heads(damage)
    inject_engine!
    @text << ".damage(#{damage} * flip_results.heads())"
    self
  end

  def discard_attached_energies(energy_requirements)
    inject_engine!

    energies = energy_requirements.map { |e| "Type::#{e}" }.join(", ")

    @text << ".discard_attached_energies(#{@engine_name}.player(), #{@engine_name}.attacking(), &[#{energies}], dm)"
    self
  end

  def discard_all_attached_energies
    inject_engine!
    @text << ".discard_all_attached_energies(#{@engine_name}.player(), #{@engine_name}.attacking(), dm)"
    self
  end

  def each_own_benched(&what)
    inject_engine!
    @text << ".then(|e| e.target_all(e.bench(e.player()), |e2| #{Builder.new("e2").instance_eval(&what).to_s(compact: true)}))"
    self
  end

  def each_opponents_benched(&what)
    inject_engine!
    @text << ".then(|e| e.target_all(e.bench(e.opponent()), |e2| #{Builder.new("e2").instance_eval(&what).to_s(compact: true)}))"
    self
  end

  def prevent_damage_during_opponents_next_turn
    inject_engine!
    @text << <<~EOF
        .with_effect(Effect {
                name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
                source: EffectSource::Attack(#{@engine_name}.player(), #{@engine_name}.attacking().id),
                target: EffectTarget::InPlay(#{@engine_name}.player(), #{@engine_name}.attacking().id),
                consequence: EffectConsequence::BlockDamage,
                expires: EffectExpiration::EndOfTurn(#{@engine_name}.opponent(), 0),
                system: false,
            })
    EOF
    self
  end


  def damage_minus_per_damage_counter_on_itself(base_damage, minus)
    inject_engine!
    @text << ".damage(#{base_damage}usize.saturating_sub(#{@engine_name}.damage_counters_on(#{@engine_name}.attacking()) * #{minus}))"
    self
  end

  def to_s(compact: false)
    if compact
      @text.join
    else
      @text.map { |x| if x.start_with?(".") then "    #{x}\n" else "#{x}\n" end }.join
    end
  end
end

def attack_impl(attack)
  text = attack.fetch('text')
  damage = attack.fetch('damage', "")

  if text.empty?
    Builder.new.damage(damage)
  elsif text =~ /^The Defending Pokémon is now (Asleep|Paralyzed|Confused|Poisoned)\.$/
    status = status_to_verb($1.downcase)
    Builder.new.damage(damage).inflict(status)
  elsif text =~ /^The Defending Pokémon is now Poisoned\. It now takes (\w+) Poison damage instead of 10 after each player's turn \(even if it was already Poisoned\)\.$/
    counters = $1.to_i / 10
    Builder.new.damage(damage).severe_poison(counters)
  elsif text =~ /^Flip a coin\. If heads, the Defending Pokémon is now (Asleep|Paralyzed|Confused|Poisoned)\.$/
    inflict = status_to_verb($1.downcase)
    Builder.new.flip_a_coin.damage(damage).if_heads{ inflict(inflict) }
  elsif text =~ /^Flip a coin\. If heads, the Defending Pokémon is now (Asleep|Paralyzed|Confused|Poisoned)\; if tails, it is now (Asleep|Paralyzed|Confused|Confused)\.$/
    inflict_yes = status_to_verb($1.downcase)
    inflict_no = status_to_verb($2.downcase)
    Builder.new.flip_a_coin.damage(damage).if_heads { inflict(inflict_yes) }.if_tails { inflict(inflict_no) }
  elsif text =~ /^Flip a coin\. If tails, \w+ does (\d+) damage to itself\.$/
    self_damage = $1.to_i
    Builder.new.flip_a_coin.damage(damage).if_tails { damage_itself(self_damage) }
  elsif text =~ /^Flip (\d+) coins\. This attack does (\d+) damage times the number of heads\./
    coins = $1.to_i
    damage = $2.to_i
    Builder.new.flip_coins(coins).damage_per_heads(damage)
  elsif text =~ /^Does (\d+) damage minus (\d+) damage for each damage counter on \w+\.$/
    base_damage = $1.to_i
    minus_damage = $2.to_i

    Builder.new.damage_minus_per_damage_counter_on_itself(base_damage, minus_damage)
  elsif text =~ /^\w+ does (\d+) damage to itself.$/
    damage_self = $1.to_i
    Builder.new.damage(damage).damage_itself(damage_self)
  elsif text =~ /^Flip a coin\. If heads, prevent all damage done to (.+) during your opponent's next turn\. \(Any other effects of attacks still happen\.\)$/
    Builder
      .flip_a_coin
      .if_heads { prevent_damage_during_opponents_next_turn }
  elsif text =~ /^Discard 1 (Fire|Water) Energy card attached to \w+ in order to use this attack\.$/
    energy_type = $1
    Builder.new
      .discard_attached_energies([energy_type])
      .damage(damage)
  elsif text =~ /^Discard all Energy cards attached to \w+ in order to use this attack\.$/
    Builder.new
      .discard_all_attached_energies
      .damage(damage)
  elsif text =~ /^Flip a coin\. If heads, this attack does (\d+) damage plus (\d+) more damage; if tails, this attack does (\d+) damage (?:plus|and) \w+ does (\d+) damage to itself\.$/
    damage_base1 = $1.to_i
    damage_extra = $2.to_i
    damage_base2 = $3.to_i
    damage_self = $4.to_i

    Builder.new.flip_a_coin
      .if_heads{ damage(damage_base1 + damage_extra) }
      .if_tails{ damage(damage_base2).damage_itself(damage_self) }
  elsif text =~ /^Does (\d+) damage to each of your own Benched Pokémon\. \(Don't apply Weakness and Resistance for Benched Pokémon\.\)$/
    bench_damage = $1
    Builder.new
      .damage(damage)
      .each_own_benched { damage(bench_damage) }
  elsif text =~ /^Does (\d+) damage to each Pokémon on each player's Bench\. \(Don't apply Weakness and Resistance for Benched Pokémon.\) \w+ does (\d+) damage to itself\.$/
    bench_damage = $1.to_i
    self_damage = $2.to_i
    Builder.new
      .damage(damage)
      .each_own_benched { damage(bench_damage) }
      .each_opponents_benched { damage(bench_damage) }
      .damage_itself(self_damage)
  else
    Builder.unimplemented
  end
    .to_s.rstrip.gsub("\n", "\n        ")
end

def idify(name)
  name
    .gsub(' ', '_')
    .gsub('♂', 'm')
    .gsub("'d", "_d")
    .gsub("-", "_")
    .downcase
end

def namify(name)
  name
    .gsub(' ', '')
    .gsub('♂', 'M')
    .gsub("'d", "D")
end

output = ""
output << PRELUDE

cards = JSON.parse(File.read("base1.json"))['data']
pokemon_cards = cards.filter { |c| c["supertype"] == "Pokémon" }
pokemon_cards.each do |card|
  stage = if card["subtypes"].include?("Basic")
            "basic!()"
          elsif card["subtypes"].include?("Stage 1")
            'stage1!("' + card["evolvesFrom"] + '")'
          elsif card["subtypes"].include?("Stage 2")
            'stage2!("' + card["evolvesFrom"] + '")'
          else
            1/0
          end

  weakness = if card["weaknesses"]
               "weak_to!(#{card["weaknesses"].map{|w|w["type"]}.join(", ")})"
             else
               "no_weakness!()";
             end
  resistance = if card["resistances"]
                 "resists!(#{card["resistances"].map{|w|w["type"]}.join(", ")}, 30)";
               else
                 "no_resistance!()";
               end

  card = OpenStruct.new(
    struct_name: namify(card["name"]),
    display_name: card["name"],
    stage: stage,
    weakness: weakness,
    resistance: resistance,
    retreat: card.fetch("retreatCost", []).size,
    types: card["types"],
    hp: card["hp"],
    attacks: card["attacks"].map { |a|
      OpenStruct.new(a.merge(
        fn_name: idify(a["name"]),
        impl: attack_impl(a),
      ))
    },
  )

  output << ERB.new(TEMPLATE, trim_mode: "<>-").result_with_hash(card: card)
end

puts output.strip.gsub(/^ +$/, '')

poops = pokemon_cards.flat_map { |card| card.fetch("attacks", []) }.filter do |attack|
  attack_impl(attack) == "unimplemented!();"
end.map do |attack|
  [attack["text"].gsub(/\d+/, "X").gsub(/\([^)]+\)/, '').split(". "), attack["damage"]]
end

PP.pp poops.sort, out=$stderr
