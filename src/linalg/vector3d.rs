use std::num::{Primitive, cast};

use point3d::Point3D;
mod constants;

pub struct Vector3D<T>
{
    x: T,
    y: T,
    z: T
}

impl<T: Primitive> Vector3D<T>
{
    pub fn new(x: T, y: T, z: T) -> Vector3D<T>
    {
        Vector3D{x: x, y: y, z: z}
    }

    pub fn new_empty() -> Vector3D<T>
    {
        Vector3D::new( cast(0).unwrap(),
                       cast(0).unwrap(),
                       cast(0).unwrap() )
    }
    
    pub fn from_points(a: &Point3D<T>, b: &Point3D<T>) -> Vector3D<T>
    {
        Vector3D::new( (b.x - a.x),
                       (b.y - a.y),
                       (b.z - a.z) )
    }

    pub fn dot_product(&self, other: &Vector3D<T>) -> T
    {
        ( (self.x * other.x) +
          (self.y * other.y) +
          (self.z * other.z) )
    }
    
    pub fn cross_product(&self, other: &Vector3D<T>) -> Vector3D<T>
    {
        Vector3D::new( (self.y * other.z) - (self.z * other.y),
                       (self.z * other.x) - (self.x * other.z),
                       (self.x * other.y) - (self.y * other.x) )
    }
    
    pub fn normalize(&self) -> Vector3D<T>
    {
        let mut x: f32;
        let mut y: f32;
        let mut z: f32;
        let mut result = Vector3D::new(self.x.clone(),
                                       self.y.clone(),
                                       self.z.clone());
                                       
        let dist = result.distance();
        
        if ( !(dist.is_zero()) )
        {
            x = result.x.to_f32().unwrap() / dist.to_f32().unwrap();
            y = result.y.to_f32().unwrap() / dist.to_f32().unwrap();
            z = result.z.to_f32().unwrap() / dist.to_f32().unwrap();
            
            result.x = cast(x).unwrap();
            result.y = cast(y).unwrap();
            result.z = cast(z).unwrap();
        }
        
        return result;
    }
    
    pub fn length(&self) -> T
    {
        self.distance()
    }
    
    pub fn distance(&self) -> T
    {
        let origin = Point3D::new_empty();
        let endpoint = Point3D::new(self.x.clone(),
                                    self.y.clone(),
                                    self.z.clone());
        
        origin.distance(&endpoint)
    }
}
