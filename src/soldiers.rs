// The Soldiers, their tribes and statistics.
// Making a club of them is in fightclub

// use yew::prelude::*;

// How many tribes are there to fight?
// (used across multiple modules; should be here or in fightclub.rs I guess
// pub const FIGHT_QUANTITY: usize = 6;

// #[derive(Debug, Copy, Clone)]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Tribe {
   Barbarian,
   Empire,
   Atlantean,
   Frisian,
   Amazon,
   Custom(String),
}

// impl Tribe {
//    pub fn name(&self) -> String {
//       let base = match &self {
//          Tribe::Barbarian => "Barbarians".to_string(),
//          Tribe::Empire => "Empire".to_string(),
//          Tribe::Atlantean => "Atlanteans".to_string(),
//          Tribe::Frisian => "Frisians".to_string(),
//          Tribe::Amazon => "Amazons".to_string(),
//          Tribe::Custom(name) => name.clone(),
//       };
//       format!("{}", base)
//    }
// }

#[derive(Debug, Copy, Clone)]
pub enum _Param {
   Attack,
   Defence,
   Health,
   Evade,
}

/////////////////////////////////////////
// Base Soldiers to go in a fight club //
/////////////////////////////////////////

/// SoldierBase is a (trained) soldier's type; a template to become many identical soldiers to
/// fight.
/// - SoldierType (below) defines the numerical parameters (attack, increment per training level
/// etc) without how many levels a soldier has been trained.
/// - Soldier (below) is an individual soldier's health/attack/etc without name or origin.
// #[derive(Debug, Copy, Clone)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SoldierBase {
   pub params: SoldierType,
   pub tribe: Tribe,
   pub levels: Levels,
}

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub struct Levels {
   pub health: u8,
   pub attack: u8,
   pub defence: u8,
   pub evade: u8,
}

impl SoldierBase {
   /// SoldierBase::new(tribe) makes a fully trained soldier
   pub fn new(tribe: Tribe) -> Self {
      let mut base = Self::new_zero(tribe);
      base.levels.health = base.params.health_lvls as u8;
      base.levels.attack = base.params.attack_lvls as u8;
      base.levels.defence = base.params.defence_lvls as u8;
      base.levels.evade = base.params.evade_lvls as u8;
      base
   }

   /// SoldierBase::new_zero(tribe) makes a soldier with no training
   pub fn new_zero(tribe: Tribe) -> Self {
      let levels = Levels {
         health: 0,
         attack: 0,
         defence: 0,
         evade: 0,
      };
      let params = match tribe {
         Tribe::Barbarian => BARBARIAN,
         Tribe::Empire => EMPIRE,
         Tribe::Atlantean => ATLANTEAN,
         Tribe::Frisian => FRISIAN,
         Tribe::Amazon => AMAZON,
         Tribe::Custom(_) => AMAZON,
      };
      SoldierBase {
         params,
         tribe,
         levels,
      }
   }

   pub fn soldier(&self) -> Soldier {
      Soldier::new_anylvl(
         self.params,
         self.levels.health as i32,
         self.levels.attack as i32,
         self.levels.defence as i32,
         self.levels.evade as i32,
      )
   }

   pub fn name(&self) -> String {
      let base = match &self.tribe {
         Tribe::Barbarian => "Barbarians".to_string(),
         Tribe::Empire => "Empire".to_string(),
         Tribe::Atlantean => "Atlanteans".to_string(),
         Tribe::Frisian => "Frisians".to_string(),
         Tribe::Amazon => "Amazons".to_string(),
         Tribe::Custom(name) => name.clone(),
      };
      format!("{}", base)
   }

   pub fn lvls(&self) -> String {
      format!(
         "{} {}\n{} {}",
         self.levels.attack, self.levels.defence, self.levels.health, self.levels.evade
      )
   }

   pub fn attack(&self) -> String {
      format!(
         "{}-{}",
         (self.params.attack_base + self.params.attack_incr * self.levels.attack as i32),
         (self.params.attack_maxm + self.params.attack_incr * self.levels.attack as i32)
      )
   }

   pub fn defence(&self) -> String {
      format!(
         "{}",
         (self.params.defence_base + self.params.defence_incr * self.levels.defence as i32)
      )
   }

   pub fn health(&self) -> String {
      format!(
         "{}",
         (self.params.health_base + self.params.health_incr * self.levels.health as i32)
      )
   }

   pub fn evade(&self) -> String {
      format!(
         "{}",
         (self.params.evade_base + self.params.evade_incr * self.levels.evade as i32)
      )
   }

   // pub fn _name_html(&self) -> Html {
   //    html! {
   //       <div> { self.name() }
   //       <table>
   //          <tr>
   //             <td>{self.levels.attack}</td>
   //             <td>{self.levels.defence}</td>
   //          </tr><tr>
   //             <td>{self.levels.health}</td>
   //             <td>{self.levels.evade}</td>
   //          </tr>
   //       </table>
   //       </div>
   //    }
   // }

