pub mod save;
pub mod set;

// pub enum EventEnum {
//     Start(Event),
//     Stop(Event),
//     Step(Event),
//     Reject(Event),
//     Call(Event),
//     Detect(Detection, Event),
// }

pub struct Event<Save, Set, Call> {
    save: Save,
    set: Set,
    call: Call,
}

impl Event<(), (), ()> {
    pub fn new() -> Self {
        Self {
            save: (),
            set: (),
            call: (),
        }
    }
}

impl<Save, Set, Call> Event<Save, Set, Call> {
    pub fn save<NewSave>(self, save: NewSave) -> Event<NewSave, Set, Call> {
        Event::<NewSave, Set, Call> {
            save,
            set: self.set,
            call: self.call,
        }
    }

    pub fn set<NewSet>(self, set: NewSet) -> Event<Save, NewSet, Call> {
        Event::<Save, NewSet, Call> {
            save: self.save,
            set,
            call: self.call,
        }
    }

    pub fn call<NewCall>(self, call: NewCall) -> Event<Save, Set, NewCall> {
        Event::<Save, Set, NewCall> {
            save: self.save,
            set: self.set,
            call,
        }
    }
}

pub struct Detection {
    detection: (),
    location: (),
}

#[cfg(test)]
mod event_tests {

    use super::*;

    #[test]
    fn test() {
        let mut x = 5.;
        let event = Event::new();
        let event = Event::new().save(|| 42.);
        let event = Event::new().set(|| println!("Hi"));
        let event = Event::new().save(|| 42.).set(|| x = 69.);
        let event = Event {
            save: (),
            set: (),
            call: (),
        };

        // let event = Event::new().save(|| 42.).set(|| x = 69.);
    }
}
