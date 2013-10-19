use std::num::{Primitive};

use point3d::Point3D;
use vector3d::Vector3D;

pub struct Line3D<T>
{
    a: Point3D<T>,
    b: Point3D<T>
}

impl<T: Primitive> Line3D<T>
{
    pub fn new(ax: T, ay: T, az: T,
               bx: T, by: T, bz: T) -> Line3D<T>
    {
        let a = Point3D::new(ax, ay, az);
        let b = Point3D::new(bx, by, bz);
        
        Line3D::from_points(&a, &b)
    }
    
    pub fn from_points(a: &Point3D<T>, b: &Point3D<T>) -> Line3D<T>
    {
        Line3D{ a: a.clone(), b: b.clone() }
    }
    
    pub fn to_vector(&self) -> Vector3D<T>
    {
        Vector3D::from_points(&(self.a).clone(), &(self.b).clone())
    }
    
    pub fn midpoint(&self) -> Point3D<T>
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
