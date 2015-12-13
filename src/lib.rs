extern crate aws_dynamodb;

use aws_dynamodb::{DynamoDb, Table};

#[test]
fn it_works() {
}

struct PubSub{
	table: Table
}
impl PubSub{
	fn new(table: Table) -> PubSub{
		PubSub{
			table: table
		}
	}
}