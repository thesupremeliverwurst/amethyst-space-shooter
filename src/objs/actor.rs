//helper types for your actors to be stored in parallel to your actor components, and some methods all actors must have.
use std::ops;

use amethyst::{
	ecs::prelude::{
		Component,
		DenseVecStorage,
	},
};

pub enum DamageType {
	Ballistic,
	Fire,
	Gamma,
	Yeet,
}

pub trait Actor {
	fn take_damage<T: Actor>(&mut self,amount:u32, dam_type:DamageType, from_id:&T);
	fn do_collision<T: Actor>(&mut self,input_newtons:f32,other:&T);

	fn to_string (&self) -> String {
		String::from("(unimplemented to_string) Actor")
	}
}

pub struct Vector {
	x:f32,
	y:f32,
}

impl ops::AddAssign<Vector> for Vector {

	fn add_assign(&mut self, other:Vector) {
		self.x+=other.x;
		self.y+=other.y;
	}
}

impl ops::Add<Vector> for Vector {
	type Output = Vector;

	fn add(self, other: Vector) -> Vector {
		Vector {
			x:self.x+other.x,
			y:self.y+other.y,
		}
	}
}

impl ops::Mul<f32> for Vector {
	type Output = Vector;

	fn mul(self, scalar:f32) -> Vector {
		Vector {
			x:self.x*scalar,
			y:self.y*scalar,
		}
	}
}

impl<'a> ops::Mul<Vector> for f32 {
	type Output = Vector;

	fn mul(self, vec: Vector) -> Vector {
		Vector {
			x:self*vec.x,
			y:self*vec.y,
		}
	}
}

impl Vector {
	fn new(x:f32,y:f32) -> Self {
		Vector {
			x:x,
			y:y,
		}
	}
	fn set_is_pos(mut self, dt:f32, v:Vector, a:Vector) {//This is mostly a formality, use reimplementation for transform component
		self+=v*dt+0.5*a*dt*dt;
	}
	fn set_is_vel(mut self, dt:f32, a:Vector, a2:Vector) {
		self+=0.5*(a+a2)*dt;
	}
}

impl Component for Vector {
	type Storage = DenseVecStorage<Self>;
}