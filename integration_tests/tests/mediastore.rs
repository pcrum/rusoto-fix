#![cfg(feature = "mediastore")]

extern crate rusoto_core;
extern crate rusoto_mediastore;

use rusoto_core::Region;
use rusoto_mediastore::{ListContainersInput, MediaStore, MediaStoreClient};

#[test]
fn should_list_containers() {
    let client = MediaStoreClient::new(Region::UsEast1);
    let request = ListContainersInput::default();

    println!("{:?}", client.list_containers(request).sync().unwrap());
}
