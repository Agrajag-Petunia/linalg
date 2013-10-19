use std::num::{Primitive, cast};

use point2d::Point2D;
//use euler_angle::EulerAngle$Type;
mod constants;

pub struct Vector2D<T>
{
    x: T,
    y: T,
}

impl<T: Primitive> Vector2D<T>
{
    pub fn new(x: T, y: T) -> Vector2D<T>
    {
        Vector2D{x: x, y: y}
    }

    pub fn new_empty() -> Vector2D<T>
    {
        Vector2D::new( cast(0).unwrap(), cast(0).unwrap())
    }
    
    pub fn from_points(a: &Point2D<T>, b: &Point2D<T>) -> Vector2D<T>
    {
        Vector2D::new( b.x - a.x, b.y - a.y )
    }
    
    pub fn dot_product(&self, other: &Vector2D<T>) -> T
    {
        ( (self.x * other.x) + (self.y * other.y) )
    }
    
    pub fn normalize(&self) -> Vector2D<T>
    {
        let mut x: f32;
        let mut y: f32;
        let mut result = Vector2D::new(self.x.clone(),
                                       self.y.clone());
        
        let dist = result.distance();
        
        if ( !(dist.is_zero()) )
        {
            x = result.x.to_f32().unwrap() / dist.to_f32().unwrap();
            y = result.y.to_f32().unwrap() / dist.to_f32().unwrap();
            
            result.x = cast(x).unwrap();
            result.y = cast(y).unwrap();
        }
        
        return result;
    }
    
    pub fn length(&self) -> T
    {
        self.distance()
    }
    
    pub fn distance(&self) -> T
    {
        let origin = Point2D::new_empty();
        let endpoint = Point2D::new(self.x.clone(),
                                    self.y.clone());
        
        origin.distance(&endpoint)
    }
}



