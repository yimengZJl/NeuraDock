mod aggregate;
mod repository;
mod sender;
mod value_objects;

pub use aggregate::NotificationChannel;
pub use repository::NotificationChannelRepository;
pub use sender::{NotificationMessage, NotificationSender};
pub use value_objects::{ChannelConfig, ChannelType, NotificationChannelId};
