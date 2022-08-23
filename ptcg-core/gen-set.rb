#!/usr/bin/env ruby

# Converts a pokemontcg.io json file representing a full TCG set into a rust file.

require 'pp'
require 'json'
require 'erb'

PRELUDE = <<~EOF
use crate::*;
use crate::state::Type;
use crate::attack_builder::AttackBuilder;

EOF

TEMPLATE = <<~EOF
#[derive(Default)]
pub struct <%= card.struct_name %><%= card.set_number %> {}
impl CardArchetype for <%= card.struct_name %><%= card.set_number %> {
    identifier!("<%= card.display_name %> (<%= card.set_abbreviation %> <%= card.set_number %>)");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
<% card.attacks.each do |attack| -%>
        Attack::new("<%= attack.name %>", Self::<%= attack.fn_name %>),
<% end -%>
      ]
    }
}
impl <%= card.struct_name %><%= card.set_number %> {
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
  }[status.downcase]
end

class Builder
  def initialize(engine_name="builder")
    @text = [engine_name]
    @is_engining = false
    @engine_name = engine_name
  end

  def to_s(compact: false)
    if compact
      @text.join
    else
      @text.map { |x| if x.start_with?(".") then "    #{x}\n" else "#{x}\n" end }.join
    end
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

  def prevent_damage_and_effects_during_opponents_next_turn
    @text << ".prevent_damage_and_effects_during_opponents_next_turn()"
    self
  end

  def prevent_up_to_damage_during_opponents_next_turn(damage)
    @text << ".prevent_up_to_damage_during_opponents_next_turn(#{damage})"
    self
  end

  def prevent_attack_on_a_flip_during_opponents_next_turn
    @text << ".prevent_attack_on_a_flip_during_opponents_next_turn()"
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

  def cost(&what)
    @text << ".cost(|e| #{Builder.new("e").instance_eval(&what).to_s(compact: true)})"
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

  def switch_defending
    @text << ".switch_defending()"
    self
  end

  def change_attacking_resistance_except(exceptions)
    @text << ".change_attacking_resistance_except(&[#{exceptions.map{|c| "Type::#{c}"}.join(", ")}])"
    self
  end

  def change_defending_weakness_except(exceptions)
    @text << ".change_defending_weakness_except(&[#{exceptions.map{|c| "Type::#{c}"}.join(", ")}])"
    self
  end

  def disable_defending_attack
    @text << ".disable_defending_attack()"
    self
  end

  def copy_defending_attack_without_costs
    @text << ".copy_defending_attack_without_costs()"
    self
  end

  def gust_defending
    @text << ".gust_defending()"
    self
  end

  def once_while_in_play
    @text << ".once_while_in_play()"
    self
  end

  def draw(n)
    @text << ".draw(#{n})"
    self
  end
end

