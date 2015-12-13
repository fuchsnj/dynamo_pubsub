extern crate aws_dynamodb;
extern crate pubsub;

use aws_dynamodb::{DynamoDb, Table};

#[test]
fn it_works() {
}

struct Subscription{
	sub: pubsub::Subscription
}
impl Subscription{
	pub fn cancel(self){/* sub is dropped */}
	
	pub fn notify_others(&self, msg: &str){
		self.sub.notify_others(msg)
	}
}

struct SubActivator{
	activator: pubsub::SubActivator
}
impl SubActivator{
	pub fn activate<F>(self, func: F) -> Subscription
	where F: FnMut(String) + 'static + Send{
		Subscription{
			sub: self.activate(func)
		}
	}
}


struct PubSub{
	table: Table,
	pubsub: pubsub::PubSub
}
impl PubSub{
	pub fn new(table: Table, domain: String, num_threads: usize) -> PubSub{
		PubSub{
			table: table,
			pubsub: pubsub::PubSub::new(num_threads)
		}
	}
	pub fn subscribe<F>(&self, channel: &str, func: F) -> Subscription
	where F: FnMut(String) + 'static + Send{
		Subscription{
			sub: self.pubsub.subscribe(channel, func)
		}
	}
	pub fn notify(&self, channel: &str, msg: &str){
		self.pubsub.notify(channel, msg)
	}
	pub fn lazy_subscribe(&self, channel: &str) -> SubActivator{
		SubActivator{
			activator: self.pubsub.lazy_subscribe(channel)
		}
	}
}