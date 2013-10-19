//This is kind of an ugly module but it provides an easy way to decouple
//the internal classes while providing easy conversion of classes externally

pub use self::factoriesf;
pub use self::factoriesi;
pub use self::factoriesf64;
pub use self::factoriesi64;

macro_rules! declare (($Mod: ident,
                       $Point3D: ident,
                       $Vector3D: ident,
                       $Type:ty) => (

pub mod $Mod
{
    use constants;
    use point2d::*;
    
    pub fn create_vector_from_points(a: &$Point3D b: &$Vector3D) -> $Vector3D
    {
        $Vector3D::new( (b.x - a.x),
                        (b.y - a.y),
                        (b.z - a.z) )
    }
    
    impl $Angle2D
    {
        pub fn new(angle: $Type) -> $Angle2D
        {
            return (angle as $Angle2D);
        }
        
        pub fn from_points(a: $Point2D, b: $Point2D) -> $Angle2D
        {
            let x_dist = (b.x - a.x) as float;
            let y_dist = (b.y - a.y) as float;

            (y_dist.atan2(&x_dist) * constants::RADIANS_TO_DEGREES) as $Angle2D
        }

        pub fn to_radians(&self) -> $Angle2D
        {
            ( (self as float) * constants::DEGREES_TO_RADIANS ) as $Angle2D
        }
        
        pub fn to_degrees(&self) -> $Angle2D
        {
            ( (self as float) * constants::RADIANS_TO_DEGREES ) as $Angle2D
        }
    }
} 

);)

declare!(factoriesi, Point3Di, Vector3Di, i32)
declare!(factoriesf, Point3Df, Vector3Df, f32)
declare!(factoriesi64, Point3Di64, Vector3Di64, i64)
declare!(factoriesf64, Point3Df64, Vector3Df64, f64)