$patterns = [
  {
    pattern: /^$/,
    build: ->(damage:) { damage(damage) },
  },
  {
    pattern: /^The Defending Pokémon is now (?<status>Asleep|Paralyzed|Confused|Poisoned)\.$/,
    build: ->(damage:, status:) { damage(damage).inflict(status_to_verb(status.downcase)) },
  },
  {
    pattern: /^The Defending Pokémon is now Poisoned\. It now takes (?<counters>\w+) Poison damage instead of 10 after each player's turn\.$/,
    build: ->(damage:, counters:) { damage(damage).severe_poison(counters.to_i / 10) },
  },
  {
    pattern: /^Flip a coin\. If heads, the Defending Pokémon is now (?<status>Asleep|Paralyzed|Confused|Poisoned)\.$/,
    build: ->(damage:, status:) { flip_a_coin.damage(damage).if_heads{ inflict(status_to_verb(status)) } }
  },
  {
    pattern: /^Flip a coin\. If heads, the Defending Pokémon is now (?<status_heads>Asleep|Paralyzed|Confused|Poisoned)\. If tails, it is now (?<status_tails>Asleep|Paralyzed|Confused|Confused)\.$/,
    build: ->(damage:, status_heads:, status_tails:) {
      flip_a_coin.damage(damage).if_heads { inflict(status_to_verb(status_heads)) }.if_tails { inflict(status_to_verb(status_tails)) }
    }
  },
  {
    pattern: /^Flip a coin\. If tails, \w+ does (?<self_damage>\d+) damage to itself\.$/,
    build: ->(damage:, self_damage:) { flip_a_coin.damage(damage).if_tails { damage_itself(self_damage.to_i) } },
  },
  {
    pattern: /^Flip (?<coins>\d+) coins\. This attack does (?<damage_per_heads>\d+) damage times the number of heads\./,
    build: ->(damage:, coins:, damage_per_heads:) { flip_coins(coins.to_i).damage_per_heads(damage_per_heads.to_i) },
  },
  {
    pattern: /^Does (?<damage_per_counter>\d+) damage times the number of damage counters on \w+\.$/,
    build: ->(damage:, damage_per_counter:) { damage_per_damage_counter_on_itself(damage_per_counter.to_i) },
  },
  {
    pattern: /^Does (?<base_damage>\d+) damage minus (?<minus_damage>\d+) damage for each damage counter on \w+\.$/,
    build: ->(damage:, base_damage:, minus_damage:) { damage_minus_per_damage_counter_on_itself(base_damage, minus_damage) },
  },
  {
    pattern: /^Does (?<base_damage>\d+) damage plus (?<plus_damage>\d+) more damage for each damage counter on the Defending Pokémon\.$/,
    build: ->(base_damage:, plus_damage:) { damage_plus_per_damage_counter_on_defending(base_damage.to_i, plus_damage.to_i) },
  },
  {
    pattern: /^Does damage to the Defending Pokémon equal to half the Defending Pokémon's remaining HP\.$/,
    build: ->() { damage_half_defending_remaining_hp },
  },
  {
    pattern: /^Does (?<base_damage>\d+) damage plus (?<plus_damage>\d+) more damage for each Energy card attached to the Defending Pokémon\.$/,
    build: ->(base_damage:, plus_damage:) { damage_plus_per_energy_card_on_defending(base_damage.to_i, plus_damage.to_i) },
  },
  {
    pattern: /^\w+ does (?<damage_self>\d+) damage to itself.$/,
    build: ->(damage:, damage_self:) { damage(damage).damage_itself(damage_self.to_i) },
  },
  {
    pattern: /^Flip a coin\. If heads, prevent all damage done to \w+ during your opponent's next turn\.$/,
    build: ->(damage:) { flip_a_coin.damage(damage).if_heads { prevent_damage_during_opponents_next_turn } },
  },
  {
    pattern: /^Discard (?<how_many>\d+) (?<energy_type>\w+) Energy card attached to \w+ in order to prevent all effects of attacks, including damage, done to \w+ during your opponent's next turn\.$/,
    build: ->(damage:, how_many:, energy_type:) {
      cost { discard_attacking_energy_cards([energy_type] * how_many.to_i) }
        .damage(damage)
        .prevent_damage_and_effects_during_opponents_next_turn
    },
  },
  {
    pattern: /^Flip a coin. If heads, during your opponent's next turn, prevent all effects of attacks, including damage, done to \w+\.$/,
    build: ->() { flip_a_coin.if_heads { prevent_damage_and_effects_during_opponents_next_turn } },
  },
  {
    pattern: /^If the Defending Pokémon tries to attack during your opponent's next turn, your opponent flips a coin\. If tails, that attack does nothing\.$/,
    build: ->() { prevent_attack_on_a_flip_during_opponents_next_turn },
  },
  {
    pattern: /^Discard (?<how_many>\d+) (?<energy_type>\w+) Energy card attached to \w+ in order to use this attack\.$/,
    build: ->(damage:, how_many:, energy_type:) { cost { discard_attacking_energy_cards([energy_type] * how_many.to_i) }.damage(damage) },
  },
  {
    pattern: /^Discard (?<how_many>\d+) Energy cards attached to \w+ in order to use this attack\.$/,
    build: ->(damage:, how_many:) { cost { discard_attacking_energy_cards(["Any"] * how_many.to_i) }.damage(damage) },
  },
  {
    pattern: /^Discard all Energy cards attached to \w+ in order to use this attack\.$/,
    build: ->(damage:) { cost { discard_all_attacking_energy_cards }.damage(damage) },
  },
  {
    pattern: /^Discard (?<how_many>\d+) (?<energy_type>\w+) Energy cards? attached to \w+ in order to use this attack\. Remove all damage counters from \w+\.$/,
    build: ->(damage:, how_many:, energy_type:) { cost { discard_attacking_energy_cards([energy_type] * how_many.to_i) }.heal_all_attacking },
  },
  {
    pattern: /^If the Defending Pokémon has any Energy cards attached to it, choose (?<how_many>\d+) of them and discard it\.$/,
    build: ->(damage:, how_many:) { discard_defending_attached_energy_cards(["Any"] * how_many.to_i).damage(damage) },
  },
  {
    pattern: /^Flip a coin\. If heads, this attack does (?<base1>\d+) damage plus (?<damage_extra>\d+) more damage. If tails, this attack does (?<base2>\d+) damage (?:plus|and) \w+ does (?<damage_self>\d+) damage to itself\.$/,
    build: ->(base1:, damage_extra:, base2:, damage_self:) {
      flip_a_coin
        .if_heads{ damage(base1 + damage_extra) }
        .if_tails{ damage(base2).damage_itself(damage_self) }
    },
  },
  {
    pattern: /^Flip a coin\. If heads, this attack does (?<base_heads>\d+) damage plus (?<extra>\d+) more damage\. If tails, this attack does (?<base_tails>\d+) damage\.$/,
    build: ->(base_heads:, extra:, base_tails:) {
      flip_a_coin
        .if_heads{ damage(base_heads + extra) }
        .if_tails{ damage(base_tails) }
    }
  },
  {
    pattern: /^Does (?<bench_damage>\d+) damage to each of your own Benched Pokémon\.$/,
    build: ->(damage:, bench_damage:) { damage(damage).each_own_bench { damage(bench_damage) } },
  },
  {
    pattern: /^Does (?<bench_damage>\d+) damage to each Pokémon on each player's Bench\. \w+ does (?<self_damage>\d+) damage to itself\.$/,
    build: ->(damage:, bench_damage:, self_damage:) {
      damage(damage)
        .each_own_bench { damage(bench_damage) }
        .each_opponents_bench { damage(bench_damage) }
        .damage_itself(self_damage)
    },
  },
  {
    pattern: /^You can't use this attack unless the Defending Pokémon is Asleep\.$/,
    build: ->(damage:) { defending_must_be_asleep.damage(damage) },
  },
  {
    pattern: /^Discard (?<how_many>\d+) (?<energy_type>\w+) Energy card attached to \w+ in order to use this attack. If a Pokémon Knocks Out \w+ during your opponent's next turn, Knock Out that Pokémon\./,
    build: ->(damage:, how_many:, energy_type:) {
      cost { discard_attacking_energy_cards([energy_type] * how_many.to_i) }
        .damage(damage)
        .knock_out_attacker_if_attacking_is_knocked_out_next_turn
    },
  },
  {
    pattern: /^Does (?<base_damage>\d+) damage plus (?<plus_damage>\d+) more damage for each (?<energy_type>\w+) Energy attached to \w+ but not used to pay for this attack's Energy cost. Extra \w+ Energy after the (?<energy_limit>\d+)(?:st|nd|rd|th) (?:doesn't|don't) count\.$/,
    build: ->(base_damage:, plus_damage:, energy_type:, energy_limit:) { damage_plus_per_extra_energy_on_attacking(base_damage, plus_damage, energy_type, energy_limit) },
  },
  {
    pattern: /^Flip a coin\. If tails, this attack does nothing\.$/,
    build: ->(damage:) { flip_a_coin.if_heads { damage(damage) } },
  },
  {
    pattern: /^Flip a coin\. If tails, this attack does nothing\. Either way, you can't use this attack again as long as [\w']+ stays in play\.$/,
    build: ->(damage:) {
      flip_a_coin
        .if_heads { damage(damage) }
        .once_while_in_play
    },
  },
  {
    pattern: /^Unless all damage from this attack is prevented, you may remove (?<counters>\d+) damage counter from \w+\.$/,
    build: ->(damage:, counters:) { damage(damage).if_did_damage { heal_attacking(counters.to_i * 10) } },
  },
  {
    pattern: /^If your opponent has any Benched Pokémon, he or she chooses 1 of them and switches it with the Defending Pokémon\.$/,
    build: ->(damage:) { damage(damage).switch_defending() },
  },
  {
    pattern: /^If your opponent has any Benched Pokémon, choose 1 of them and switch it with his or her Active Pokémon\.$/,
    build: ->(damage:) { damage(damage).gust_defending() },
  },
  {
    pattern: /^Flip a coin\. If heads, flip another coin\. If the second coin is heads, this attack does (?<damage_heads>\d+) damage to each of your opponent's Benched Pokémon\. If the second coin is tails, this attack does (?<damage_tails>\d+) damage to each of your opponent's Benched Pokémon\.$/,
    build: ->(damage:, damage_heads:, damage_tails:) {
      damage(damage)
        .flip_a_coin
        .if_heads {
          flip_a_coin
            .if_heads{ each_opponents_bench{ damage(damage_heads) } }
            .if_tails{ each_own_bench{ damage(damage_tails) } }
        }
    }
  },
  {
    pattern: /^Change \w+'s Resistance to a type of your choice other than (?<except>\w+)\.$/,
    build: ->(damage:, except:) {
      damage(damage)
        .change_attacking_resistance_except([except])
    },
  },
  {
    pattern: /^If the Defending Pokémon has a Weakness, you may change it to a type of your choice other than (?<except>\w+)\.$/,
    build: ->(damage:, except:) {
      damage(damage)
        .change_defending_weakness_except([except])
    },
  },
  {
    pattern: /^Choose 1 of the Defending Pokémon's attacks\. That Pokémon can't use that attack during your opponent's next turn\.$/,
    build: ->(damage:) {
      damage(damage)
        .disable_defending_attack()
    },
  },
  {
    pattern: /^During your opponent's next turn, whenever (?<up_to>\d+) or less damage is done to \w+, prevent that damage\.$/,
    build: ->(damage:, up_to:) {
      damage(damage)
        .prevent_up_to_damage_during_opponents_next_turn(up_to)
    }
  },
  {
    pattern: /^Choose 1 of the Defending Pokémon's attacks\. Metronome copies that attack except for its Energy costs and anything else required in order to use that attack, such as discarding Energy cards\.$/,
    build: ->(damage:) {
      damage(damage)
        .copy_defending_attack_without_costs()
    }
  },

  {
    pattern: /^Draw a card\.$/,
    build: ->(damage:) {
      damage(damage)
        .draw(1)
    }
  },
  {
    pattern: /^Flip a coin\. If heads, draw a card\.$/,
    build: ->(damage:) {
      flip_a_coin
        .damage(damage)
        .if_heads { draw(1) }
    }
  },
]

