// Copyright 2024 Charlotte Ausel

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alga::general::{AbstractMagma, Additive, Identity, Multiplicative, TwoSidedInverse};
use alga_derive::Alga;

#[derive(Alga, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[alga_traits(Field(Additive, Multiplicative))]
pub struct GF2 {
    b: bool,
}

impl Identity<Additive> for GF2 {
    fn identity() -> Self {
        GF2 { b: false }
    }
}

impl Identity<Multiplicative> for GF2 {
    fn identity() -> Self {
        GF2 { b: true }
    }
}

impl AbstractMagma<Additive> for GF2 {
    fn operate(&self, right: &Self) -> Self {
        GF2 { b: self.b ^ right.b }
    }
}

impl AbstractMagma<Multiplicative> for GF2 {
    fn operate(&self, right: &Self) -> Self {
        GF2 { b: self.b & right.b }
    }
}

impl TwoSidedInverse<Additive> for GF2 {
    fn two_sided_inverse(&self) -> Self {
        *self
    }
}

impl TwoSidedInverse<Multiplicative> for GF2 {
    fn two_sided_inverse(&self) -> Self {
        GF2 { b: !self.b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let f: GF2 = <GF2 as Identity<Additive>>::identity();
        let t: GF2 = <GF2 as Identity<Multiplicative>>::identity();

        assert_eq!(t, <GF2 as AbstractMagma<Additive>>::operate(&f, &t));
    }
}

