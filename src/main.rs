use ggez::*;    //tells rust we want to use ggez.

struct State {
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from((0.500, 0.500, 0.500, 0.0)));    //set the background color. In this case grey. 
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(500.0, 250.0, 200.0, 100.0),
            graphics::WHITE,
            )?;
        graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }

}
fn main(){
    let state = &mut State { };

    //let c = conf::Conf::new();    //default.

    let cb = ContextBuilder::new("game-template", "ggez")
    .window_setup(conf::WindowSetup::default().title("Visual Keyboard"))  //set title. 
    //.window_setup(conf::WindowSetup::default().resizable(true));  //...Not sure why not working.
    //.window_mode(conf::WindowMode::default().dimensions(1000.0, 1000.0)); //manually set window size. 
    .window_mode(conf::WindowMode::default().borderless(false)) //Desktop enum work True doesn't....?
    .window_mode(conf::WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Desktop)); //Desktop enum work True doesn't....?


    let (ctx, event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
