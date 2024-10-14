use crate::results;
use crate::soldiers;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Simulate many parallel fights and return mean,stdev of remaining health for each team.
/// half the fights start with s1, half with s2, and wins and survivors' statistics are returned
/// - Parallel means one soldier fights one solder, and the winner goes home. And alternative mode
/// would have soldiers fight in series so an injured soldier might return to fight another.
/// - See also naval warfare in widelands v1.2, where the attacker's soldiers fight to the death
/// but the defender's go indoors after victorious injury and a fresh comrade steps out.
pub fn fight_parallel(
   s1: soldiers::Soldier,
   s2: soldiers::Soldier,
   n: i32, // how many times to fight.
) -> results::ParallelFight {
   // results only handles maximum health <= 65536. Fine for now so just check and panic
   if (s1.health > 65536) || (s2.health > 65536) {
      panic!();
   }

   let mut rng = SmallRng::from_entropy();

   // Fight A: s1 always initiates against s2

   // results:
   let mut survivors1a: Vec<u16> = Vec::new();
   let mut survivors2a: Vec<u16> = Vec::new();

   // fight_loop
   for _ in 0..n / 2 {
      let (s1_wins, remaining_health) = fight(s1, s2, &mut rng);
      if s1_wins {
         survivors1a.push(remaining_health as u16);
      } else {
         survivors2a.push(remaining_health as u16);
      }
   }

   // Fight B: s2 initiates
   // results:
   let mut survivors1b: Vec<u16> = Vec::new();
   let mut survivors2b: Vec<u16> = Vec::new();

   // fight_loop
   for _ in 0..n / 2 {
      let (s2_wins, remaining_health) = fight(s2, s1, &mut rng);
      if s2_wins {
         survivors2b.push(remaining_health as u16);
      } else {
         survivors1b.push(remaining_health as u16);
      }
   }

   // Calculate the results

   let mut fight_results = results::ParallelFight::new();

   let stats = |survivors: &Vec<u16>, base_health: i32| {
      let health_total: u64 = survivors.iter().map(|h| (*h as u64)).sum();
      let health_total_percent = health_total as f32 * 200.0 / n as f32 / base_health as f32;
      let health_mean: i32 = if survivors.len() > 0 {
         (survivors.iter().map(|h| (*h as u64)).sum::<u64>() / (survivors.len() as u64)) as i32
      } else {
         0
      };
      let health_stdev = if survivors.len() > 0 {
         ((survivors
            .iter()
            .map(|h| (*h as i64 - health_mean as i64).pow(2))
            .sum::<i64>() as f32)
            / (survivors.len() as f32 - 1.0))
            .sqrt()
      } else {
         0.0
      };

      // println!(
      // "{} remaining health: army total {:.1}%, individual average {} +- {:.0}",
      // name, health_total_percent, health_mean, health_stdev
      // );
      (health_total_percent, health_mean as f32, health_stdev)
   };

   // because I've got lazy now. Should rework stats and stats_t together not duplicate
   let stats_t = |survivors_a: &Vec<u16>, survivors_b: &Vec<u16>, base_health: i32| {
      let survivors = survivors_a.iter().chain(survivors_b.iter());
      let survivors_len = survivors_a.len() + survivors_b.len();
      let health_total: u64 = survivors.map(|h| (*h as u64)).sum();
      let health_total_percent = health_total as f32 * 100.0 / n as f32 / base_health as f32;
      let health_mean: i32 = if survivors_len > 0 {
         (health_total / (survivors_len as u64)) as i32
      } else {
         0
      };
      let survivors = survivors_a.iter().chain(survivors_b.iter());
      let health_stdev = if survivors_len > 0 {
         ((survivors
            .map(|h| (*h as i64 - health_mean as i64).pow(2))
            .sum::<i64>() as f32)
            / (survivors_len as f32 - 1.0))
            .sqrt()
      } else {
         0.0
      };

      // println!(
      //    "{} remaining health: army total {:.1}%, individual average {} +- {:.0}",
      //    name, health_total_percent, health_mean, health_stdev
      // );
      (health_total_percent, health_mean as f32, health_stdev)
   };

   // println!("\n------\n");

   let wins_a = survivors1a.len() as f32 * 200.0 / n as f32;
   fight_results.s1a_wins(wins_a);
   // println!(
   //    "{0} attacking {1}: {0} win {3} out of {4} = {2:.1}%",
   //    s1_name,
   //    s2_name,
   //    wins_a,
   //    survivors1a.len(),
   //    n / 2
   // );

   let (a, b, c) = stats(&survivors1a, s1.health);
   fight_results.s1a_stats(a, b, c);
   let (a, b, c) = stats(&survivors2a, s2.health);
   fight_results.s2d_stats(a, b, c);

   // println!("\n------\n");

   let wins_b = survivors2b.len() as f32 * 200.0 / n as f32;
   fight_results.s2a_wins(wins_b);
   // println!(
   //    "{0} attacking {1}: {0} win {3} out of {4} = {2:.1}%",
   //    s2_name,
   //    s1_name,
   //    wins_b,
   //    survivors1b.len(),
   //    n / 2
   // );

   let (a, b, c) = stats(&survivors1b, s1.health);
   fight_results.s1d_stats(a, b, c);
   let (a, b, c) = stats(&survivors2b, s2.health);
   fight_results.s2a_stats(a, b, c);

   // println!("\n------\n");

   let wins_total = (survivors1a.len() + survivors1b.len()) as f32 * 100.0 / n as f32;
   fight_results.s1_wins(wins_total);
   // println!(
   //    "{0} and {1} fight: {0} win {3} out of {4} = {2:.1}%",
   //    s1_name,
   //    s2_name,
   //    wins_total,
   //    survivors1a.len() + survivors1b.len(),
   //    n
   // );

   let (a, b, c) = stats_t(&survivors1a, &survivors1b, s1.health);
   fight_results.s1_stats(a, b, c);
   let (a, b, c) = stats_t(&survivors2a, &survivors2b, s2.health);
   fight_results.s2_stats(a, b, c);

   fight_results
}

// fight once, to the death
// returns true if s1 (aggressor) wins
#[inline]
fn fight(mut s1: soldiers::Soldier, mut s2: soldiers::Soldier, rng: &mut SmallRng) -> (bool, i32) {
   loop {
      // s1 swings
      if rng.gen_range(1..=100) > s2.evade {
         // it hits!
         let attack = rng.gen_range(s1.attack_min..=s1.attack_max);
         s2.health = s2.health - (attack * (100 - s2.defence) / 100);
         if s2.health <= 0 {
            return (true, s1.health);
         }
      }

      // s2 swings
      if rng.gen_range(1..=100) > s1.evade {
         // it hits!
         let attack = rng.gen_range(s2.attack_min..=s2.attack_max);
         s1.health = s1.health - (attack * (100 - s1.defence) / 100);
         if s1.health <= 0 {
            return (false, s2.health);
         }
      }
   }
}
