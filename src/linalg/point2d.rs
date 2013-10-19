use std::num::{Primitive, cast};
use vector2d::Vector2D;
mod constants;

pub struct Point2D<T>
{
    x: T,
    y: T
}

impl<T: Primitive> Point2D<T>
{
    pub fn new(x: T, y: T) -> Point2D<T>
    {
        Point2D{x: x, y: y}
    }
    
    pub fn new_empty() -> Point2D<T>
    {
        Point2D::new( cast(0).unwrap(), 
                      cast(0).unwrap() )
    }

    pub fn clone(&self) -> Point2D<T>
    {
        Point2D::new( self.x.clone(), self.y.clone() )
    }
    
    pub fn to_vector(&self, other: &Point2D<T>) -> Vector2D<T>
    {
        Vector2D::from_points(self, other)
    }
    
    pub fn to_angle_rad(&self, other: &Point2D<T>) -> T
    {
        let x_dist = (other.x - self.x).to_f32().unwrap();
        let y_dist = (other.y - self.y).to_f32().unwrap();
        
        cast(y_dist.atan2(&x_dist)).unwrap()
    }
    
    pub fn to_angle_deg(&self, other: &Point2D<T>) -> T
    {
        let ang: f32 = self.to_angle_rad(other).to_f32().unwrap();
        
        cast(ang * constants::RADIANS_TO_DEGREES).unwrap()
    }
    
    pub fn midpoint(&self, other: &Point2D<T>) -> Point2D<T>
    {
        let mut result = Point2D::new_empty();

        result.x = (self.x + other.x) / cast(2).unwrap();
        result.y = (self.y + other.y) / cast(2).unwrap();

        return result;
    }

    pub fn find_point(&self, distance: T, angle: T) -> Point2D<T>
    {
        let mut result = Point2D::new_empty();
        let opp = angle.to_f32().unwrap().sin() * distance.to_f32().unwrap();
        let adj = angle.to_f32().unwrap().cos() * distance.to_f32().unwrap();

        result.x = self.x + cast(adj).unwrap();
        result.y = self.y + cast(opp).unwrap();

        return result;
    }

    pub fn distance(&self, other: &Point2D<T>) -> T
    {
        let x_dist = (other.x - self.x).to_f32().unwrap();
        let y_dist = (other.y - self.y).to_f32().unwrap();

        let result = ((x_dist * x_dist) + (y_dist * y_dist)).sqrt();
        
        return cast(result).unwrap();
    }
}
