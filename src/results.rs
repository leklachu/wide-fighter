// use crate::fightclub::FIGHT_QUANTITY;
// use crate::fightclub::FightClub;
// use crate::soldiers::SoldierBase;
// use yew::prelude::*;

/////////////////////
// Results to view //
/////////////////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Datum {
   Nil,
   // Num(i32),
   Stat((f32, f32)),
   Percent(f32),
}

impl Default for Datum {
   fn default() -> Datum {
      Datum::Nil
   }
}

impl std::fmt::Display for Datum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         Datum::Nil => write!(f, "-"),
         // Datum::Num(x) => write!(f, "{}", x),
         Datum::Stat((m, s)) => write!(f, "{:.0}±{:.0}", m, s),
         Datum::Percent(x) => write!(f, "{:.1}%", x),
      }
   }
}

#[derive(Copy, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ResultType {
   EqPercent,
   EqHealth,
   EqHealthAvg,

   AsymPercent,

   AsymAgrHealth,
   AsymAgrHealthAvg,

   AsymDefHealth,
   AsymDefHealthAvg,
}

pub const RESULT_TYPES: [ResultType; 8] = [
   ResultType::EqPercent,
   ResultType::EqHealth,
   ResultType::EqHealthAvg,
   ResultType::AsymPercent,
   ResultType::AsymAgrHealth,
   ResultType::AsymAgrHealthAvg,
   ResultType::AsymDefHealth,
   ResultType::AsymDefHealthAvg,
];

fn eq_percent(result: &ParallelFight, flipped: bool) -> Datum {
   let p = if !flipped {
      result.s1_total_win_percent
   } else {
      100.0 - result.s1_total_win_percent
   };
   Datum::Percent(p)
}

fn eq_health(result: &ParallelFight, flipped: bool) -> Datum {
   let p = if !flipped {
      result.s1_total_health_percent
   } else {
      result.s2_total_health_percent
   };
   Datum::Percent(p)
}

fn eq_health_avg(result: &ParallelFight, flipped: bool) -> Datum {
   let (m, s) = if !flipped {
      result.s1_total_health_average
   } else {
      result.s2_total_health_average
   };
   Datum::Stat((m, s))
}

fn asym_percent(result: &ParallelFight, flipped: bool) -> Datum {
   let p = if !flipped {
      result.s1_aggressor_win_percent
   } else {
      result.s2_aggressor_win_percent
   };
   Datum::Percent(p)
}

fn agr_health(result: &ParallelFight, flipped: bool) -> Datum {
   let p = if !flipped {
      result.s1_aggressor_health_percent
   } else {
      result.s2_aggressor_health_percent
   };
   Datum::Percent(p)
}

fn agr_health_avg(result: &ParallelFight, flipped: bool) -> Datum {
   let (m, s) = if !flipped {
      result.s1_aggressor_health_average
   } else {
      result.s2_aggressor_health_average
   };
   Datum::Stat((m, s))
}

fn def_health(result: &ParallelFight, flipped: bool) -> Datum {
   let p = if !flipped {
      result.s1_defender_health_percent
   } else {
      result.s2_defender_health_percent
   };
   Datum::Percent(p)
}

fn def_health_avg(result: &ParallelFight, flipped: bool) -> Datum {
   let (m, s) = if !flipped {
      result.s1_defender_health_average
   } else {
      result.s2_defender_health_average
   };
   Datum::Stat((m, s))
}

impl ResultType {
   //this should return an iterator, but that's more work for me
   // pub fn all() -> &[ResultType; 8] {
   //    //
   // }

   pub fn extractor(&self) -> fn(&ParallelFight, bool) -> Datum {
      match self {
         ResultType::EqPercent => eq_percent,
         ResultType::EqHealth => eq_health,
         ResultType::EqHealthAvg => eq_health_avg,

         ResultType::AsymPercent => asym_percent,

         ResultType::AsymAgrHealth => agr_health,
         ResultType::AsymAgrHealthAvg => agr_health_avg,

         ResultType::AsymDefHealth => def_health,
         ResultType::AsymDefHealthAvg => def_health_avg,
      }
   }

   pub fn title(&self) -> &str {
      match self {
         ResultType::EqPercent => "(Equal Fights) % win rate of row soldier vs column soldier",
         ResultType::EqHealth => "(Equal Fights) % health remaining of row's team",
         ResultType::EqHealthAvg => {
            "(Equal Fights) average health remaining of row's surviving soldiers"
         }

         ResultType::AsymPercent => "(Asymmetric) % win rate of row soldier",

         ResultType::AsymAgrHealth => "(Asymmetric) % health remaining of row (aggressor) team",
         ResultType::AsymAgrHealthAvg => {
            "(Asymmetric) average health remaining of row (aggressor)'s soldiers"
         }

         ResultType::AsymDefHealth => "(Asymmetric) % health remaining of column (defender) team",
         ResultType::AsymDefHealthAvg => {
            "(Asymmetric) average health remaining of column (defender)'s soldiers"
         }
      }
   }

