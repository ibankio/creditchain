// Copyright © CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use libra2_enum_conversion_derive::EnumConversion;

#[derive(EnumConversion)]
enum Messages {
    Test(String, String)
}

fn main() {

}
