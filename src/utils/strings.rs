//    A Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow.
//    Copyright (C) 2023  Christina Sørensen <christina@cafkafk.com>
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see https://www.gnu.org/gpl-3.0.html.
//
//! Module for chunk of text
//!
//! Ideally, at a VERY long term scale, this should be a nice pattern for
//! possible translations.

/// Contains the notice for interactive programs from the GPLv3's "How to Apply
/// These Terms to Your New Programs"
pub const INTERACTIVE_NOTICE: &str = "\
gg  Copyright (C) 2023  Christina Sørensen <christina@cafkafk.com>
This program comes with ABSOLUTELY NO WARRANTY; for details type `gg --warranty'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `gg --license' for details.
";

/// Contains the license part of the long notice for interactive programs from
/// the GPLv3's "How to Apply These Terms to Your New Programs"
pub const INTERACTIVE_LICENSE: &str = "\
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
";

/// Contains the warranty part of the long notice for interactive programs from
/// the GPLv3's "How to Apply These Terms to Your New Programs"
pub const INTERACTIVE_WARRANTY: &str = "\
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
";

/// Contains the message for quick commit subcommit
pub const QUICK_COMMIT: &'static str = "git: quick commit";
