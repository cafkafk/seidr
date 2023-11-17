// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::sync::atomic::AtomicBool;

pub static QUIET: AtomicBool = AtomicBool::new(false);

pub static EMOJIS: AtomicBool = AtomicBool::new(false);

pub static UNLINK: AtomicBool = AtomicBool::new(false);

pub static FORCE: AtomicBool = AtomicBool::new(false);
