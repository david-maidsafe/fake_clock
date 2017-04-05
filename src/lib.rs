// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};
use std::time::Duration;

/// Struct representing a fake instant
#[derive(Clone, Copy)]
pub struct FakeClock {
    time_created: u64,
}

thread_local!{
    static LOCAL_TIME: RefCell<u64> = RefCell::new(0);
}

impl FakeClock {
    /// Sets the thread-local fake time to the given value
    pub fn set_time(time: u64) {
        LOCAL_TIME.with(|t| { *t.borrow_mut() = time; });
    }

    /// Advances the thread-local fake time by the given amount of milliseconds
    pub fn advance_time(millis: u64) {
        LOCAL_TIME.with(|t| { *t.borrow_mut() += millis; });
    }

    /// Returns the current thread-local fake time
    pub fn time() -> u64 {
        LOCAL_TIME.with(|t| *t.borrow())
    }

    /// Returns a `FakeClock` instance representing the current instant.
    pub fn now() -> Self {
        let time = Self::time();
        FakeClock { time_created: time }
    }

    /// Returns the duration that passed between `self` and `earlier`.
    pub fn duration_since(&self, earlier: &Self) -> Duration {
        Duration::from_millis(self.time_created - earlier.time_created)
    }

    /// Returns how much fake time has elapsed since the creation of `self`.
    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(Self::time() - self.time_created)
    }
}

impl PartialEq for FakeClock {
    fn eq(&self, other: &FakeClock) -> bool {
        self.time_created == other.time_created
    }
}

impl Eq for FakeClock {}

impl PartialOrd for FakeClock {
    fn partial_cmp(&self, other: &FakeClock) -> Option<Ordering> {
        self.time_created.partial_cmp(&other.time_created)
    }
}

impl Ord for FakeClock {
    fn cmp(&self, other: &FakeClock) -> Ordering {
        self.time_created.cmp(&other.time_created)
    }
}

impl fmt::Debug for FakeClock {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
               "FakeClock {{ time_created: {} }}",
               self.time_created)
    }
}

impl Add<Duration> for FakeClock {
    type Output = FakeClock;
    fn add(mut self, other: Duration) -> FakeClock {
        self.time_created += other.as_secs() * 1000 + other.subsec_nanos() as u64 / 1000000;
        self
    }
}

impl Sub<Duration> for FakeClock {
    type Output = FakeClock;
    fn sub(mut self, other: Duration) -> FakeClock {
        self.time_created -= other.as_secs() * 1000 + other.subsec_nanos() as u64 / 1000000;
        self
    }
}

impl Sub<FakeClock> for FakeClock {
    type Output = Duration;
    fn sub(self, other: FakeClock) -> Duration {
        Duration::from_millis(self.time_created - other.time_created)
    }
}