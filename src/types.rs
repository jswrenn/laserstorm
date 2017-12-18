use nalgebra::{Point2, Vector1, Vector2, Matrix1, UnitComplex, Translation2, Isometry2};

/// The point type.
pub type Point<N> = Point2<N>;

/// The vector type.
pub type Vector<N> = Vector2<N>;

/// The orientation type.
pub type Orientation<N> = Vector1<N>;

/// The transformation matrix type.
pub type Isometry<N> = Isometry2<N>;

/// The rotation matrix type.
pub type Rotation<N> = UnitComplex<N>;

/// The translation type.
pub type Translation<N> = Translation2<N>;

/// The inertia tensor type.
pub type AngularInertia<N> = Matrix1<N>;