use std::num::{Primitive, cast};

use euler::Euler;
use vector3d::Vector3D;
mod constants;

pub struct Quaternion<T>
{
    x: T,
    y: T,
    z: T,
    w: T
}

impl<T: Primitive> Quaternion<T>
{
    pub fn new(x: T, y: T, z: T, w: T) -> Quaternion<T>
    {
        Quaternion{ x: x, y: y, z: z, w: w}
    }
    
    pub fn new_empty() -> Quaternion<T>
    {
        Quaternion::new( cast(0).unwrap(),
                         cast(0).unwrap(),
                         cast(0).unwrap(),
                         cast(0).unwrap() )
    }
    
    pub fn from_euler(euler: &Euler<T>) -> Quaternion<T>
    {
        let mut result = Quaternion::new_empty();
        let pitch = euler.pitch.to_f32().unwrap() * constants::DEGREES_TO_RADIANS;
        let yaw = euler.yaw.to_f32().unwrap() * constants::DEGREES_TO_RADIANS;
        let roll = euler.roll.to_f32().unwrap() * constants::DEGREES_TO_RADIANS;
        
        let spitch = (pitch * 0.5).sin();
        let syaw = (yaw * 0.5).sin();
        let sroll = (roll * 0.5).sin();
        let cpitch = (pitch * 0.5).cos();
        let cyaw = (yaw * 0.5).cos();
        let croll = (roll * 0.5).cos();
        
        let cyawroll = cyaw * croll;
        let syawroll = syaw * sroll;
        
        result.x = cast( (spitch * cyawroll) - (cpitch * syawroll) ).unwrap();
        result.y = cast( (cpitch * syaw * croll) + (spitch * cyaw * sroll) ).unwrap();
        result.z = cast( (cpitch * cyaw * sroll) + (spitch * syaw * croll) ).unwrap();
        result.w = cast( (cpitch * cyawroll) + (spitch * syawroll) ).unwrap();
        
        return result;
    }
    
    pub fn from_axis_angle(axis: &Vector3D<T>, angle: T) -> Quaternion<T>
    {
        let mut result = Quaternion::new_empty();
        let sang = angle.to_f32().unwrap().sin();
        let cang = angle.to_f32().unwrap().cos();
        let norm = axis.normalize();
        
        result.x = norm.x * cast(sang).unwrap();
        result.y = norm.y * cast(sang).unwrap();
        result.z = norm.z * cast(sang).unwrap();
        result.w = cast(cang).unwrap();
        
        return result;
    }
    
    pub fn to_vector(&self) -> Vector3D<T>
    {
        Vector3D::new(self.x.clone(),
                      self.y.clone(),
                      self.z.clone())
    }
    
    /*
    pub fn to_matrix(&self) -> Matrix3x3<T>
    {
        
    }
    */
    
    pub fn scale(&self, scalar: T) -> Quaternion<T>
    {
        Quaternion::new( self.x * scalar,
                         self.y * scalar,
                         self.z * scalar,
                         self.w * scalar )
    }
    
    pub fn normal(&self) -> T
    {
        ( (self.x * self.x) + 
          (self.y * self.y) + 
          (self.z * self.z) + 
          (self.w * self.w) )
    }
    
    pub fn magnitude(&self) -> T
    {
        let norm = self.normal().to_f32().unwrap();
        let ret = cast(norm.sqrt()).unwrap();
        
        return ret;
    }
    
    pub fn conjugate(&self) -> Quaternion<T>
    {
        Quaternion::new(-self.x, -self.y, -self.z, self.w.clone())
    }
    
    pub fn normalize(&self) -> Quaternion<T>
    {
        let mut x: f32;
        let mut y: f32;
        let mut z: f32;
        let mut w: f32;
        let mut result = Quaternion::new(self.x.clone(),
                                         self.y.clone(),
                                         self.z.clone(),
                                         self.w.clone());
                                       
        let mag = result.magnitude();
        
        if ( !(mag.is_zero()) )
        {
            x = result.x.to_f32().unwrap() / mag.to_f32().unwrap();
            y = result.y.to_f32().unwrap() / mag.to_f32().unwrap();
            z = result.z.to_f32().unwrap() / mag.to_f32().unwrap();
            w = result.w.to_f32().unwrap() / mag.to_f32().unwrap();
            
            result.x = cast(x).unwrap();
            result.y = cast(y).unwrap();
            result.z = cast(z).unwrap();
            result.w = cast(w).unwrap();
        }
        
        return result;
    }
    
    //inverse
    //slerp
}