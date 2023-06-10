use crate::updater::{config::Config, updater, current_game::CurrentGame, wait_1m};
use eframe::egui;
use std::{str::FromStr, sync::{Arc, Mutex}, thread, };


pub struct TemplateApp {
    open_dir_dialog: Option<egui_file::FileDialog>,

    // 各种配置项
    proxy_url: String,
    opened_dir: String,
    tiles: bool,
    sounds: bool,
    backup_config: bool,
    backup_save: bool,
    backup_templates: bool,

    // 本工具的配置结构
    cfg: Arc<Mutex<Config>>,
    // 当前的游戏版本
    current_version: Arc<Mutex<String>>,
    // 最新的游戏版本
    newst_version: Arc<Mutex<String>>,
    // 更新中的进度状态
    log: Arc<Mutex<String>>,

    // 更新进行中为true
    start_updater: Arc<Mutex<bool>>,
    // 检查版本中为true
    check_version: Arc<Mutex<bool>>,
}

// impl Default for TemplateApp {
//     fn default() -> Self {
//         Self {
//             rt: None,
//             proxy_url: "".to_owned(),
//             opened_dir: "".to_owned(),
//             tiles: true,
//             sounds: false,
//             backup_config: true,
//             backup_save: true,
//             backup_templates: true,
//             start_updater: false,
//             check_version: false,
//             open_dir_dialog: None,
//         }
//     }
// }

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "myfont".to_owned(),
            egui::FontData::from_static(include_bytes!(r"C:\Windows\Fonts\msyh.ttc")),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "myfont".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("myfont".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        Self {
            proxy_url: "".to_owned(),
            opened_dir: "".to_owned(),
            tiles: true,
            sounds: false,
            backup_config: true,
            backup_save: true,
            backup_templates: true,
            start_updater: Arc::new(Mutex::new(false)),
            check_version: Arc::new(Mutex::new(false)),
            current_version: Arc::new(Mutex::new("未检测".to_string())),
            newst_version: Arc::new(Mutex::new("未检测".to_string())),
            log: Arc::new(Mutex::new("".to_string())),
            cfg: Arc::new(Mutex::new(Config::new())),
            open_dir_dialog: None,
        }

        // Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // let Self {
        //     tiles,
        //     sounds,
        //     backup_config,
        //     backup_save,
        //     backup_templates,
        //     ..
        // } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 _frame.close();
        //             }
        //         });
        //     });
        // });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 1.0;
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.vertical_centered(|ui| {
                ui.heading("更新CDDA至最新实验版");
            });

            ui.add_space(20.);
            ui.horizontal(|ui| {
                if (ui.button("选择游戏安装目录")).clicked() {
                    let mut dialog = egui_file::FileDialog::select_folder(Some(
                        std::path::PathBuf::from_str(&self.opened_dir).unwrap(),
                    ))
                    .show_rename(false)
                    .show_new_folder(false);
                    dialog.open();
                    self.open_dir_dialog = Some(dialog);
                }
                ui.text_edit_singleline(&mut self.opened_dir)
                    .on_hover_text("CDDA的安装目录");
                if let Some(dialog) = &mut self.open_dir_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            if file.as_os_str() != "" {
                                let a = std::fs::canonicalize(file)
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                if a != *self.opened_dir {
                                    self.opened_dir = a;
                                    self.cfg.lock().unwrap().current_game = CurrentGame::new(&self.opened_dir);
                                    *self.check_version.lock().unwrap() = true;
                                }
                            }
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("github加速器网址：");
                ui.text_edit_singleline(&mut self.proxy_url)
                    .on_hover_text("github加速器网址，没有就别填");
            });
            if *self.check_version.lock().unwrap() {
                ui.label("检查版本中...");
                ui.add(egui::Spinner::new());
                let check_version_clone = self.check_version.clone();
                let current_version_clone = self.current_version.clone();
                let newst_version_clone = self.newst_version.clone();
                let cfg_clone = self.cfg.clone();
                thread::spawn(move || {
                    // 获取最新发布的版本号
                    // #[cfg(debug_assertions)]
                    // {
                    //     cfg_clone.lock().unwrap().latestbuild.build_number = "2023-05-16-2259".to_string();
                    // }
                    // #[cfg(not(debug_assertions))]
                    cfg_clone.lock().unwrap().latestbuild.pull();

                    *current_version_clone.lock().unwrap() = cfg_clone.lock().unwrap().current_game.build_number.clone();
                    *newst_version_clone.lock().unwrap() = cfg_clone.lock().unwrap().latestbuild.build_number.clone();
                    *check_version_clone.lock().unwrap() = false;
                });
            }
            ui.horizontal(|ui| {
                ui.label("当前游戏版本：");
                ui.label(self.current_version.lock().unwrap().clone());
            });
            ui.horizontal(|ui| {
                ui.label("最新游戏版本：");
                ui.label(self.newst_version.lock().unwrap().clone());
            });

            ui.add_space(20.);
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.label("额外选项");
            });
            egui::Grid::new("extra_option")
                .num_columns(1)
                .spacing([40., 8.])
                .striped(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("贴图版或字符版：");
                        ui.selectable_value(&mut self.tiles, true, "贴图版");
                        if ui.selectable_value(&mut self.tiles, false, "字符版").clicked() {
                            self.sounds = false;
                        }
                        ui.add_enabled_ui(self.tiles, |ui| {
                            ui.checkbox(&mut self.sounds, "带音乐包");
                        });
                    });
                    ui.end_row();

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.backup_save, "保留存档").on_hover_text(
                            "老版本的存档文件夹(也就是游戏根目录的save文件夹)会保留下来",
                        );
                        ui.checkbox(&mut self.backup_config, "保留配置").on_hover_text(
                            "老版本的配置(也就是游戏根目录的config文件夹)会保留下来",
                        );
                        ui.checkbox(&mut self.backup_templates, "保留人物模板").on_hover_text(
                            "老版本的人物模板(也就是游戏根目录的templates文件夹)会保留下来",
                        );
                    });
                    ui.end_row();
                });

            ui.add_space(30.);
            ui.vertical_centered(|ui| {
                
                if ui.button("更新").clicked() {
                    *self.start_updater.lock().unwrap() = true;
                    self.cfg.lock().unwrap().backup_configdir = self.backup_config;
                    self.cfg.lock().unwrap().backup_savedir = self.backup_save;
                    self.cfg.lock().unwrap().sounds = self.sounds;
                    self.cfg.lock().unwrap().tiles = self.tiles;

                    println!("{:?}", self.cfg.lock().unwrap());

                    let log_clone = self.log.clone();
                    let start_updater_clone = self.start_updater.clone();
                    let cfg_clone = self.cfg.clone();
                    thread::spawn(move || {
                        // wait_1m(cfg_clone, log_clone, start_updater_clone);
                        updater(cfg_clone, log_clone, start_updater_clone);
                    });
                };
                if *self.start_updater.lock().unwrap() {
                    ui.add(egui::Spinner::new());
                }
                egui::warn_if_debug_build(ui);
            });

            ui.label(self.log.lock().unwrap().clone());
        });

        // if false {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally choose either panels OR windows.");
        //     });
        // }
    }
}
