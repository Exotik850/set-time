//! set-time
//!
//! A simple cross-platform utility library to set the system time for a system
//!
//! # Example
//! ```ignore
//! use set_time::{set_time, Utc, DateTime};
//!
//! fn main() {
//!   // Set the system time to January 1, 2020
//!   let new_time_str = "2020-01-01 00:00:00";
//!   let new_time = DateTime::parse_from_str(new_time_str, "%Y-%m-%d %H:%M:%S")
//!       .expect("Failed to parse time");
//!   set_time(new_time).expect("Failed to set system time");
//! }
//! ```
#![allow(dead_code)]

pub use chrono::{DateTime, Datelike, Timelike, Utc, offset::TimeZone};

#[derive(Debug)]
pub enum SetTimeError {
    PermissionDenied,
    InvalidTime,
}

impl std::fmt::Display for SetTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetTimeError::PermissionDenied => write!(f, "Permission denied, are you running as administrator/root?"),
            SetTimeError::InvalidTime => write!(f, "Invalid time format"),
        }
    }
}
impl std::error::Error for SetTimeError {}

#[cfg(target_os = "windows")]
fn set_time_windows<D: Datelike + Timelike>(time: D) -> Result<(), SetTimeError> {
    use windows_sys::Win32::{Foundation::SYSTEMTIME, System::SystemInformation::SetSystemTime};
    let time: SYSTEMTIME = SYSTEMTIME {
        wYear: time.year() as u16,
        wMonth: time.month() as u16,
        wDayOfWeek: time.weekday().num_days_from_sunday() as u16,
        wDay: time.day() as u16,
        wHour: time.hour() as u16,
        wMinute: time.minute() as u16,
        wSecond: time.second() as u16,
        wMilliseconds: (time.nanosecond() / 1_000_000) as u16,
    };
    unsafe {
        let result = SetSystemTime(&time);
        if result == 0 {
            return Err(SetTimeError::PermissionDenied);
        }
    }
    Ok(())
}

#[cfg(unix)]
fn set_time_unix<D: Datelike + Timelike>(time: D) -> Result<(), SetTimeError> {
    let naive_date = chrono::NaiveDate::from_ymd_opt(time.year(), time.month(), time.day())
        .ok_or(SetTimeError::InvalidTime)?;
    let naive_time = chrono::NaiveTime::from_hms_nano_opt(time.hour(), time.minute(), time.second(), time.nanosecond())
        .ok_or(SetTimeError::InvalidTime)?;
    let naive_dt = chrono::NaiveDateTime::new(naive_date, naive_time);
    
    // Convert to Unix timestamp
    let timestamp = naive_dt.and_utc().timestamp();
    
    let tv = libc::timeval {
        tv_sec: timestamp as libc::time_t,
        tv_usec: (time.nanosecond() / 1000) as libc::suseconds_t,
    };
    
    unsafe {
        let result = libc::settimeofday(&tv, std::ptr::null());
        if result != 0 {
            return Err(SetTimeError::PermissionDenied);
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn set_time_macos<D: Datelike + Timelike>(time: D) -> Result<(), SetTimeError> {
    set_time_unix(time)
}

#[cfg(target_os = "linux")]
fn set_time_linux<D: Datelike + Timelike>(time: D) -> Result<(), SetTimeError> {
    set_time_unix(time)
}

/// Sets the system time to the specified time.
/// 
/// # Errors
/// 
/// - `SetTimeError::PermissionDenied` if the operation fails due to insufficient permissions.
/// - `SetTimeError::InvalidTime` if the provided time is invalid.
pub fn set_time<D: Datelike + Timelike>(time: D) -> Result<(), SetTimeError> {
    #[cfg(target_os = "windows")]
    return set_time_windows(time);
    
    #[cfg(target_os = "linux")]
    return set_time_linux(time);
    
    #[cfg(target_os = "macos")]
    return set_time_macos(time);
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    compile_error!("Unsupported platform");
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_set_time() {
        let original_time = chrono::Utc::now();
        let new_time = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc();
        let result = set_time(new_time);
        assert!(result.is_ok());
        let now_after = chrono::Utc::now();
        assert_eq!(now_after.year(), 2020);
        assert_eq!(now_after.month(), 1);
        assert_eq!(now_after.day(), 1);
        set_time(original_time).expect("Failed to restore original time");
    }
}
