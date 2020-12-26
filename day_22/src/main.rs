extern crate im_rc;

use im_rc::hashset;
use im_rc::HashMap;
use im_rc::HashSet;
use im_rc::Vector;
use itertools::Itertools;
use std::fmt;

fn main() {
    println!("--- [AoC 2020] Day 22: Crab Combat ---");

    let input = utils::read_strings_from_param();

    println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two(&input));
}

fn part_one(input: &Vector<String>) -> usize {
    let decks = Deck::parse_all(input);
    println!("{}", decks.iter().join("\n\n"));
    Deck::play(1, decks)
}

fn part_two(input: &Vector<String>) -> usize {
    let original_decks = Deck::parse_all(input);
    println!("{}\n", original_decks.iter().join("\n\n"));
    let winner = Deck::play_rec(1, 1, original_decks, HashMap::new());

    println!("== Post-game results ==");
    println!(
        "winners {} deck: {}",
        winner.player,
        winner.cards.iter().join(", ")
    );
    winner.score()
}

#[derive(Clone, Debug)]
struct Deck {
    player: String,
    cards: Vector<usize>,
}

impl Deck {
    fn parse_all(input: &Vector<String>) -> Vector<Deck> {
        itertools::unfold(input.iter(), |iterator| {
            let lines: Vector<_> = iterator
                .by_ref()
                .take_while(|line| !line.is_empty())
                .collect();
            if lines.is_empty() {
                None
            } else {
                Some(Deck::parse(&lines))
            }
        })
        .collect()
    }

    fn parse(input: &Vector<&String>) -> Deck {
        let name = input.head().unwrap().replace(":", "");
        let cards = input
            .iter()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|l| l.parse::<usize>().unwrap())
            .collect();
        Deck {
            player: name,
            cards,
        }
    }

    fn play(round: usize, mut players: Vector<Deck>) -> usize {
        println!("-- Round {:4} --", round);
        for deck in &players {
            println!("{}'s deck: {}", deck.player, deck.cards.iter().join(", "));
        }

        let cards: Vector<_> = players
            .iter_mut()
            .filter_map(|deck| {
                deck.next_card()
                    .map(move |card| (deck.player.to_owned(), card))
            })
            .collect();

        for (name, card) in &cards {
            println!("{} plays: {}", name, card)
        }

        let winner = cards
            .iter()
            .max_by_key(|(_name, card)| card)
            .map(|(name, _)| name)
            .unwrap();

        println!("{} wins the round", winner);

        let winning_deck = players
            .iter_mut()
            .find(|deck| &deck.player == winner)
            .unwrap();
        let winners_cards: Vector<_> = cards
            .iter()
            .map(|(_name, card)| card)
            .sorted()
            .rev()
            .cloned()
            .collect();

        winning_deck.add_cards(winners_cards);

        players.retain(|deck| !deck.is_empty());
        if players.len() > 1 {
            Deck::play(round + 1, players)
        } else {
            println!("== Post-game results ==");
            for deck in &players {
                println!("{}'s deck: {}", deck.player, deck.cards.iter().join(", "));
            }

            players.head().unwrap().score()
        }
    }

    fn play_rec(
        round: usize,
        game: usize,
        mut players: Vector<Deck>,
        previous_states: HashMap<String, HashSet<Vector<usize>>>,
    ) -> Deck {
        let identical_round = players.iter().find(|player| {
            previous_states
                .get(&player.player[..])
                .map(|states| states.contains(&player.cards))
                .unwrap_or(false)
        });

        if round == 1 {
            println!("=== Game {} ===\n", game);
        }
        println!("-- Round {} (Game {}) --", round, game);

        for deck in &players {
            println!("{}'s deck: {}", deck.player, deck.cards.iter().join(", "));
        }

        if let Some(prev) = identical_round {
            println!("Identical cards for {}", prev);
            let winner = players.head().unwrap();
            println!("The winner of game {} is {}!", game, winner.player);
            return winner.clone();
        }

        let updated_states = players.iter().fold(previous_states, |states, player| {
            states.update_with(
                player.player.to_owned(),
                hashset!(player.cards.clone()),
                |old, new| old.union(new),
            )
        });

        let cards: Vector<_> = players
            .iter_mut()
            .filter_map(|deck| {
                deck.next_card()
                    .map(move |card| (deck.player.to_owned(), card))
            })
            .collect();

        for (name, card) in &cards {
            println!("{} plays: {}", name, card)
        }

        let trigger_subgame = cards.iter().all(|(name, card)| {
            players
                .iter()
                .find_map(|p| {
                    if &p.player == name {
                        Some(p.cards.len())
                    } else {
                        None
                    }
                })
                .unwrap()
                >= *card
        });

        let winner = if trigger_subgame {
            println!("Playing a sub-game to determine the winner...\n");
            let sub_game_decks = players
                .iter()
                .map(|deck| {
                    let current_card = cards
                        .iter()
                        .find_map(|(name, card)| {
                            if name == &deck.player {
                                Some(card)
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    Deck {
                        player: deck.player.to_owned(),
                        cards: deck.cards.iter().take(*current_card).cloned().collect(),
                    }
                })
                .collect();
            let sub_game_winner = Deck::play_rec(1, game + 1, sub_game_decks, HashMap::new());
            sub_game_winner.player
        } else {
            cards
                .iter()
                .max_by_key(|(_name, card)| card)
                .map(|(name, _)| name)
                .unwrap()
                .to_owned()
        };

        println!("{} wins round {} of game {}!", winner, round, game);

        let winning_deck = players
            .iter_mut()
            .find(|deck| deck.player == winner)
            .unwrap();
        let winners_cards: Vector<_> = cards
            .iter()
            .sorted_by_key(|(name, _)| if name == &winner { 0 } else { 1 })
            .map(|(_name, card)| card)
            .cloned()
            .collect();

        winning_deck.add_cards(winners_cards);

        players.retain(|deck| !deck.is_empty());
        if players.len() > 1 {
            Deck::play_rec(round + 1, game, players, updated_states)
        } else {
            let overall_winner = players.head().unwrap();
            println!(
                "The winner of game {} is player {}!",
                game, overall_winner.player
            );

            if game > 1 {
                println!("\n...anyway, back to game {}", game - 1);
            }

            overall_winner.clone()
        }
    }

    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(pos, card)| (pos + 1) * card)
            .sum()
    }

    fn add_cards(&mut self, cards: Vector<usize>) {
        self.cards.append(cards);
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn next_card(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n{}", self.player, self.cards.iter().join("\n"))
    }
}
