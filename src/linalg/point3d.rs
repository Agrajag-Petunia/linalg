use std::num::{Primitive, cast};
use vector3d::Vector3D;
use euler::Euler;
mod constants;

pub struct Point3D<T>
{
    x: T,
    y: T,
    z: T
}

impl<T: Primitive> Point3D<T>
{
    pub fn new(x: T, y: T, z: T) -> Point3D<T>
    {
        Point3D{x: x, y: y, z: z}
    }

    pub fn new_empty() -> Point3D<T>
    {
        Point3D::new( cast(0).unwrap(),
                      cast(0).unwrap(),
                      cast(0).unwrap() )
    }
    
    pub fn clone(&self) -> Point3D<T>
    {
        Point3D::new( self.x.clone(),
                      self.y.clone(),
                      self.z.clone() )
    }
    
    pub fn to_vector(&self, other: &Point3D<T>) -> Vector3D<T>
    {
        Vector3D::from_points(self, other)
    }
    
    pub fn to_euler_rad(&self, other: &Point3D<T>) -> Euler<T>
    {
        let len = self.distance(other);
        let vec = Vector3D::from_points(self, other);
        let mut result = Euler::new_empty();
        
        let yaw = ((-vec.z).to_f32().unwrap()).atan2(&(vec.x.to_f32().unwrap()));
        let roll = (vec.y.to_f32().unwrap() / len.to_f32().unwrap()).asin();
        
        result.yaw = cast(yaw).unwrap();
        result.roll = cast(roll).unwrap();
        
        return result;
    }
    
    pub fn to_euler_deg(&self, other: &Point3D<T>) -> Euler<T>
    {
        let mut result = self.to_euler_rad(other);
        
        result.yaw = cast(result.yaw.to_f32().unwrap() * 
                          constants::RADIANS_TO_DEGREES).unwrap();
                          
        result.roll = cast(result.roll.to_f32().unwrap() * 
                          constants::RADIANS_TO_DEGREES).unwrap();
                          
        return result;
    }
    
    pub fn midpoint(&self, other: &Point3D<T>) -> Point3D<T>
    {
        let mut result = Point3D::new_empty();

        result.x = (self.x + other.x) / cast(2).unwrap();
        result.y = (self.y + other.y) / cast(2).unwrap();
        result.z = (self.z + other.z) / cast(2).unwrap();

        return result;
    }

    pub fn distance(&self, other: &Point3D<T>) -> T
    {
        let x_dist = (other.x - self.x).to_f32().unwrap();
        let y_dist = (other.y - self.y).to_f32().unwrap();
        let z_dist = (other.z - self.z).to_f32().unwrap();
        
        let result = ((x_dist * x_dist) + 
                      (y_dist * y_dist) +
                      (z_dist * z_dist)).sqrt();
        
        return cast(result).unwrap();
    }
}
