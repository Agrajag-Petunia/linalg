use std::num::{Primitive};

use point2d::Point2D;
use vector2d::Vector2D;

pub struct Line2D<T>
{
    a: Point2D<T>,
    b: Point2D<T>
}

impl<T: Primitive> Line2D<T>
{
    pub fn new(ax: T, ay: T, bx: T, by: T) -> Line2D<T>
    {
        let a = Point2D::new(ax, ay);
        let b = Point2D::new(bx, by);
        
        Line2D::from_points(&a, &b)
    }
    
    pub fn from_points(a: &Point2D<T>, b: &Point2D<T>) -> Line2D<T>
    {
        Line2D{ a: a.clone(), b: b.clone() }
    }
    
    pub fn to_vector(&self) -> Vector2D<T>
    {
        Vector2D::from_points(&(self.a).clone(), &(self.b).clone())
    }
    
    pub fn midpoint(&self) -> Point2D<T>
    {
        self.a.midpoint( &(self.b))
    }
    
    pub fn length(&self) -> T
    {
        self.distance()
    }
    
    pub fn distance(&self) -> T
    {
        self.a.distance( &(self.b) )
    }
}
