// use crate::voyage;
// use crate::voyage::Event;

use crate::fight;
// use crate::fightclub;
use crate::fightclub::FightClub;
use crate::results;
use crate::results::ResultType;
use crate::soldiers::*;

pub struct App {
   fightclub: FightClub,
   fights: Vec<((Soldier, Soldier), results::ParallelFight)>,
   to_fight: Vec<(Soldier, Soldier)>,
   modified_params: bool,
   fighting: bool,
   n: i32,
   remove_from_fightclub: Option<usize>,
   new_soldier: SoldierBase,
   result_show: ResultType,
}

impl Default for App {
   fn default() -> Self {
      Self {
         fightclub: FightClub::new(),
         fights: Vec::new(),
         to_fight: Vec::new(),
         modified_params: true,
         fighting: false,
         n: 100000,
         remove_from_fightclub: None,
         new_soldier: SoldierBase::new(Tribe::Custom("ScaryDudes".to_string())),
         result_show: ResultType::EqPercent,
      }
   }
}

impl App {
   pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
      // n.b. this is where one can customise look&feel with
      // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

      // if let Some(storage)...
      // TODO do store - settings but not results(?)
      // and have 'reset settings' button

      Default::default()
      // let mut e = App::default();
      // e.events[0] = Some(Event::default());
      // return e;
   }
}

const HOVER_FIGHT_N: &str = "How many pairs will fight per match-up. Note that for now, results will not be freshly generated if the number is changed.";

impl eframe::App for App {
   // fn save

   /// Called each time the UI needs repainting
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      // Fight :: The Plan, II:
      //
      // I had a grand plan with ordered vectors and efficiency, but ever-evolving rough edges
      // needing real programming to fix. What ho, this part of the program doesn't need to be
      // efficient. Enter: the easy way!
      //
      // a. (Soldier 1, Soldier 2) is always normalised s1 >= s2, so (s1,s2) = (s2,s1)
      // b. app.fights: a list of soldier-pairs and results
      // c. app.to_fight: a list of soldier pairs
      //
      // 1. If modified soldiers / parameters: iterate over soldier pairs to fights
      // 2. for each not in `fights`, put in (fresh) to_fight
      // 3. If fighting, pop one to_fight, and put plus result in fights
      // (4.) could update a stored table, to avoid re-create results table every frame

      if self.modified_params {
         self.to_fight = self
            .fightclub
            .iter_fights()
            .map(|(i, j)| {
               let (s1, s2) = (self.fightclub.0[i].soldier(), self.fightclub.0[j].soldier());
               if s1 > s2 {
                  (s1, s2)
               } else {
                  (s2, s1)
               }
            })
            .filter(|(s1, s2)| !self.fights.iter().any(|((s3, s4), _)| (s1, s2) == (s3, s4)))
            .collect();
      }

      if self.fighting {
         match self.to_fight.pop() {
            Some((s1, s2)) => self
               .fights
               .push(((s1, s2), fight::fight_parallel(s1, s2, self.n))),
            None => self.fighting = false,
         };
         ctx.request_repaint();
      }

      /////////////////
      // Draw the UI //
      /////////////////

      // When removing a fighter, remove here so the index-based loop doesn't muck up.
      if let Some(i) = self.remove_from_fightclub {
         self.fightclub.remove(i);
         self.remove_from_fightclub = None;
      }

