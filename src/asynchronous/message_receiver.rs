#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait MessageReceiver {
    fn receive(&self);
}