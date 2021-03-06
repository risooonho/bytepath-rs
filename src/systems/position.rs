use nphysics2d::object::{BodyHandle, RigidBody};
use specs::*;

use crate::components::*;
use crate::resources::*;

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
  type SystemData = (
    Write<'a, PhysicsSim>,
    ReadStorage<'a, RigidBodyComponent>,
    WriteStorage<'a, PositionComponent>,
  );

  fn run(&mut self, (mut physics, rb, mut position): Self::SystemData) {
    for (rb, position) in (&rb, &mut position).join() {
      if let Some(body) = physics.world.rigid_body_mut(rb.handle) {
        let rb_position = body.position().translation;
        position.x = rb_position.x;
        position.y = rb_position.y;
        position.angle = body.position().rotation.angle();
      }
    }
  }
}
