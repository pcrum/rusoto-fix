#![cfg(feature = "dms")]

extern crate rusoto_core;
extern crate rusoto_dms;

use rusoto_core::Region;
use rusoto_dms::{
    DatabaseMigrationService, DatabaseMigrationServiceClient, DescribeEndpointsMessage,
};

#[test]
fn should_describe_tags() {
    let client = DatabaseMigrationServiceClient::new(Region::UsEast1);
    let request = DescribeEndpointsMessage::default();

    let result = client.describe_endpoints(request).sync().unwrap();
    println!("{:#?}", result);
}
