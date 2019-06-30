use ggez::*;    //tells rust we want to use ggez.

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }

}
fn main(){
    let state = &mut State { dt: std::time::Duration::new(0, 0) };

    //let c = conf::Conf::new();    //default.

    let cb = ContextBuilder::new("game-template", "ggez")
    .window_setup(conf::WindowSetup::default().title("Visual Keyboard"))  //set title. 
    //.window_setup(conf::WindowSetup::default().resizable(true));  //...Not sure why not working.
    //.window_mode(conf::WindowMode::default().dimensions(1000.0, 1000.0)); //manually set window size. 
    .window_mode(conf::WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Desktop)); //Desktop enum work True doesn't....?

    let (ctx, event_loop) = &mut cb.build().unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
