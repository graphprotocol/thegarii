// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![allow(missing_docs)]

pub mod cs {
    pub mod arweave {
        pub mod codec {
            pub mod v1 {
                tonic::include_proto!("cs.arweave.codec.v1");
            }
        }
    }
}

pub mod sf {
    pub mod firehose {
        pub mod v1 {
            tonic::include_proto!("sf.firehose.v1");
        }
    }
}

pub use self::{cs::arweave::codec::v1::*, sf::firehose::v1::*};