   // pub fn _name_html_long(&self) -> Html {
   //    html! {
   //       <table>
   //          <tr>
   //             <td></td>
   //             <td>{self.levels.attack}</td>
   //             <td>{self.levels.defence}</td>
   //          </tr><tr>
   //             <td>{self.name()}</td>
   //             <td>{self.levels.health}</td>
   //             <td>{self.levels.evade}</td>
   //          </tr>
   //       </table>
   //    }
   // }

   // pub fn name_two_lines(&self) -> Html {
   //    html! { <>{self.name()}<br/>
   //       <span class="small">
   //       {self.levels.attack}{", "}
   //       {self.levels.defence}{", "}
   //       {self.levels.health}{", "}
   //       {self.levels.evade}</span></>
   //    }
   // }
}

///////////////////////
// Soldier types and //
// default settings  //
///////////////////////

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub struct SoldierType {
   pub health_lvls: i32,
   pub health_base: i32,
   pub health_incr: i32,

   pub attack_lvls: i32,
   pub attack_base: i32,
   pub attack_maxm: i32,
   pub attack_incr: i32,

   pub defence_lvls: i32,
   pub defence_base: i32,
   pub defence_incr: i32,

   pub evade_lvls: i32,
   pub evade_base: i32,
   pub evade_incr: i32,
}

#[derive(
   Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Soldier {
   pub health: i32,
   pub attack_max: i32,
   pub attack_min: i32,
   pub defence: i32,
   pub evade: i32,
   // pub tribe: Tribe,
}

impl Soldier {
   fn _new(tribe: SoldierType) -> Self {
      Soldier {
         health: tribe.health_base,
         attack_max: tribe.attack_maxm,
         attack_min: tribe.attack_base,
         defence: tribe.defence_base,
         evade: tribe.evade_base,
      }
   }

   pub fn new_anylvl(tribe: SoldierType, h_lvl: i32, a_lvl: i32, d_lvl: i32, e_lvl: i32) -> Self {
      Soldier {
         health: tribe.health_base + tribe.health_incr * h_lvl,
         attack_max: tribe.attack_maxm + tribe.attack_incr * a_lvl,
         attack_min: tribe.attack_base + tribe.attack_incr * a_lvl,
         defence: tribe.defence_base + tribe.defence_incr * d_lvl,
         evade: tribe.evade_base + tribe.evade_incr * e_lvl,
      }
   }

   fn _new_max(tribe: SoldierType) -> Self {
      Soldier {
         health: tribe.health_base + tribe.health_incr * tribe.health_lvls,
         attack_max: tribe.attack_maxm + tribe.attack_incr * tribe.attack_lvls,
         attack_min: tribe.attack_base + tribe.attack_incr * tribe.attack_lvls,
         defence: tribe.defence_base + tribe.defence_incr * tribe.defence_lvls,
         evade: tribe.evade_base + tribe.evade_incr * tribe.evade_lvls,
      }
   }
}

pub const BARBARIAN: SoldierType = SoldierType {
   health_lvls: 3,
   health_base: 13000,
   health_incr: 3000,

   attack_lvls: 5,
   attack_base: 1200,
   attack_maxm: 1600,
   attack_incr: 850,

   defence_lvls: 0,
   defence_base: 3,
   defence_incr: 4,

   evade_lvls: 2,
   evade_base: 25,
   evade_incr: 16,
};

pub const EMPIRE: SoldierType = SoldierType {
   health_lvls: 4,
   health_base: 13000,
   health_incr: 2100,

   attack_lvls: 4,
   attack_base: 1300,
   attack_maxm: 1500,
   attack_incr: 920,

   defence_lvls: 0,
   defence_base: 5,
   defence_incr: 5,

   evade_lvls: 2,
   evade_base: 30,
   evade_incr: 16,
};

pub const ATLANTEAN: SoldierType = SoldierType {
   health_lvls: 1,
   health_base: 13500,
   health_incr: 4000,

   attack_lvls: 4,
   attack_base: 1200,
   attack_maxm: 1600,
   attack_incr: 920,

   defence_lvls: 2,
   defence_base: 6,
   defence_incr: 8,

   evade_lvls: 2,
   evade_base: 30,
   evade_incr: 17,
};

pub const FRISIAN: SoldierType = SoldierType {
   health_lvls: 2,
   health_base: 12250,
   health_incr: 2955,

   attack_lvls: 6,
   attack_base: 1300,
   attack_maxm: 1600,
   attack_incr: 1006,

   defence_lvls: 2,
   defence_base: 4,
   defence_incr: 16,

   evade_lvls: 0,
   evade_base: 35,
   evade_incr: 0,
};

pub const AMAZON: SoldierType = SoldierType {
   health_lvls: 3,
   health_base: 13000,
   health_incr: 2025,

   attack_lvls: 2,
   attack_base: 1200,
   attack_maxm: 1600,
   attack_incr: 800,

   defence_lvls: 2,
   defence_base: 5,
   defence_incr: 10,

   evade_lvls: 3,
   evade_base: 30,
   evade_incr: 15,
};

///////////
// TESTS //
///////////

// Lol like we bother with tests //

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn iterator_len_correct() {
      let fc = FightClub::new();
      let fci = fc.iter_fights();
      assert_eq!(fci.count(), FIGHT_QUANTITY * (FIGHT_QUANTITY + 1) / 2);
   }
}
