use bitflags::bitflags;

type KeyCode = u32;
type EventData = u32;

macro_rules! bit {
    ($b:expr) => {{
        1 << $b
    }};
}

bitflags! {
    pub struct EventCategory: u32  {
        const NONE = 0;
        const EVENT_CATEGORY_APPLICATION = bit!(0);
        const EVENT_CATEGORY_INPUT = bit!(1);
        const EVENT_CATEGORY_KEYBOARD = bit!(2);
        const EVENT_CATEGORY_MOUSE = bit!(3);
        const EVENT_CATEGORY_MOUSE_BUTTON = bit!(4);
    }
}
pub enum EventType {
    None,
    WindowClose,
    WindowResize {
        width: EventData,
        height: EventData,
    },
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed {
        key_code: KeyCode,
        repeat: EventData,
    },
    KeyReleased {
        key_code: KeyCode,
    },
    MouseButtonPressed {
        button: EventData,
    },
    MouseButtonReleased {
        button: EventData,
    },
    MouseMoved {
        x_mouse: EventData,
        y_mouse: EventData,
    },
    MouseScrolled {
        x_offset: EventData,
        y_offset: EventData,
    },
}

pub struct Event {
    catogories: EventCategory,
    name: &'static str,
    handled: bool,
    data: EventType,
}

impl Event {
    pub fn none() -> Event {
        Event {
            catogories: EventCategory::NONE,
            name: "None",
            handled: false,
            data: EventType::None,
        }
    }

    pub fn key_pressed(key_code: KeyCode, repeat: u32) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_KEYBOARD
                | EventCategory::EVENT_CATEGORY_INPUT,
            name: "KeyPressed",
            handled: false,
            data: EventType::KeyPressed { key_code, repeat },
        }
    }
}

pub trait EventObserver {
    fn notify(&self, event: &Event);
    fn can_handle(&self, event: &Event) -> bool;
}

pub struct EventDispatcher<'a> {
    event: &'a Event,
}

impl<'a> EventDispatcher<'a> {
    pub fn new(event: &'a Event) -> EventDispatcher<'a> {
        EventDispatcher { event }
    }

    pub fn dispatch<T: EventObserver>(&self, observer: &T) {
        if observer.can_handle(self.event) {
            observer.notify(self.event);
        }
    }
}
