// Adopted from
// https://github.com/BurntSushi/ripgrep/blob/8892bf/src/messages.rs under the
// MIT License.
//
// Copyright (c) 2015 Andrew Gallant
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// This file has been sublicensed by Michael Sanders under the Apache License,
// Version 2.0, <LICENSE-APACHE or https://apache.org/licenses/LICENSE-2.0> or
// the MIT License (see above, and <LICENSE-MIT or
// https://opensource.org/licenses/MIT>), at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::sync::atomic::{AtomicBool, Ordering};

static QUIET_OUTPUT: AtomicBool = AtomicBool::new(false);
static DRY_RUN_OUTPUT: AtomicBool = AtomicBool::new(false);
static VERBOSE_OUTPUT: AtomicBool = AtomicBool::new(false);

/// Emit a successful message to stdout, unless quiet output is enabled.
#[macro_export]
macro_rules! message {
    ($($tt:tt)*) => {
        if !$crate::messages::quiet_output() {
            println!("{} {}", "Success!".underline().green(), format!($($tt)*))
        }
    }
}

/// Emit a non-fatal diagnostic message to stdout, if verbose output is enabled
/// and quiet output is disabled.
#[macro_export]
macro_rules! verbose_message {
    ($($tt:tt)*) => {
        if !$crate::messages::quiet_output() && $crate::messages::verbose_output() {
            println!("{} {}", "==>".bold().blue(), format!($($tt)*));
        }
    }
}

/// Emit a non-fatal diagnostic message to stdout, if dry run or verbose output
/// is enabled and quiet output is disabled.
#[macro_export]
macro_rules! command_message {
    ($($tt:tt)*) => {
        if !$crate::messages::dry_run_output() {
            // Distinguish between verbose and dry-run modes.
            verbose_message!($($tt)*)
        } else if !$crate::messages::quiet_output() {
            println!("{} {}", "==>".bold().yellow(), format!($($tt)*));
        }
    }
}

/// Returns true if and only if messages should not be shown.
pub fn quiet_output() -> bool {
    QUIET_OUTPUT.load(Ordering::SeqCst)
}

/// Set whether messages should be shown or not.
///
/// By default, they are shown.
pub fn set_quiet_output(yes: bool) {
    QUIET_OUTPUT.store(yes, Ordering::SeqCst)
}

/// Returns true if and only if dry_run messages should be shown.
pub fn dry_run_output() -> bool {
    DRY_RUN_OUTPUT.load(Ordering::SeqCst)
}

/// Set whether "dry_run" related messages should be shown or not.
///
/// By default, they are not shown.
///
/// Note that this is overridden if `quiet_output` is enabled. Namely, if
/// `quiet_output` is enabled, then "dry_run" messages are never shown,
/// regardless of this setting.
pub fn set_dry_run_output(yes: bool) {
    DRY_RUN_OUTPUT.store(yes, Ordering::SeqCst)
}

/// Returns true if and only if verbose messages should be shown.
pub fn verbose_output() -> bool {
    VERBOSE_OUTPUT.load(Ordering::SeqCst)
}

/// Set whether "verbose" related messages should be shown or not.
///
/// By default, they are not shown.
///
/// Note that this is overridden if `quiet_output` is enabled. Namely, if
/// `quiet_output` is enabled, then "verbose" messages are never shown,
/// regardless of this setting.
pub fn set_verbose_output(yes: bool) {
    VERBOSE_OUTPUT.store(yes, Ordering::SeqCst)
}
