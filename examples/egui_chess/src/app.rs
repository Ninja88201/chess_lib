use eframe::egui;
use egui::TextureHandle;

mod render;
mod helper;
use helper::UIState;
mod input;

use chess_lib::{Board, Tile};
use rand::rngs::ThreadRng;

pub struct ChessApp {
    board: Board,
    rand: ThreadRng,
    flipped: bool,
    selected: Option<Tile>,

    atlas: TextureHandle,
    board_size: f32,

    ui_state: UIState,
    should_close: bool,
}
impl ChessApp
{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        let texture = crate::utils::load_texture_from_png(ctx);
        Self {
            board: Board::new(),
            rand: rand::rng(),
            flipped: false,
            selected: None,
            atlas: texture,
            board_size: 400.0,
            ui_state: UIState::Playing,
            should_close: false,
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        // Move history sidebar
        egui::SidePanel::right("move_history")
            .min_width(150.0)        // minimum width of the panel
            .max_width(400.0)        // maximum width it can grow to
            .resizable(true)         // user can resize it manually too
            .show(ctx, |ui| {
                ui.heading("Move History");

                egui::ScrollArea::vertical()
                    .auto_shrink([false, true]) // horizontal size won't shrink
                    .show(ui, |ui| {
                        // Wrap label in a horizontal scroll so very long lines can be scrolled instead of clipped
                        ui.add(
                            egui::Label::new(self.board.get_move_history())
                        );
                    });
            });
        self.render_board(ctx);
    }
}