   pub fn shortname(&self) -> &str {
      match self {
         ResultType::EqPercent => "Win rate",
         ResultType::EqHealth => "Remaining health %",
         ResultType::EqHealthAvg => "Remaining health avg",

         ResultType::AsymPercent => "(Asym) win rate",

         ResultType::AsymAgrHealth => "Aggr health %",
         ResultType::AsymAgrHealthAvg => "Aggr health avg",

         ResultType::AsymDefHealth => "Def health %",
         ResultType::AsymDefHealthAvg => "Def health avg",
      }
   }

   pub fn description(&self) -> &str {
      match self {
         ResultType::EqPercent => "Equal Fights: row and column soldiers initiate attack an equal number of times.\n\n%win rate: The percentage of fights won by the row soldier vs the column soldier.",
         ResultType::EqHealth => "Remaining percent health of the row-tribe's whole team.",
         ResultType::EqHealthAvg => "Average remaining health of the row-tribe's suviving soldiers.\n\n(For reference, max level Barbarians start at 22000 health.)",

         ResultType::AsymPercent => "Asymmetric: row tribe's soldiers always swing first.\n\nPercentage of fights won by the row soldier vs the column soldier.",

         ResultType::AsymAgrHealth => "Asymmetric: row tribe's soldiers always swing first.\n\nRemaining percent health of the row (aggressor)'s team.",
         ResultType::AsymAgrHealthAvg => "Asymmetric: row tribe's soldiers always swing first.\n\nRemaining average health of the row (aggressor) team's surviving soldiers.",

         ResultType::AsymDefHealth => "Asymmetric: row tribe's soldiers always swing first.\n\nRemaining percent health of the column (defender)'s team.",
         ResultType::AsymDefHealthAvg => "Asymmetric: row tribe's soldiers always swing first.\n\nRemaining average health of the column (defender) team's surviving soldiers.",
      }
   }
}

////////////////////////////
// Parallel fight results //
////////////////////////////

/// A `ParallelFight` result is from many identical soldiers s1, each fighting a soldier s2. (All
/// s2 are identical; s1 and s2 can be different.)
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ParallelFight {
   // s1 v s2
   pub s1_aggressor_win_percent: f32,
   pub s1_aggressor_health_percent: f32,
   pub s1_aggressor_health_average: (f32, f32),
   pub s2_defender_health_percent: f32,
   pub s2_defender_health_average: (f32, f32),

   // s2 v s1
   pub s2_aggressor_win_percent: f32,
   pub s2_aggressor_health_percent: f32,
   pub s2_aggressor_health_average: (f32, f32),
   pub s1_defender_health_percent: f32,
   pub s1_defender_health_average: (f32, f32),

   // total
   pub s1_total_win_percent: f32,
   pub s1_total_health_percent: f32,
   pub s1_total_health_average: (f32, f32),
   pub s2_total_health_percent: f32,
   pub s2_total_health_average: (f32, f32),
}

impl ParallelFight {
   pub fn new() -> ParallelFight {
      ParallelFight {
         s1_aggressor_win_percent: 0.0,
         s1_aggressor_health_percent: 0.0,
         s1_aggressor_health_average: (0.0, 0.0),
         s2_defender_health_percent: 0.0,
         s2_defender_health_average: (0.0, 0.0),

         s2_aggressor_win_percent: 0.0,
         s2_aggressor_health_percent: 0.0,
         s2_aggressor_health_average: (0.0, 0.0),
         s1_defender_health_percent: 0.0,
         s1_defender_health_average: (0.0, 0.0),

         s1_total_win_percent: 0.0,
         s1_total_health_percent: 0.0,
         s1_total_health_average: (0.0, 0.0),
         s2_total_health_percent: 0.0,
         s2_total_health_average: (0.0, 0.0),
      }
   }

   // Fight s1 v s2
   pub fn s1a_wins(&mut self, winpc: f32) -> () {
      self.s1_aggressor_win_percent = winpc;
   }
   pub fn s1a_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s1_aggressor_health_percent = total_percent;
      self.s1_aggressor_health_average = (mean, stdev);
   }
   pub fn s2d_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s2_defender_health_percent = total_percent;
      self.s2_defender_health_average = (mean, stdev);
   }

   // Fight s2 v s1
   pub fn s2a_wins(&mut self, winpc: f32) -> () {
      self.s2_aggressor_win_percent = winpc;
   }
   pub fn s2a_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s2_aggressor_health_percent = total_percent;
      self.s2_aggressor_health_average = (mean, stdev);
   }
   pub fn s1d_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s1_defender_health_percent = total_percent;
      self.s1_defender_health_average = (mean, stdev);
   }

   // Fight s1 vs s2 on equal-footing
   pub fn s1_wins(&mut self, winpc: f32) -> () {
      self.s1_total_win_percent = winpc;
   }
   pub fn s1_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s1_total_health_percent = total_percent;
      self.s1_total_health_average = (mean, stdev);
   }
   pub fn s2_stats(&mut self, total_percent: f32, mean: f32, stdev: f32) -> () {
      self.s2_total_health_percent = total_percent;
      self.s2_total_health_average = (mean, stdev);
   }
}
