#[macro_use] extern crate itertools;

use std::collections::HashMap;

mod dice_iterator;
use dice_iterator::dice_iterator;

mod utils;
use utils::{count_values, sum_maps, normalize_map, max_by_key_partial, sum_tuples};

fn compute_outcome(attacker_dice: &Vec<u32>, defender_dice: &Vec<u32>) -> (i32, i32) {
    let mut attacker_dice = attacker_dice.clone();
    attacker_dice.sort_unstable();
    let mut defender_dice = defender_dice.clone();
    defender_dice.sort_unstable();

    let it = attacker_dice.iter().rev().zip(defender_dice.iter().rev());
    sum_tuples(it.map(|(a_die, d_die)| if a_die > d_die { (0, -1) } else { (-1, 0) }))
}

struct Preference {
    attacker_ratio: f64
}

impl Preference {
    fn new(ratio: f64) -> Preference {
        Preference { attacker_ratio: ratio }
    }

    fn utility(&self, (outcome_attacker, outcome_defender): (i32, i32)) -> f64 {
        -self.attacker_ratio * (outcome_attacker as f64) + (outcome_defender as f64)
    }
}

fn outcome_stats(probs: &HashMap<(i32, i32), f64>) -> (f64, f64, f64, f64) {
    let (exp_a, exp_d) = sum_tuples(probs.iter().map(|(&(o_a, o_d), &p)| {
        (p * o_a as f64, p * o_d as f64)
    }));
    let (var_a, var_d) = sum_tuples(probs.iter().map(|(&(o_a, o_d), &p)| {
        (p * (o_a as f64 - exp_a).powf(2.0), p * (o_d as f64 - exp_d).powf(2.0))
    }));
    (exp_a, var_a, exp_d, var_d)
}

fn main() {
    let ratio = 0.1;
    let n_attacker = 2;
    let n_defender = 2;
    let pref = Preference::new(ratio);

    let mut strategy = HashMap::new();
    let mut sum_probs = HashMap::new();

    for a_dice in dice_iterator(n_attacker) {
        let n_dice: Vec<u32> = (1..n_defender+1).collect();
        let n_outcomes: Vec<Vec<_>> = n_dice.iter().map(|&n| {
            dice_iterator(n).map(|b| compute_outcome(&a_dice, &b)).collect()
        }).collect();

        let n_utilities: Vec<_> = n_outcomes.iter().map(|os| {
            os.iter().map(|o| pref.utility(*o)).sum::<f64>() / os.len() as f64
        }).collect();

        println!("{:?}", a_dice);
        println!("{:?}", normalize_map(count_values(n_outcomes[0].iter().cloned())));
        println!("{:?}", normalize_map(count_values(n_outcomes[1].iter().cloned())));
        println!("{:?}", n_utilities);

        let zip = izip!(n_dice.into_iter(), n_outcomes.into_iter(), n_utilities.into_iter());
        let (n_best, outcomes, _utility) = max_by_key_partial(zip, |x| x.2).unwrap();


        let mut a_dice_sorted = a_dice.clone();
        a_dice_sorted.sort_unstable();
        strategy.insert(a_dice_sorted, n_best);

        let outcomes_count = count_values(outcomes.into_iter());
        let probs = normalize_map(outcomes_count);
        sum_probs = sum_maps(sum_probs, probs);

    }
    let probs = normalize_map(sum_probs);
    println!("Outcome (attacker loss, defender loss) probabilities:\n{:?}", probs);
    let (exp_a, var_a, exp_d, var_d) = outcome_stats(&probs);
    println!(
        "Expected loss Attacker: {}, Defender: {},\nExpected diff: {}, Variance diff: {}",
        exp_a, exp_d, exp_a - exp_d, var_a + var_d);

    let mut strategy: Vec<_> = strategy.into_iter().collect();
    strategy.sort_by_key(|(k, _)| k.clone());
    println!("Optimal strategy (attacker throw, number of defender dice):\n{:?}", strategy);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fight() {
        let v = vec![
            ((vec![1, 2, 3], vec![1, 2]), (0, -2)),
            ((vec![1, 2, 3], vec![2, 2]), (-1, -1)),
            ((vec![1, 2, 3], vec![3, 2]), (-2, 0)),
            ((vec![1, 2, 3], vec![3]), (-1, 0)),
            ((vec![6], vec![6]), (-1, 0)),
            ((vec![6], vec![1, 6]), (-1, 0)),
        ];

        for ((d_a, d_d), o) in v {
            assert_eq!(compute_outcome(&d_a, &d_d), o);
        }
    }
}