def attack_impl(attack)
  text = attack
    .fetch('text')
    .gsub(/\([^)]+\)/, '')
    .gsub(/ +/, " ")
    .gsub(/\ +\./, ".")
    .gsub(/\ +\,/, ",")
    .strip
  damage = attack.fetch('damage', "")

  builder = Builder.new.attack_cost(attack["cost"])

  $patterns.each do |pattern|
    m = pattern[:pattern].match(text)
    if m
      v = m.named_captures
        .transform_keys(&:to_sym)
        .transform_values { |v| v =~ /^\d+$/ ? v.to_i : v.to_s }
      v.merge!({ damage: attack.fetch('damage') }) if pattern[:build].parameters.include?([:keyreq, :damage])
      return builder.instance_exec(**v, &pattern[:build]).to_s.rstrip.gsub("\n", "\n        ")
    end
  end

  "unimplemented!();"
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
    .gsub('♀', 'F')
    .gsub("'d", "D")
    .gsub('.', "")
end


def process_set(contents)
  output = ""
  output << PRELUDE

  # typos in dataset
  contents = contents
    .gsub(/Paralzyed/, "Paralyzed")
    .gsub(/during your next turn, any damage done by the attack is reduced by/, "during your opponent's next turn, any damage done by the attack is reduced by")

  # normalization
  contents = contents
    .gsub(/not used to pay for this attack('s Energy cost)?/, "not used to pay for this attack's Energy cost")
    .gsub(/do (that|the) damage/, "do that damage")
    .gsub(/; if tails/, ". If tails")
    .gsub(/ If \w+ has fewer damage counters than that, remove all of them\./, '')

  cards = JSON.parse(contents)['data']
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
      set_number: card["number"],
      set_abbreviation: card["set"]["ptcgoCode"],
      stage: stage,
      weakness: weakness,
      resistance: resistance,
      retreat: card.fetch("retreatCost", []).size,
      types: card["types"],
      hp: card["hp"],
      attacks: card["attacks"].then { |as| Array(as) }.map { |a|
        OpenStruct.new(a.merge(
          fn_name: idify(a["name"]),
          impl: attack_impl(a),
        ))
      },
    )

    output << ERB.new(TEMPLATE, trim_mode: "<>-").result_with_hash(card: card)
  end

  output = output.strip.gsub(/^ +$/, '')

  set_name = cards[0]['set']['name'].downcase.gsub(/ /, "_")
  File.write("src/sets/#{set_name}/pokemon.rs", output)

  poops = pokemon_cards
    .flat_map { |card| card.fetch("attacks", []).map { |a| a.merge("card" => card) } }
    .filter { |attack| attack_impl(attack) == "unimplemented!();" }
    .map do |attack|
      attack["text"]
        .gsub(/\d+/, "X")
        .gsub(/\s*\([^)]+\)\s*/, '')
        .gsub(/\b(Water|Fire|Lightning|Psychic|Fighting)\b/, 'TYPE')
        .gsub(/\b(Poisoned|Asleep|Confused|Paralyzed)\b/, 'SPECIAL_CONDITION')
        .gsub(Regexp.new("\\b" + Regexp.escape(attack['card']['name']) + '\b'), 'THIS_POKEMON')
        .split(".")
        .map(&:strip)
    end

  {
    total_attacks: pokemon_cards.sum { |card| card.fetch("attacks", []).size },
    missing_attacks: pokemon_cards.sum { |card| card.fetch("attacks", []).filter{|attack| attack_impl(attack) == "unimplemented!();" }.size },
    missing_patterns: poops,
  }
end

if !ARGV.empty?
  stats = ARGV
    .map { |filename| process_set(File.read(filename)) }

  poops = stats.flat_map { |x| x[:missing_patterns] }.tally.map(&:reverse).sort
  total_attacks = stats.sum { |x| x[:total_attacks] }
  missing_attacks = stats.sum { |x| x[:missing_attacks] }

  pp poops
  puts "total attacks: #{total_attacks}"
  puts "missing attacks: #{missing_attacks}"
  puts "implemented attacks: #{total_attacks - missing_attacks}"
  puts "missing patterns: #{poops.size}"
  puts "implemented patterns: #{$patterns.size}"
  puts "implemented verbs: #{(Builder.instance_methods(false) - %i[initialize to_s]).size}"
else
  process_set(STDIN.read)
end
