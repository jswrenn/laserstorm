use nalgebra;
use ncollide;
use specs;
use nalgebra::core::dimension;

pub type Precision = f32;
pub type Dimension = dimension::U2;
pub type Data      = specs::Entity;

pub type Point           = nalgebra::Point<Precision, Dimension>;
pub type Vector          = nalgebra::VectorN<Precision,Dimension>;
pub type Orientation     = nalgebra::VectorN<Precision, <Dimension as nalgebra::DimSub<dimension::U1>>::Output>;
pub type Isometry        = nalgebra::Isometry<Precision, Dimension, Rotation>;
pub type Rotation        = nalgebra::UnitComplex<Precision>;
pub type Translation     = nalgebra::Translation<Precision, Dimension>;
pub type AngularInertia  = nalgebra::Matrix1<Precision>;
pub type CollisionWorld  = ncollide::world::CollisionWorld<Point, Isometry, Data>;
pub type CollisionObject = ncollide::world::CollisionObject<Point, Isometry, Data>;
pub type Contact         = ncollide::query::Contact<Point>;
pub type ShapeHandle     = ncollide::shape::ShapeHandle<Point, Isometry>;
