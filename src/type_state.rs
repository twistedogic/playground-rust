#![allow(dead_code)]
use std::fmt;
use std::marker::PhantomData;

trait State {}

struct Locked;
impl State for Locked {}

struct Unlocked;
impl State for Unlocked {}

type Passcode = u16;

#[derive(Debug, PartialEq)]
enum SafeError {
    WrongPasscode(Passcode),
}

struct Safe<T: State = Locked> {
    state: PhantomData<T>,
    secret: String,
    passcode: u16,
}

impl Safe<Locked> {
    fn unlock(&self, passcode: u16) -> Result<Safe<Unlocked>, SafeError> {
        if self.passcode == passcode {
            Ok(Safe {
                state: PhantomData::<Unlocked>,
                secret: self.secret.clone(),
                passcode: self.passcode,
            })
        } else {
            Err(SafeError::WrongPasscode(passcode))
        }
    }
}

impl fmt::Display for Safe<Locked> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The safe is locked.")
    }
}

impl Safe<Unlocked> {
    fn lock(self, passcode: u16) -> Safe<Locked> {
        Safe {
            state: std::marker::PhantomData::<Locked>,
            secret: self.secret,
            passcode,
        }
    }
}

impl fmt::Display for Safe<Unlocked> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.secret)
    }
}

impl Safe {
    fn new(secret: String, passcode: u16) -> Self {
        Safe {
            state: Default::default(),
            secret,
            passcode,
        }
    }
}

#[test]
fn test_safe() {
    let secret = String::from("suppp");
    let safe = Safe::new(secret.clone(), 123);
    assert_eq!(Some(SafeError::WrongPasscode(111)), safe.unlock(111).err());
    assert_eq!(None, safe.unlock(123).err());
    if let Ok(opened_safe) = safe.unlock(123) {
        assert_eq!(secret, opened_safe.to_string());
        assert_eq!(
            Some(SafeError::WrongPasscode(123)),
            opened_safe.lock(111).unlock(123).err()
        );
    }
    assert_ne!(secret, safe.to_string());
}
