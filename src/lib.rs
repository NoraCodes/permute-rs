// SPDX-FileCopyrightText: 2020 Leonora Tindall <nora@nora.codes>
//
// SPDX-License-Identifier: AGPL-3.0-or-later WITH LicenseRef-AppStore

#![doc = include_str!("../README.md")]

mod arbitrary_tandem_control_iter;
mod permutations;
mod permute_iter;

pub use arbitrary_tandem_control_iter::ArbitraryTandemControlIterator;
pub use permutations::permute;
pub use permute_iter::permutations_of;