      egui::TopBottomPanel::top("soldiers_lineup")
         // .resizeable(true)
         .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
               ui.heading("Wide Fighter");
               ui.label("Fight simulator for Widelands soldiers.");
            });
            ui.add_space(16.0);
            egui::ScrollArea::horizontal().show(ui, |ui| {
               // egui::Ui::horizontal(ui, |ui| {
               ui.horizontal(|ui| {
                  // for s in &self.fightclub.0 {
                  for i in 0..self.fightclub.0.len() {
                     let s = &mut self.fightclub.0[i];
                     egui::Frame::default()
                        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                        .rounding(ui.visuals().widgets.noninteractive.rounding)
                        .show(ui, |ui| {
                           ui.vertical(|ui| {
                              ui.horizontal(|ui| {
                                 ui.label(s.name());
                                 ui.label(s.lvls());
                                 // ui.add(
                                 //    egui::DragValue::new(&mut s.levels.attack)
                                 //       .speed(0.1)
                                 //       .range(0..=s.params.attack_lvls),
                                 // );
                              });
                              egui::Grid::new(i).show(ui, |ui| {
                                 ui.label("Attack:");
                                 ui.label(s.attack());
                                 ui.end_row();
                                 if s.params.attack_lvls > 0 {
                                    ui.label("lvl");
                                    ui.add(egui::Slider::new(
                                       &mut s.levels.attack,
                                       0..=s.params.attack_lvls.try_into().unwrap(),
                                    ));
                                 } else {
                                    ui.label("");
                                    ui.label("(no training)");
                                 }

                                 ui.end_row();
                                 ui.label("Defence:");
                                 ui.label(s.defence());
                                 ui.end_row();
                                 if s.params.defence_lvls > 0 {
                                    ui.label("lvl");
                                    ui.add(egui::Slider::new(
                                       &mut s.levels.defence,
                                       0..=s.params.defence_lvls.try_into().unwrap(),
                                    ));
                                 } else {
                                    ui.label("");
                                    ui.label("(no training)");
                                 }
                                 ui.end_row();
                                 ui.label("Health:");
                                 ui.label(s.health());
                                 ui.end_row();
                                 if s.params.health_lvls > 0 {
                                    ui.label("lvl");
                                    ui.add(egui::Slider::new(
                                       &mut s.levels.health,
                                       0..=s.params.health_lvls.try_into().unwrap(),
                                    ));
                                 } else {
                                    ui.label("");
                                    ui.label("(no training)");
                                 }
                                 ui.end_row();
                                 ui.label("Evade:");
                                 ui.label(s.evade());
                                 ui.end_row();
                                 if s.params.evade_lvls > 0 {
                                    ui.label("lvl");
                                    ui.add(egui::Slider::new(
                                       &mut s.levels.evade,
                                       0..=s.params.evade_lvls.try_into().unwrap(),
                                    ));
                                 } else {
                                    ui.label("");
                                    ui.label("(no training)");
                                 }
                                 ui.end_row();
                              });
                              if ui.button("Remove").clicked() {
                                 // FIXED I know what's happening here:
                                 // This loop sets up fighter readouts by index in the vector
                                 // then the button removes one if clicked
                                 // then the result is displayed... so all later indices are
                                 // invalid
                                 // self.fightclub.remove(i);
                                 self.remove_from_fightclub = Some(i);
                              }
                           });
                        });
                  }
               });
               ui.add_space(16.0);
            });
         });

      // Left panel: add new soldier
      egui::SidePanel::left("setup").show(ctx, |ui| {
         egui::ScrollArea::vertical().show(ui, |ui| {
            // ui.label(format!("{} fights to simulate.", self.to_fight.len()));
            // if ui.button("¡FIGHT!").clicked() {
            //    self.fighting = true;
            // }
            //
            // ui.horizontal(|ui| {
            //    ui.label("Each pair will fight");
            //    ui.add(egui::DragValue::new(&mut self.n).speed(1000))
            //       .on_hover_text(HOVER_FIGHT_N);
            //    ui.label("times.");
            // });
            // ui.add(egui::Slider::new(&mut self.n, 1000..=1000000).logarithmic(true))
            //    .on_hover_text(HOVER_FIGHT_N);
            //
            // ui.add_space(16.0);
            //Add new soldeier
            ui.label("Add a new soldier");
            egui::Frame::default()
               .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
               .rounding(ui.visuals().widgets.noninteractive.rounding)
               .show(ui, |ui| {
                  // Tribe combo box
                  egui::ComboBox::from_id_salt("new_tribe")
                     .selected_text(match self.new_soldier.tribe {
                        Tribe::Custom(_) => "Custom".to_string(),
                        _ => self.new_soldier.name(),
                     })
                     .show_ui(ui, |ui| {
                        if ui
                           .selectable_value(
                              &mut self.new_soldier.tribe,
                              Tribe::Barbarian,
                              "Barbarian",
                           )
                           .clicked
                        {
                           self.new_soldier = SoldierBase::new(Tribe::Barbarian);
                        }
                        if ui
                           .selectable_value(&mut self.new_soldier.tribe, Tribe::Empire, "Empire")
                           .clicked
                        {
                           self.new_soldier = SoldierBase::new(Tribe::Empire);
                        }
                        if ui
                           .selectable_value(
                              &mut self.new_soldier.tribe,
                              Tribe::Atlantean,
                              "Atlantean",
                           )
                           .clicked
                        {
                           self.new_soldier = SoldierBase::new(Tribe::Atlantean);
                        }
                        if ui
                           .selectable_value(&mut self.new_soldier.tribe, Tribe::Frisian, "Frisian")
                           .clicked
                        {
                           self.new_soldier = SoldierBase::new(Tribe::Frisian);
                        }
                        if ui
                           .selectable_value(&mut self.new_soldier.tribe, Tribe::Amazon, "Amazon")
                           .clicked
                        {
                           self.new_soldier = SoldierBase::new(Tribe::Amazon);
                        }
                        if ui
                           .selectable_value(
                              &mut self.new_soldier.tribe,
                              Tribe::Custom("ScaryDudes".to_string()),
                              "Custom",
                           )
                           .clicked
                        {
                           // could randomly generate a name ;-)
                           // self.new_soldier =
                           //    SoldierBase::new(Tribe::Custom("ScaryDudes".to_string()));
                           self.new_soldier.tribe = Tribe::Custom("ScaryDudes".to_string());
                        }
                     });

                  // The tribe name
                  let mut aname = self.new_soldier.name().clone();
                  let response = ui.add(egui::TextEdit::singleline(&mut aname));
                  if response.changed() {
                     self.new_soldier.tribe = Tribe::Custom(aname);
                  }

                  egui::Grid::new("soldier-builder").show(ui, |ui| {
                     // Attack
                     // base, max, incr, lvls, lvl
                     ui.label("Attack");
                     ui.end_row();
                     ui.label("base:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.attack_base,
                     ));
                     // ui.end_row();
                     ui.label("max:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.attack_maxm,
                     ));
                     ui.end_row();
                     ui.label("increment:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.attack_incr,
                     ));
                     ui.end_row();
                     ui.label("max levels:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.attack_lvls,
                     ));
                     // ui.end_row();
                     ui.label("level:");
                     ui.add(
                        egui::DragValue::new(&mut self.new_soldier.levels.attack)
                           .range(0..=self.new_soldier.params.attack_lvls)
                           .speed(0.1),
                     );
                     ui.end_row();
                     // ui.add_space(2.0);

                     // Defence
                     // base, incr, lvls, lvl
                     ui.label("Defence");
                     ui.end_row();
                     ui.label("base:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.defence_base,
                     ));
                     // ui.end_row();
                     ui.label("incr:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.defence_incr,
                     ));
                     ui.end_row();
                     ui.label("max levels:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.defence_lvls,
                     ));
                     // ui.end_row();
                     ui.label("level:");
                     ui.add(
                        egui::DragValue::new(&mut self.new_soldier.levels.defence)
                           .range(0..=self.new_soldier.params.defence_lvls)
                           .speed(0.1),
                     );
                     ui.end_row();

                     // Health
                     // base, incr, lvls, lvl
                     ui.label("Health");
                     ui.end_row();
                     ui.label("base:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.health_base,
                     ));
                     // ui.end_row();
                     ui.label("incr:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.health_incr,
                     ));
                     ui.end_row();
                     ui.label("max levels:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.health_lvls,
                     ));
                     // ui.end_row();
                     ui.label("level:");
                     ui.add(
                        egui::DragValue::new(&mut self.new_soldier.levels.health)
                           .range(0..=self.new_soldier.params.health_lvls)
                           .speed(0.1),
                     );
                     ui.end_row();

                     // Evade
                     // base, incr, lvls, lvl
                     ui.label("Evade");
                     ui.end_row();
                     ui.label("base:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.evade_base,
                     ));
                     // ui.end_row();
                     ui.label("incr:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.evade_incr,
                     ));
                     ui.end_row();
                     ui.label("max levels:");
                     ui.add(egui::DragValue::new(
                        &mut self.new_soldier.params.evade_lvls,
                     ));
                     // ui.end_row();
                     ui.label("level:");
                     ui.add(
                        egui::DragValue::new(&mut self.new_soldier.levels.evade)
                           .range(0..=self.new_soldier.params.evade_lvls)
                           .speed(0.1),
                     );
                     ui.end_row();
                  });

                  // Add to roster
                  if ui.button("Add").clicked() {
                     self.fightclub.add(self.new_soldier.clone());
                  }
               });
         });
      });

      // Right panel: Fight! action; and results settings
      egui::SidePanel::right("result-settings").show(ctx, |ui| {
         egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(format!("{} fights to simulate.", self.to_fight.len()));
            if ui.button("¡FIGHT!").clicked() {
               self.fighting = true;
            }

            ui.horizontal(|ui| {
               ui.label("Each pair will fight");
               ui.add(egui::DragValue::new(&mut self.n).speed(1000))
                  .on_hover_text(HOVER_FIGHT_N);
               ui.label("times.");
            });
            //TODO don't have maximum? let dragval/type move further than slider?
            ui.add(egui::Slider::new(&mut self.n, 1000..=10000000).logarithmic(true))
               .on_hover_text(HOVER_FIGHT_N);

            ui.add_space(16.0);

            //TODO better ResultType printing (make fn in results module)
            egui::ComboBox::from_label("Results to show")
               .selected_text(self.result_show.shortname())
               .show_ui(ui, |ui| {
                  for rt in results::RESULT_TYPES {
                     ui.selectable_value(&mut self.result_show, rt, rt.shortname())
                        .on_hover_text(rt.description());
                  }
               });

            ui.add_space(16.0);
            // Reset
            if ui.button("Forget Results").clicked() {
               self.fights = Vec::new();
            }
            if ui.button("Reset Page").clicked() {
               *self = App::default();
            }
         });
      });

      egui::CentralPanel::default().show(ctx, |ui| {
         // - show the table, with uncalculated results as ...
         // table_symmetric_winrate(ui, &self.fightclub, &self.fights);

         //TODO match title to result_show
         // let title = "(Equal Fights) % win rate of row soldier vs column soldier".to_string();
         //TODO match hover to result_show
         // let hover = HOVER_EQ_PWIN.to_string();
         // let extract = results::extractor(self.result_show);
         // table(ui, &self.fightclub, &self.fights, extract, title, hover);
         table(
            ui,
            &self.fightclub,
            &self.fights,
            self.result_show.extractor(),
            self.result_show.title(),
            self.result_show.description(),
         );
      });
   }
}

