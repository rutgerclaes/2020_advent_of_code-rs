extern crate im_rc;

use im_rc::vector;
use im_rc::HashMap;
use im_rc::HashSet;
use im_rc::Vector;
use itertools::Itertools;

fn main() {
    println!("--- [AoC 2020] Day 21: Allergen Assessment ---");

    let input = utils::read_strings_from_param();

    println!("Solution to part one: {}", part_one(&input));
    println!("Solution to part two: {}", part_two(&input));
}

fn part_one(input: &Vector<String>) -> usize {
    let foods: Vector<_> = input.iter().map(|line| parse_line(&line[..])).collect();

    let ingredients_by_allergens: HashMap<&Allergen, Vector<HashSet<&Ingredient>>> =
        foods.iter().fold(
            HashMap::new(),
            |ingr_per_allergen, (ingredients, allergens)| {
                allergens
                    .iter()
                    .fold(ingr_per_allergen, |ingr_per_allergen, allergen| {
                        ingr_per_allergen.update_with(
                            allergen,
                            vector!(ingredients.iter().collect()),
                            |mut a, b| {
                                a.append(b);
                                a
                            },
                        )
                    })
            },
        );

    let candidates_by_allergens: HashMap<&Allergen, HashSet<&Ingredient>> =
        ingredients_by_allergens
            .iter()
            .map(|(&allergens, ingredients_per_food)| {
                let all_ingredients: HashSet<&Ingredient> =
                    HashSet::unions(ingredients_per_food.clone());
                let candidates: HashSet<_> = all_ingredients
                    .iter()
                    .filter(|&ingredient| {
                        ingredients_per_food
                            .iter()
                            .all(|list| list.contains(ingredient))
                    })
                    .cloned()
                    .collect();
                (allergens, candidates)
            })
            .collect();

    let all_ingredients: HashSet<&Ingredient> =
        foods.iter().map(|(ingr, _)| ingr).flatten().collect();

    let allergen_free: HashSet<&Ingredient> = all_ingredients
        .iter()
        .by_ref()
        .filter(|&ingredient| {
            candidates_by_allergens
                .values()
                .all(|cand| !cand.contains(ingredient))
        })
        .cloned()
        .collect();

    allergen_free
        .iter()
        .map(|ingredient| {
            foods
                .iter()
                .filter(|(ingredients, _)| ingredients.contains(ingredient))
                .count()
        })
        .sum()
}

fn part_two(input: &Vector<String>) -> String {
    let foods: Vector<_> = input.iter().map(|line| parse_line(&line[..])).collect();

    let ingredients_by_allergens: HashMap<&Allergen, Vector<HashSet<&Ingredient>>> =
        foods.iter().fold(
            HashMap::new(),
            |ingr_per_allergen, (ingredients, allergens)| {
                allergens
                    .iter()
                    .fold(ingr_per_allergen, |ingr_per_allergen, allergen| {
                        ingr_per_allergen.update_with(
                            allergen,
                            vector!(ingredients.iter().collect()),
                            |mut a, b| {
                                a.append(b);
                                a
                            },
                        )
                    })
            },
        );

    let mut candidates_by_allergens: HashMap<&Allergen, HashSet<&Ingredient>> =
        ingredients_by_allergens
            .iter()
            .map(|(&allergens, ingredients_per_food)| {
                let all_ingredients: HashSet<&Ingredient> =
                    HashSet::unions(ingredients_per_food.clone());
                let candidates: HashSet<_> = all_ingredients
                    .iter()
                    .filter(|&ingredient| {
                        ingredients_per_food
                            .iter()
                            .all(|list| list.contains(ingredient))
                    })
                    .cloned()
                    .collect();
                (allergens, candidates)
            })
            .collect();

    let mut resolved: HashMap<&Ingredient, &Allergen> = HashMap::new();

    while resolved.len() < candidates_by_allergens.len() {
        if let Some((&allergen, &ingredient)) =
            candidates_by_allergens
                .iter()
                .find_map(|(allergen, ingredients)| {
                    if ingredients.len() == 1 {
                        Some((allergen, ingredients.iter().next().unwrap()))
                    } else {
                        None
                    }
                })
        {
            candidates_by_allergens
                .iter_mut()
                .for_each(|(_, ingredients)| {
                    ingredients.remove(ingredient);
                });
            resolved.insert(ingredient, allergen);
        } else {
            println!("{:?}", candidates_by_allergens);
            panic!("Could not find unique combo");
        }
    }

    let result: Vector<_> = resolved
        .iter()
        .sorted_by_key(|(_, allergen)| &allergen.0)
        .map(|(ingredient, _)| &ingredient.0[..])
        .collect();

    let outcome: String = result.iter().join(",");
    outcome
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Allergen(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Ingredient(String);

fn parse_line(input: &str) -> (HashSet<Ingredient>, HashSet<Allergen>) {
    let (ingredients, allergens) = input.split('(').collect_tuple().unwrap();
    let parsed_ingredients: HashSet<_> = ingredients
        .split(' ')
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(Ingredient(l.to_owned()))
            }
        })
        .collect();
    let parsed_allergens: HashSet<_> = allergens
        .replace("contains ", "")
        .replace(")", "")
        .split(", ")
        .map(|l| Allergen(l.trim().to_owned()))
        .collect();

    (parsed_ingredients, parsed_allergens)
}
