#!/usr/bin/env ruby

# Converts a pokemontcg.io json file representing a full TCG set into a rust file.

require 'pp'
require 'json'
require 'erb'

PRELUDE = <<~EOF
use crate::*;
use crate::attack_builder::AttackBuilder;
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
            .register("<%= attack.name %>", Self::<%= attack.fn_name %>)
<% end -%>
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl <%= card.struct_name %> {
<% card.attacks.each do |attack| -%>
    pub fn <%= attack.fn_name %>(<%= attack.impl.include?("builder") ? "" : "_" %>builder: AttackBuilder) -> AttackBuilder {
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
  def initialize(engine_name="builder")
    @text = [engine_name]
    @is_engining = false
    @engine_name = engine_name
  end

  def damage(amount)
    if !amount.to_s.empty?
      @text << ".damage(#{amount})"
    end
    self
  end

  def flip_a_coin
    @text << ".flip_a_coin()"
    self
  end

  def flip_coins(n)
    @text << ".flip_coins(#{n})"
    self
  end

  def inflict(affliction)
    @text << ".#{affliction}()"
    self
  end

  def severe_poison(counters)
    @text << ".severe_poison(#{counters})"
    self
  end

  def if_heads(&what)
    @text << ".if_heads(|e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def if_tails(&what)
    @text << ".if_tails(|e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def if_did_damage(&what)
    @text << ".if_did_damage(|e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def damage_itself(damage)
    @text << ".damage_self(#{damage})"
    self
  end

  def damage_per_heads(damage)
    @text << ".damage_per_heads(#{damage})"
    self
  end

  def discard_attacking_energy_cards(energy_requirements)
    energies = energy_requirements.map { |e| "Type::#{e}" }.join(", ")

    @text << ".discard_attacking_energy_cards(&[#{energies}])"
    self
  end

  def discard_defending_attached_energy_cards(energy_requirements)
    energies = energy_requirements.map { |e| "Type::#{e}" }.join(", ")

    @text << ".discard_defending_energy_cards(&[#{energies}])"
    self
  end

  def discard_all_attacking_energy_cards
    @text << ".discard_all_attacking_energy_cards()"
    self
  end

  def attacking
    @text << ".attacking()"
    self
  end

  def heal_all_attacking
    @text << ".heal_all_attacking()"
    self
  end

  def heal_attacking(damage)
    @text << ".heal_attacking(#{damage})"
    self
  end

  def each_own_bench(&what)
    @text << ".each_own_bench(|e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def each_opponents_bench(&what)
    @text << ".each_opponents_bench(|e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def knock_out_attacker_if_attacking_is_knocked_out_next_turn
    @text << ".knock_out_attacker_if_attacking_is_knocked_out_next_turn()"
    self
  end

  def prevent_damage_during_opponents_next_turn
    @text << ".prevent_damage_during_opponents_next_turn()"
    self
  end

  def damage_per_damage_counter_on_itself(damage_per_counter)
    @text << ".damage_per_damage_counter_on_itself(#{damage_per_counter})"
    self
  end

  def damage_minus_per_damage_counter_on_itself(base_damage, minus)
    @text << ".damage_minus_per_damage_counter_on_itself(#{base_damage}, #{minus})"
    self
  end

  def damage_plus_per_damage_counter_on_defending(base_damage, plus)
    @text << ".damage_plus_per_damage_counter_on_defending(#{base_damage}, #{plus})"
    self
  end

  def damage_plus_per_energy_card_on_defending(base_damage, plus)
    @text << ".damage_plus_per_energy_card_on_defending(#{base_damage}, #{plus})"
    self
  end

  def damage_plus_per_extra_energy_on_attacking(base_damage, plus, energy_type, energy_limit)
    energy_type = "Type::#{energy_type}"

    @text << ".damage_plus_per_extra_energy_on_attacking(#{base_damage}, #{plus}, #{energy_type}, #{energy_limit})"
    self
  end

  def damage_half_defending_remaining_hp
    @text << ".damage_half_defending_remaining_hp()"
    self
  end

  def attack_cost(cost)
    @text << ".attack_cost(&[#{cost.map{|c| "Type::#{c}"}.join(", ")}])"
    self
  end

  def must(&what)
    @text << ".must(|e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
    self
  end

  def defending_must_be_asleep
    @text << ".defending_must_be_asleep()"
    self
  end

  def done
    @text[-1] += ";"
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
  text = attack.fetch('text').gsub(/\([^)]+\)/, '').gsub(/ +/, " ").gsub(/\ +\./, ".").strip
  damage = attack.fetch('damage', "")

  builder = Builder.new
    .attack_cost(attack["cost"])

  if text.empty?
    builder.damage(damage)
  elsif text =~ /^The Defending Pokémon is now (Asleep|Paralyzed|Confused|Poisoned)\.$/
    status = status_to_verb($1.downcase)
    builder
      .damage(damage)
      .inflict(status)
  elsif text =~ /^The Defending Pokémon is now Poisoned\. It now takes (\w+) Poison damage instead of 10 after each player's turn\.$/
    counters = $1.to_i / 10
    builder
      .damage(damage)
      .severe_poison(counters)
  elsif text =~ /^Flip a coin\. If heads, the Defending Pokémon is now (Asleep|Paralyzed|Confused|Poisoned)\.$/
    inflict = status_to_verb($1.downcase)
    builder
      .flip_a_coin
      .damage(damage)
      .if_heads{ inflict(inflict) }
  elsif text =~ /^Flip a coin\. If heads, the Defending Pokémon is now (Asleep|Paralyzed|Confused|Poisoned)\; if tails, it is now (Asleep|Paralyzed|Confused|Confused)\.$/
    inflict_yes = status_to_verb($1.downcase)
    inflict_no = status_to_verb($2.downcase)
    builder
      .flip_a_coin
      .damage(damage)
      .if_heads { inflict(inflict_yes) }
      .if_tails { inflict(inflict_no) }
  elsif text =~ /^Flip a coin\. If tails, \w+ does (\d+) damage to itself\.$/
    self_damage = $1.to_i
    builder
      .flip_a_coin.damage(damage)
      .if_tails { damage_itself(self_damage) }
  elsif text =~ /^Flip (\d+) coins\. This attack does (\d+) damage times the number of heads\./
    coins = $1.to_i
    damage = $2.to_i
    builder
      .flip_coins(coins)
      .damage_per_heads(damage)
  elsif text =~ /^Does (\d+) damage times the number of damage counters on \w+\.$/
    damage_per_counter = $1.to_i
    builder
      .damage_per_damage_counter_on_itself(damage_per_counter)
  elsif text =~ /^Does (\d+) damage minus (\d+) damage for each damage counter on \w+\.$/
    base_damage = $1.to_i
    minus_damage = $2.to_i
    builder
      .damage_minus_per_damage_counter_on_itself(base_damage, minus_damage)
  elsif text =~ /^Does (\d+) damage plus (\d+) more damage for each damage counter on the Defending Pokémon\.$/
    base_damage = $1.to_i
    plus_damage = $2.to_i
    builder
      .damage_plus_per_damage_counter_on_defending(base_damage, plus_damage)
  elsif text =~ /^Does damage to the Defending Pokémon equal to half the Defending Pokémon's remaining HP\.$/
    builder
      .damage_half_defending_remaining_hp
  elsif text =~ /^Does (\d+) damage plus (\d+) more damage for each Energy card attached to the Defending Pokémon\.$/
    base_damage = $1.to_i
    plus_damage = $2.to_i
    builder
      .damage_plus_per_energy_card_on_defending(base_damage, plus_damage)
  elsif text =~ /^\w+ does (\d+) damage to itself.$/
    damage_self = $1.to_i
    builder
      .damage(damage)
      .damage_itself(damage_self)
  elsif text =~ /^Flip a coin\. If heads, prevent all damage done to (.+) during your opponent's next turn\.$/
    builder
      .flip_a_coin
      .if_heads { prevent_damage_during_opponents_next_turn }
  elsif text =~ /^Discard 1 (Fire|Water) Energy card attached to \w+ in order to use this attack\.$/
    energy_type = $1
    builder
      .must { discard_attacking_energy_cards([energy_type]) }
      .damage(damage)
  elsif text =~ /^Discard (\d+) Energy cards attached to \w+ in order to use this attack\.$/
    how_many = $1.to_i
    builder
      .must { discard_attacking_energy_cards(["Any"] * how_many) }
      .damage(damage)
  elsif text =~ /^Discard all Energy cards attached to \w+ in order to use this attack\.$/
    builder
      .must { discard_all_attacking_energy_cards }
      .damage(damage)
  elsif text =~ /^Discard (\d+) (\w+) Energy cards? attached to \w+ in order to use this attack\. Remove all damage counters from \w+\.$/
    how_many = $1.to_i
    energy_type = $2
    builder
      .must { discard_attacking_energy_cards([energy_type] * how_many) }
      .heal_all_attacking
  elsif text =~ /^If the Defending Pokémon has any Energy cards attached to it, choose (\d+) of them and discard it\.$/
    how_many = $1.to_i
    builder.discard_defending_attached_energy_cards(["Any"] * how_many).damage(damage)
  elsif text =~ /^Flip a coin\. If heads, this attack does (\d+) damage plus (\d+) more damage; if tails, this attack does (\d+) damage (?:plus|and) \w+ does (\d+) damage to itself\.$/
    damage_base1 = $1.to_i
    damage_extra = $2.to_i
    damage_base2 = $3.to_i
    damage_self = $4.to_i

    builder
      .flip_a_coin
      .if_heads{ damage(damage_base1 + damage_extra) }
      .if_tails{ damage(damage_base2).damage_itself(damage_self) }
  elsif text =~ /^Does (\d+) damage to each of your own Benched Pokémon\.$/
    bench_damage = $1
    builder
      .damage(damage)
      .each_own_bench { damage(bench_damage) }
  elsif text =~ /^Does (\d+) damage to each Pokémon on each player's Bench\. \w+ does (\d+) damage to itself\.$/
    bench_damage = $1.to_i
    self_damage = $2.to_i
    builder
      .damage(damage)
      .each_own_bench { damage(bench_damage) }
      .each_opponents_bench { damage(bench_damage) }
      .damage_itself(self_damage)
  elsif text =~ /^You can't use this attack unless the Defending Pokémon is Asleep\.$/
    builder
      .defending_must_be_asleep
      .damage(damage)
  elsif text =~ /^Discard (\d+) (\w+) Energy card attached to \w+ in order to use this attack. If a Pokémon Knocks Out \w+ during your opponent's next turn, Knock Out that Pokémon\./
    how_many = $1.to_i
    energy_type = $2
    builder
      .damage(damage)
      .knock_out_attacker_if_attacking_is_knocked_out_next_turn
  elsif text =~ /^Does (\d+) damage plus (\d+) more damage for each (\w+) Energy attached to \w+ but not used to pay for this attack's Energy cost. Extra (\w+) Energy after the (\d+)(?:st|nd|rd|th) (?:doesn't|don't) count\.$/

    base_damage = $1.to_i
    plus_damage = $2.to_i
    energy_type = $3
    energy_type2 = $4
    energy_limit = $5.to_i

    raise unless energy_type == energy_type2

    builder
      .damage_plus_per_extra_energy_on_attacking(base_damage, plus_damage, energy_type, energy_limit)
  elsif text =~ /^Unless all damage from this attack is prevented, you may remove (\d+) damage counter from \w+\.$/
    counters = $1.to_i
    builder
      .damage(damage)
      .if_did_damage { heal_attacking(counters * 10) }
  elsif text =~ /^$/
  else
    return "unimplemented!();"
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
  [attack["text"].gsub(/\d+/, "X").gsub(/\([^)]+\)/, '').gsub(/\b(Water|Fire|Lightning|Psychic)\b/, 'TYPE').split(". "), attack["damage"]]
end

PP.pp poops.sort, out=$stderr
