// A group of soldiers, and ways to make them fight. Actual fight simulations are in fight.rs

// need:
// - method to derive list/iterator of match-ups
// - method for ordered list/iterator of match-ups (basic stats)

use crate::fight;
use crate::results::Datum;
use crate::soldiers::*;

////////////////////////
// A Club of Fighters //
////////////////////////

// #[derive(Debug, Copy, Clone)]
// pub struct FightClub([SoldierBase; FIGHT_QUANTITY]);
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FightClub(pub Vec<SoldierBase>);

impl FightClub {
   // make an iterator of all the fight combinations
   pub fn iter_fights(&self) -> FightIter {
      FightIter {
         i: 1,
         j: 0,
         n: self.0.len(),
      }
   }

   // new club, default stuff
   pub fn new() -> Self {
      FightClub(vec![
         SoldierBase::new(Tribe::Barbarian),
         SoldierBase::new(Tribe::Empire),
         SoldierBase::new(Tribe::Atlantean),
         SoldierBase::new(Tribe::Frisian),
         SoldierBase::new(Tribe::Amazon),
         // SoldierBase::new(Tribe::Custom),
      ])
   }

   /// get the list of match-ups, in order of the fight club, for display
   // pub fn matchup(&self) -> Iter<'_, (SoldierBase, SoldierBase)> {
   //    self.iter_fights().map(|(i, j)| (self.0[i], self.0[j]))
   // } // maybe? Won't fill n^2 grid, only n(n+1)/2

   /// get the list of fights to happen, in order, for calculations
   // pub fn _ordered_fights(&self) -> Iter<'_, (Soldier, Soldier)> {
   //    // let ord_fights = Vec::with_capacity((self.len() * (self.len() + 1)) / 2);
   //    self
   //       .iter_fights()
   //       .map(|(i, j)| {
   //          let (s1, s2) = (self.0[i].soldier(), self.0[j].soldier());
   //          if s1 > s2 {
   //             (s1, s2)
   //          } else {
   //             (s2, s1)
   //          }
   //       })
   //       .collect()
   //       .sort_unstable()
   //       .dedup() // seems wasteful to sort then dedup. Maybe the compiler will do the work I gave up on?
   //       .into_iter()
   // }

   /// Get an iterator over the fights to happen, for calculations. Soldier pairs are ordered
   /// within the pair, so (s1,s2) = (s2,s1)
   ///
   // This is where I most struggle with Rust: "You need an obscure interim type for the type
   // signature"; okay; "no, it has to be in a dynamic thing" ah, okay, "and the bounds are wrong"
   // is that memory lifetime stuff? structural bounds?
   // ...ah blow it, I'll write it inline where it's used.
   //
   // pub fn fights(
   //    &self,
   // ) -> std::iter::Map<FightIter, dyn FnMut(usize, usize) -> (Soldier, Soldier)> {
   //    self.iter_fights().map(|(i, j)| {
   //       let (s1, s2) = (self.0[i].soldier(), self.0[j].soldier());
   //       if s1 > s2 {
   //          (s1, s2)
   //       } else {
   //          (s2, s1)
   //       }
   //    })
   // }
   //
   // therefore need new public methods

