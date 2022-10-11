use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct MyGame {
    // Your game state and assets go here...
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| MyGame { /* ... */ })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
    }
}