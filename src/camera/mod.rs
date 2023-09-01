use cgmath::{perspective,Deg,Matrix4, Point3, vec1, Vector3, vec3};
pub struct Camera
{
    pub perspective:[[f32;4];4],
    pub projection:[[f32;4];4],
    pub pos:Point3<f32>,
    pub forward:Vector3<f32>,
    pub right:Vector3<f32>,
    pub up:Vector3<f32>,
    aspect:f32,
    zfar:f32,
    znear:f32,
    pub pitch:f32,
    pub yaw:f32,
    pub roll:f32,
    pub fov:f32,
}
impl Camera
{
    pub fn new(fov:f32,zfar:f32,znear:f32,aspect:f32) -> Self
    {
        Self
        {
            fov,
            zfar,
            znear,
            pos:Point3 { x: 0.0, y:0.0, z: -2.0 },
            aspect,
            perspective:[[0.0;4];4],
            projection:[[0.0;4];4],
            pitch:0.,
            yaw:-90.,
            roll:0.,
            forward: vec3(0., 0., 0.),
            right: vec3(0.0, 0.0, 0.0),
            up: vec3(0., 0., 0.),
        }
    }
    pub fn set_aspect(&mut self,height:u32,width:u32)
    {
        self.aspect = width as f32 / height as f32;
    }
    pub fn update(&mut self)
    {
        self.build_perspective();
        self.vector_calculation();
        self.build_projection();
    }
    pub fn build_perspective(&mut self)
    {
        self.perspective = perspective(Deg{0:self.fov}, self.aspect, self.znear, self.zfar).into()
    }
    pub fn build_projection(&mut self)
    {
        self.projection = Matrix4::look_at_lh(self.pos, self.pos + self.forward, self.up).into();
    }
    fn vector_calculation(&mut self)
    {
        self.forward.y = self.pitch.to_degrees().sin();
        self.forward.x = self.yaw.to_degrees().cos() * self.pitch.to_degrees().cos();
        self.forward.z = self.roll.to_degrees().cos() * self.pitch.to_degrees().cos();
        self.forward = cgmath::InnerSpace::normalize(self.forward); 
        self.right = cgmath::InnerSpace::normalize(self.forward.cross(vec3(0., 1., 0.)));
        self.up = cgmath::InnerSpace::normalize(self.right.cross(self.forward)); 
    }
}