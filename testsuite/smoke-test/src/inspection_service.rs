// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use crate::smoke_test_environment::new_local_swarm_with_libra2;
use libra2_forge::Swarm;

#[tokio::test]
async fn test_inspection_service_connection() {
    let swarm = new_local_swarm_with_libra2(1).await;
    let info = swarm.libra2_public_info();
    // Ping the inspection service index page and verify we get a successful response
    let resp = reqwest::get(info.inspection_service_url().to_owned())
        .await
        .unwrap();
    assert_eq!(reqwest::StatusCode::OK, resp.status());
}
