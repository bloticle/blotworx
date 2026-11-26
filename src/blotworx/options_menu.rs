//! User preferences. Profiles change presentation only, never game assets.
use crate::render_support::scene_finish::Settings;

/// The Blotworx F10 options panel (shared by the EM1 and EM2 front ends): slides down from the
/// top, tabs Graphics / Display & controls / Game / Tools. `contents` draws the selected tab.
pub fn panel(ctx:&egui::Context, open:&mut bool, tab:&mut usize, contents:impl FnOnce(&mut egui::Ui, usize)) {
    if ctx.input(|i|i.key_pressed(egui::Key::F10)){*open=!*open;}
    let progress=ctx.animate_bool_with_time(egui::Id::new("options_slide"),*open,0.22);
    if progress<=0.001{return}
    let screen=ctx.content_rect();let height=(screen.height()-32.).min(430.).max(160.);
    let width=(screen.width()-32.).min(960.).max(240.);
    egui::Area::new(egui::Id::new("blotworx_options")).order(egui::Order::Foreground)
      .fixed_pos(egui::pos2(screen.center().x-width*0.5,16.-(height+24.)*(1.-progress)))
      .show(ctx,|ui|{
        egui::Frame::new().fill(egui::Color32::from_rgb(17,20,25))
          .stroke(egui::Stroke::new(1.,egui::Color32::from_rgb(62,69,77))).corner_radius(10.)
          .inner_margin(20.).show(ui,|ui|{
            ui.visuals_mut().selection.bg_fill=egui::Color32::from_rgb(44,94,94);
            ui.visuals_mut().widgets.inactive.bg_fill=egui::Color32::from_rgb(28,33,40);
            ui.spacing_mut().item_spacing=egui::vec2(10.,8.);
            ui.set_width(width-40.);ui.set_min_height(height-40.);
            ui.horizontal_wrapped(|ui|{ui.strong("BLOTWORX");ui.add_space(24.);
              for (i,n) in ["Graphics","Display & controls","Game","Tools","Multiplayer"].iter().enumerate(){ui.selectable_value(tab,i,*n);}
              ui.with_layout(egui::Layout::right_to_left(egui::Align::Center),|ui|{if ui.button("Close · F10").clicked(){*open=false;}});
            });ui.separator();ui.add_space(10.);
            let selected=*tab;
            egui::ScrollArea::vertical().max_height(height-100.).show(ui,|ui|contents(ui,selected));
          });
      });
}
#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum Profile {
    #[default]
    Classic,
    ClassicPlus,
    Remix,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub profile: Profile,
    pub grades: [Settings; 3],
    pub surfaces: [crate::render_support::surface_lighting::Settings; 3],
    pub hud: bool,
    pub sensitivity: f32,
    pub invert_y: bool,
    pub distance: f32,
    pub fov: f32,
    pub fullscreen: bool,
    pub smooth_textures: bool,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            profile: Profile::Classic,
            surfaces: crate::render_support::surface_lighting::Settings::presets(),
            grades: [
                Settings::default(),
                Settings {
                    contrast: 1.03,
                    saturation: 1.04,
                    bloom: 0.12,
                    smoothing: 0.2,
                    sharpen: 0.15,
                    ..Default::default()
                },
                Settings {
                    exposure: 0.12,
                    contrast: 1.12,
                    saturation: 1.12,
                    bloom: 0.55,
                    threshold: 0.65,
                    vignette: 0.16,
                    smoothing: 0.3,
                    rolloff: 0.15,
                    warmth: 0.025,
                    radius: 5.,
                    sharpen: 0.25,
                    ..Default::default()
                },
            ],
            hud: true,
            sensitivity: 1.,
            invert_y: false,
            distance: 5.,
            fov: 60.,
            fullscreen: false,
            smooth_textures: false,
        }
    }
}
impl Preferences {
    pub fn index(&self) -> usize {
        match self.profile {
            Profile::Classic => 0,
            Profile::ClassicPlus => 1,
            Profile::Remix => 2,
        }
    }
    pub fn surface(&self) -> crate::render_support::surface_lighting::Settings {
        if self.profile == Profile::Classic {
            Default::default()
        } else {
            let mut s = self.surfaces[self.index()];
            s.mode = self.index() as u32;
            s
        }
    }
    pub fn grade(&self) -> Settings {
        self.grades[self.index()]
    }
    fn path() -> std::path::PathBuf {
        // Use platform application-data directory for preferences
        #[cfg(windows)]
        let storage = std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from);
        #[cfg(not(windows))]
        let storage = dirs::data_local_dir();
        storage
            .unwrap_or_else(std::env::temp_dir)
            .join("BlueThinner")
            .join("blotworx-options.json")
    }
    pub fn load() -> Self {
        let mut s: Self = std::fs::read(Self::path())
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default();
        s.sanitize();
        s
    }
    pub fn sanitize(&mut self) {
        fn clamp(x: &mut f32, a: f32, b: f32, d: f32) {
            *x = if x.is_finite() { x.clamp(a, b) } else { d }
        }
        clamp(&mut self.sensitivity, 0.2, 3., 1.);
        clamp(&mut self.distance, 2., 16., 5.);
        clamp(&mut self.fov, 40., 100., 60.);
        for s in &mut self.surfaces {
            s.sanitize();
        }
        for s in &mut self.grades {
            clamp(&mut s.exposure, -2., 2., 0.);
            clamp(&mut s.contrast, 0.5, 1.5, 1.);
            clamp(&mut s.saturation, 0., 2., 1.);
            clamp(&mut s.gamma, 0.6, 1.6, 1.);
            clamp(&mut s.bloom, 0., 1.5, 0.);
            clamp(&mut s.threshold, 0.3, 1., 0.75);
            clamp(&mut s.vignette, 0., 0.6, 0.);
            clamp(&mut s.sharpen, 0., 1., 0.);
            clamp(&mut s.radius, 1., 12., 3.);
            clamp(&mut s.warmth, -0.3, 0.3, 0.);
            clamp(&mut s.rolloff, 0., 0.5, 0.);
            clamp(&mut s.smoothing, 0., 1., 0.);
        }
    }
    /// "Display & controls" tab of the F10 menu. Returns true when the camera distance changed.
    pub fn display_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
        if ui.checkbox(&mut self.fullscreen,"Fullscreen").changed(){ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));}
        ui.horizontal_wrapped(|ui|{ui.label("Window size");for (w,h) in [(1280.,720.),(1920.,1080.),(2560.,1440.)]{if ui.button(format!("{} × {}",w as u32,h as u32)).clicked(){self.fullscreen=false;ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(w,h)));}}});
        ui.checkbox(&mut self.hud,"Show HUD");
        ui.add(egui::Slider::new(&mut self.sensitivity,0.2..=3.).text("Camera sensitivity"));
        ui.checkbox(&mut self.invert_y,"Invert vertical camera");
        let distance=ui.add(egui::Slider::new(&mut self.distance,2.0..=16.0).text("Camera distance")).changed();
        ui.add(egui::Slider::new(&mut self.fov,40.0..=100.0).text("Gameplay field of view"));
        ui.add_space(12.);ui.weak("WASD · Move     Space · Jump     Q / E · Paint / Thinner     F · Spin");
        ui.weak("Right drag · Camera     Wheel · Zoom     Esc · Pause     F3 · Debug");
        ui.weak("F9 · Multiplayer (16p)     T · Text Chat     Hold V · Proximity Voice");
        distance
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let p = Self::path();
        std::fs::create_dir_all(p.parent().unwrap())?;
        let tmp = p.with_extension("tmp");
        std::fs::write(&tmp, serde_json::to_vec_pretty(self)?)?;
        std::fs::rename(tmp, p)?;
        Ok(())
    }
    pub fn graphics(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (p, n) in [
                (Profile::Classic, "Classic"),
                (Profile::ClassicPlus, "Classic+"),
                (Profile::Remix, "Remix"),
            ] {
                ui.selectable_value(&mut self.profile, p, n);
            }
        });
        ui.add_space(8.);
        ui.label(match self.profile {
            Profile::Classic => "Original materials, lighting and color.",
            Profile::ClassicPlus => {
                "Authored color with soft surface lighting and restrained reflections."
            }
            Profile::Remix => "Per-pixel lighting, material highlights and geometry shadows.",
        });
        ui.add_space(12.);
        ui.checkbox(&mut self.smooth_textures, "Smooth textures")
            .on_hover_text("Use bilinear filtering with blended mip levels on scene textures.");
        let i = self.index();
        if i != 0 {
            egui::CollapsingHeader::new("Lighting & materials")
                .default_open(true)
                .show(ui, |ui| self.surfaces[i].ui(ui));
        }
        egui::CollapsingHeader::new("Color & effects")
            .default_open(i == 0)
            .show(ui, |ui| {
                let s = &mut self.grades[i];
                ui.columns(2, |columns| {
                    columns[0].strong("Color");
                    egui::Grid::new("graphics_color")
                        .num_columns(2)
                        .spacing([18., 10.])
                        .show(&mut columns[0], |ui| {
                            for (n, v, r) in [
                                ("Exposure", &mut s.exposure, -2.0..=2.0),
                                ("Contrast", &mut s.contrast, 0.5..=1.5),
                                ("Saturation", &mut s.saturation, 0.0..=2.0),
                                ("Gamma", &mut s.gamma, 0.6..=1.6),
                                ("Temperature", &mut s.warmth, -0.3..=0.3),
                                ("Highlight rolloff", &mut s.rolloff, 0.0..=0.5),
                            ] {
                                ui.label(n);
                                ui.add(egui::Slider::new(v, r));
                                ui.end_row();
                            }
                        });
                    columns[1].strong("Effects");
                    egui::Grid::new("graphics_effects")
                        .num_columns(2)
                        .spacing([18., 10.])
                        .show(&mut columns[1], |ui| {
                            for (n, v, r) in [
                                ("Bloom", &mut s.bloom, 0.0..=1.5),
                                ("Bloom threshold", &mut s.threshold, 0.3..=1.0),
                                ("Bloom radius", &mut s.radius, 1.0..=12.0),
                                ("Vignette", &mut s.vignette, 0.0..=0.6),
                                ("Sharpening", &mut s.sharpen, 0.0..=1.0),
                                ("Edge smoothing", &mut s.smoothing, 0.0..=1.0),
                            ] {
                                ui.label(n);
                                ui.add(egui::Slider::new(v, r));
                                ui.end_row();
                            }
                        });
                });
                ui.add_space(12.);
            });
        if ui.button("Reset this profile").clicked() {
            self.grades[i] = Self::default().grades[i];
            self.surfaces[i] = Self::default().surfaces[i];
        }
    }
}
