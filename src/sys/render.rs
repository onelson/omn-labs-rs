use specs;
use world as w;
use rand;
use sys;


#[derive(Clone)]
pub struct System<'a> {
    pub scene: &'a sys::Scene
}

impl<'a> System<'a> {
    pub fn new(scene: &'a sys::Scene) -> System<'a> { System { scene: scene } }
}

impl<'a> specs::System<super::Delta> for System<'a>
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