fn table(
   ui: &mut egui::Ui,
   fightclub: &FightClub,
   fights: &Vec<((Soldier, Soldier), results::ParallelFight)>,
   extract: fn(&results::ParallelFight, bool) -> results::Datum,
   // title: String, // or &str? or `impl blah`?
   // hover: String,
   title: &str,
   hover: &str,
) {
   egui::Frame::default()
      .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
      .rounding(ui.visuals().widgets.noninteractive.rounding)
      .show(ui, |ui| {
         //TODO centre this
         ui.label(title).on_hover_text(hover);

         egui::Grid::new("results").show(ui, |ui| {
            ui.label("");
            for x in 0..fightclub.0.len() {
               let s = &fightclub.0[x];
               ui.horizontal(|ui| {
                  ui.label(s.name());
                  ui.label(s.lvls());
               });
            }
            ui.end_row();
            for y in 0..fightclub.0.len() {
               let s = &fightclub.0[y];
               // ui.vertical(|ui| {
               ui.horizontal(|ui| {
                  ui.label(s.name());
                  ui.label(s.lvls());
               });
               for x in 0..fightclub.0.len() {
                  let (s1, s2) = (&fightclub.0[y].soldier(), &fightclub.0[x].soldier());
                  let (ss, flipped) = if s1 > s2 {
                     ((s1, s2), false)
                  } else {
                     ((s2, s1), true)
                  };
                  match fights.iter().find(|((s1, s2), _)| (s1, s2) == ss) {
                     //TODO why do we still have to `format!` the datum?
                     Some((_, r)) => ui.label(format!("{}", extract(r, flipped))),
                     None => ui.label("..."),
                  };
               }
               ui.end_row();
            }
         });
      });
}
