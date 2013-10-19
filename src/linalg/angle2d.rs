pub use self::anglef::Angle2Df;
pub use self::anglei::Angle2Di;
pub use self::anglef64::Angle2Df64;
pub use self::anglei64::Angle2Di64;

macro_rules! declare (($Angle2D:ident, $Mod: ident, $Point2D: ident, $Type:ty) => (

pub mod $Mod
{
    use constants;
    use point2d::*;
    
    type $Angle2D = $Type;
    
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

declare!(Angle2Di, anglei, Point2Di, i32)
declare!(Angle2Df, anglef, Point2Df, f32)
declare!(Angle2Di64, anglei64, Point2Di64, i64)
declare!(Angle2Df64, anglef64, Point2Df64, f64)


