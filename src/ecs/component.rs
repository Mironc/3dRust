use std::sync::Mutex;

use crate::ecs::worldinfo::WorldInfo;
pub trait Component
{
    fn load_content(&mut self)
    {

    }
    fn update(&mut self,info:&WorldInfo) where Self:Sized
    {

    }
}