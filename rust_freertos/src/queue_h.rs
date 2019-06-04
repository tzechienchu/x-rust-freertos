use crate::port::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum QueueError {
    QueueSendTimeout,
    QueueReceiveTimeout,
    MutexTimeout,
    QueueFull,
    QueueEmpty
}

pub const queueSEND_TO_BACK: BaseType = 0 ;
pub const queueSEND_TO_FRONT:BaseType = 1;
pub const queueOVERWRITE:BaseType = 2;

#[derive(PartialEq)]
pub enum QueueType {
    Base,
    Set,
    Mutex,
    CountingSemaphtore,
    BinarySemaphre,
    RecursiveMutex
}
impl Default for QueueType{
   fn default() -> Self {QueueType::Base}
 }

