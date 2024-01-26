mod building;
mod hotkey;
mod item;
mod terrain;
mod unit;
pub use hotkey::*;

use crate::*;

impl App {
    pub fn on_tile_click(&self, p: Pos, b: Button) -> Vec<MenuCommand> {
        if let Button::Left = b {
            return match self.menu_state {
                MenuState::Attack(idx) => {
                    let rel_mouse = self.get_world_mouse() - self.cursor.to_f();
                    let cmd = UnitCommand::Attack(idx, rel_mouse);
                    vec![MenuCommand::Command(Command::UnitCommand {
                        command: cmd,
                        pos: self.cursor,
                    })]
                }
                MenuState::DropItem(_) => {
                    let v = vec![
                        Some(Direction::Left),
                        Some(Direction::Right),
                        Some(Direction::Up),
                        Some(Direction::Down),
                        None,
                    ];
                    let mouse = if let Some(x) = self.get_world_mouse().to_i().to_pos() {
                        x
                    } else {
                        return vec![];
                    };

                    let local_pos = |opt_d: &Option<Direction>| -> Vec2i {
                        opt_d.map(|d| *d).unwrap_or(Vec2i::new(0, 0))
                    };
                    let cond = |opt_d: &Option<Direction>| {
                        Some(mouse) == self.cursor.map(|x| x + local_pos(opt_d))
                    };
                    if let Some(opt_dir) = v.into_iter().find(cond) {
                        self.drop_commands(opt_dir)
                    } else {
                        vec![]
                    }
                }
                _ => vec![MenuCommand::Cursor(p)],
            };
        }
        if let Button::Right = b {
            if let Some(d) = [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ]
            .iter()
            .find(|&d| self.cursor.map(|x| x + **d) == Some(p))
            {
                return vec![
                    MenuCommand::Command(Command::UnitCommand {
                        command: UnitCommand::Move(*d),
                        pos: self.cursor,
                    }),
                    MenuCommand::Cursor(p),
                ];
            }
        }
        vec![]
    }

    pub fn drop_commands(&self, opt_dir: Option<Direction>) -> Vec<MenuCommand> {
        let indices = if let MenuState::DropItem(indices) = &self.menu_state {
            indices
        } else {
            return Vec::new();
        };

        let cmd = |idx| Command::UnitCommand {
            command: UnitCommand::DropItem(idx, opt_dir),
            pos: self.cursor,
        };

        let mut indices: Vec<_> = indices.iter().copied().collect();
        indices.sort();
        indices
            .into_iter()
            .rev()
            .map(|idx| MenuCommand::Command(cmd(idx)))
            .chain(iter::once(MenuCommand::StateChange(MenuState::Normal)))
            .collect()
    }

    pub fn generate_widgets(&self) -> Vec<Widget> {
        let ws = self.window_size();
        let mut widgets = Vec::new();

        // left-side pane
        widgets.push(Widget {
            pos: (0.).into(),
            size: ws * (0.3, 1.),
            draw_type: self.pane_color().into(),
            on_click: vec![],
            hotkey: None,
        });

        widgets.extend(self.build_active_player_pane((0.0, 0.0).into()));
        widgets.extend(self.build_item_pane((0.0, 0.10).into()));
        widgets.extend(self.build_unit_pane((0.0, 0.25).into()));
        widgets.extend(self.build_building_pane((0.0, 0.5).into()));
        widgets.extend(self.build_terrain_pane((0.0, 0.75).into()));
        widgets.extend(self.main_button());
        widgets.push(self.msg_label());

        widgets
    }

    fn build_active_player_pane(&self, offset: Vec2f) -> Vec<Widget> {
        let ws = self.window_size();
        let mut widgets = Vec::new();

        let cond = self.world.active_player_ids.contains(&self.player_id);
        let text1 = match cond {
            true => "Your turn",
            false => "Enemies turn",
        };

        let mut text2 = "food until unit: [".to_string();
        self.world
            .pool
            .get_player_ids()
            .iter()
            .for_each(|&PlayerID(pidu)| {
                let current = self.world.invested_food_counter[pidu];
                let required = World::unit_cost_fn(self.world.created_unit_counter[pidu]);
                text2 += &format!("{}: {}/{}, ", pidu, current, required);
            });
        // this removes the trailing comma
        text2.pop();
        text2.pop();
        text2 += "]";

        widgets.push(Widget {
            pos: ws * offset,
            size: ws * (0.025, 0.025),
            draw_type: text1.to_string().into(),
            on_click: vec![],
            hotkey: None,
        });

        widgets.push(Widget {
            pos: ws * (offset + (0., 0.02)),
            size: ws * (0.025, 0.025),
            draw_type: text2.to_string().into(),
            on_click: vec![],
            hotkey: None,
        });

        widgets
    }

    pub fn main_button_cmds(&self) -> Vec<MenuCommand> {
        if let Some(p) = self
            .world
            .find_next_usable_unit_tile(self.cursor, self.player_id)
        {
            vec![MenuCommand::Cursor(p)]
        } else {
            if self.selected_unit().map(|u| u.stamina <= 0).unwrap_or(true) {
                vec![
                    MenuCommand::StateChange(MenuState::Normal),
                    MenuCommand::Command(Command::NextTurn),
                ]
            } else {
                vec![]
            }
        }
    }

    fn main_button(&self) -> Vec<Widget> {
        let ws = self.window_size();
        let mut widgets = Vec::new();

        let s = (ws.x * 0.01).into();
        widgets.push(Widget {
            pos: ws - s,
            size: s,
            draw_type: Color::rgb(100, 100, 100).into(),
            on_click: self.main_button_cmds(),
            hotkey: Some(MAIN_HOTKEY),
        });

        widgets
    }

    fn selected_unit(&self) -> Option<&'_ Unit> {
        self.world.unitmap.get(self.cursor)
    }

    fn pane_color(&self) -> Color {
        if !self.world.active_player_ids.contains(&self.player_id) {
            return Color::rgb(30, 30, 30);
        }

        if let Some(unit) = self.selected_unit() {
            if unit.stamina <= 0 {
                return Color::rgb(70, 70, 70);
            }
        }
        Color::rgb(100, 100, 100)
    }

    fn msg_label(&self) -> Widget {
        let ws = self.window_size();
        let s: Vec2f = (ws.x * 0.013).into();
        Widget {
            pos: Vec2f::new(0.0, ws.y - s.y),
            size: s,
            draw_type: self.msg.clone().into(),
            on_click: vec![],
            hotkey: None,
        }
    }
}
