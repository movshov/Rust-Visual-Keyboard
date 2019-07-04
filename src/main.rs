use ggez::*;    //tells rust we want to use ggez.
use ggez::graphics::Mesh;
//pub const GREY: graphics::Color = ggez::graphics::Color{r: 0.500, b: 0.500, g: 0.500, a: 1.0,};

/* State is all of the data and information required to represent our game's current state. These
 * could be player positions, scores, cards in your hand, etc.. What is included in your state is
 * very dependent on the game you are making.
 */
struct State {  
    /* will need to have an array of all the keys.
     * There are three modes for black keys.(Right, Middle, Left). 
     * Right = x + 20 of white key's x position. 
     * Middle = x + 17 of white key's x position.
     * Left = x + 14 of white key's x position. 
     */

shapes: Vec<Shapes>,
}

enum Shapes{
    White(graphics::Rect),  //white piano key. 
    Black(graphics::Rect),  //black piano key.
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from((0.500, 0.500, 0.500, 0.0)));    //set the background color. In this case grey. 
        for shape in &self.shapes {
            // Make the shape...
            let mesh = match shape {    //match whether we got a black or white key regardless of position.
                &Shapes::White(rect) => {
                    Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?
                }
                &Shapes::Black(rect) => {
                    Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::BLACK)?
                }
            };

            // ...and then draw it.
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())

 /***********************************************************************************************/
            /*
        graphics::clear(ctx, graphics::Color::from((0.500, 0.500, 0.500, 0.0)));    //set the background color. In this case grey. 
        //graphics::BLACK;    //set the background color. In this case Black.
        //graphics::set_color(ctx, graphics::Color::from((1.0, 1.0, 1.0, 1.0)));
        let _rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            /*x will need to get incremented. 
             *y will need to remain constant.
             */
            graphics::Rect::new(0.0, 360.0, 25.0, 185.0),    //(x,y,w,h)
            graphics::WHITE,
            )?;
        let _rect2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            /*x will need to get incremented. 
             *y will need to remain constant.
             */
            graphics::Rect::new(14.0, 360.0, 15.0, 100.0),    //(x,y,w,h)
            graphics::BLACK,
            )?; 
        graphics::draw(ctx, &_rect, graphics::DrawParam::default())?;
        graphics::draw(ctx, &_rect2, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
            */
    }

}
fn main(){  //only get called once.
    let mut shapes = Vec::new();
    /* Need to loop through all keys and input their (x,y,w,h) coordinates. 
     * Will call a separate function to do this here instead of doing it all in main.
     */
    shapes.push(Shapes::White(graphics::Rect::new(0.0, 360.0, 25.0, 185.0)));    //(x,y,w,h)
    shapes.push(Shapes::Black(graphics::Rect::new(14.0, 360.0, 15.0, 100.0)));    //(x,y,w,h)

    //let c = conf::Conf::new();    //default.

    let state = &mut State { shapes: shapes };
    let cb = ContextBuilder::new("game-template", "ggez")
    .window_setup(conf::WindowSetup::default().title("Visual Keyboard"))  //set title. 
    //.window_setup(conf::WindowSetup::default().resizable(true));  //...Not sure why not working.
    //.window_mode(conf::WindowMode::default().dimensions(1000.0, 1000.0)); //manually set window size. 
    .window_mode(conf::WindowMode::default().borderless(false)) //Desktop enum work True doesn't....?
    .window_mode(conf::WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Desktop)); 

    let (ctx, event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