   // parallel-fight them against each other, and enter results into table
   pub fn _fight_all_parallel(
      &self,
      results_total_wins: &mut [Datum],
      results_total_health_remaining: &mut [Datum],
      results_total_health_stats: &mut [Datum],
      results_asym_wins: &mut [Datum],
      results_health_remaining_a: &mut [Datum],
      results_health_stats_a: &mut [Datum],
      results_health_remaining_d: &mut [Datum],
      results_health_stats_d: &mut [Datum],
      n: i32,
   ) {
      for (i, j) in self.iter_fights() {
         let ij = self.0.len() * i + j;
         let ji = self.0.len() * j + i;

         // fight i vs j
         let outcome = fight::fight_parallel(self.0[i].soldier(), self.0[j].soldier(), n);

         // Total Win Percent
         results_total_wins[ij] = Datum::Percent(outcome.s1_total_win_percent);
         // if i != j {
         results_total_wins[ji] = Datum::Percent(100.0 - outcome.s1_total_win_percent);
         // }

         // Total health remaining
         results_total_health_remaining[ij] = Datum::Percent(outcome.s1_total_health_percent);
         results_total_health_remaining[ji] = Datum::Percent(outcome.s2_total_health_percent);

         // Total health stats
         results_total_health_stats[ij] = Datum::Stat(outcome.s1_total_health_average);
         results_total_health_stats[ji] = Datum::Stat(outcome.s2_total_health_average);

         // Asymmetric win percent
         results_asym_wins[ij] = Datum::Percent(outcome.s1_aggressor_win_percent);
         results_asym_wins[ji] = Datum::Percent(outcome.s2_aggressor_win_percent);

         // Aggressors' Health remaining
         results_health_remaining_a[ij] = Datum::Percent(outcome.s1_aggressor_health_percent);
         results_health_remaining_a[ji] = Datum::Percent(outcome.s2_aggressor_health_percent);

         // Aggressors' stats A
         results_health_stats_a[ij] = Datum::Stat(outcome.s1_aggressor_health_average);
         results_health_stats_a[ji] = Datum::Stat(outcome.s2_aggressor_health_average);

         // Defenders' Health remaining B
         results_health_remaining_d[ij] = Datum::Percent(outcome.s1_defender_health_percent);
         results_health_remaining_d[ji] = Datum::Percent(outcome.s2_defender_health_percent);

         // Defenders' Health stats B
         results_health_stats_d[ij] = Datum::Stat(outcome.s1_defender_health_average);
         results_health_stats_d[ji] = Datum::Stat(outcome.s2_defender_health_average);
      }
   }

   pub fn _iter(&self) -> std::slice::Iter<'_, SoldierBase> {
      self.0.iter()
   }
   pub fn _len(&self) -> usize {
      self.0.len()
   }
   pub fn remove(&mut self, i: usize) {
      self.0.remove(i);
   }
   pub fn _add_soldier(&mut self, t: Tribe) {
      self.0.push(SoldierBase::new(t));
   }
   pub fn add(&mut self, s: SoldierBase) {
      self.0.push(s);
   }
   // pub fn lvl_up(&mut self, i: usize, param: Param) {
   //    match param {
   //       Param::Attack => {
   //          if self.0[i].levels.attack < self.0[i].params.attack_lvls as u8 {
   //             self.0[i].levels.attack += 1;
   //          }
   //       }
   //       Param::Defence => {
   //          if self.0[i].levels.defence < self.0[i].params.defence_lvls as u8 {
   //             self.0[i].levels.defence += 1;
   //          }
   //       }
   //       Param::Health => {
   //          if self.0[i].levels.health < self.0[i].params.health_lvls as u8 {
   //             self.0[i].levels.health += 1;
   //          }
   //       }
   //       Param::Evade => {
   //          if self.0[i].levels.evade < self.0[i].params.evade_lvls as u8 {
   //             self.0[i].levels.evade += 1;
   //          }
   //       }
   //    }
   // }
   // pub fn lvl_down(&mut self, i: usize, param: Param) {
   //    match param {
   //       Param::Attack => {
   //          if self.0[i].levels.attack > 0 {
   //             self.0[i].levels.attack -= 1;
   //          }
   //       }
   //       Param::Defence => {
   //          if self.0[i].levels.defence > 0 {
   //             self.0[i].levels.defence -= 1;
   //          }
   //       }
   //       Param::Health => {
   //          if self.0[i].levels.health > 0 {
   //             self.0[i].levels.health -= 1;
   //          }
   //       }
   //       Param::Evade => {
   //          if self.0[i].levels.evade > 0 {
   //             self.0[i].levels.evade -= 1;
   //          }
   //       }
   //    }
   // }
}

//////////////////////////////////////
// Iterator for fight combinations  //
// as references into the fightclub //
// ///////////////////////////////////

pub struct FightIter {
   i: usize,
   j: usize,
   n: usize,
}

impl Iterator for FightIter {
   type Item = (usize, usize);

   fn next(&mut self) -> Option<Self::Item> {
      if self.j < self.n {
         self.j += 1;
         Some((self.i - 1, self.j - 1))
      } else if self.i < self.n {
         self.i += 1;
         self.j = self.i;
         Some((self.i - 1, self.j - 1))
      } else {
         None
      }
   }
}
