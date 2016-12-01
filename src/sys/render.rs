use specs;
use piston_window::ImageSize;
use world as w;
use rand;
use sys;


#[derive(Clone)]
pub struct System<'a, I> where I: ImageSize + 'a {
    pub scene: &'a sys::Scene<I>
}

impl<'a, I: ImageSize + 'a> System<'a, I> {
    pub fn new(scene: &'a sys::Scene<I>) -> System<'a, I> { System { scene: scene } }
}

impl<'a, I: ImageSize + 'a> specs::System<super::Delta> for System<'a, I>
{
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use specs::Join;

        let (body, sprited) = arg.fetch(|w| {
            (w.read::<w::Body>(), w.read::<w::Sprited>())
        });

        // update entities
        for (b, s) in (&body, &sprited).iter() {
            for sprite in self.scene.child(s.id).iterMut {
                sprite.set_rotation(b.rotation);
            }
        }
    }
}
