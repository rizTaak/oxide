use bitflags::bitflags;

use super::app::Application;

type KeyCode = i32;
type EventData = f64;

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

#[derive(Debug)]
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
    },
    KeyReleased {
        key_code: KeyCode,
    },
    KeyTyped {
        key_code: KeyCode,
    },
    MouseButtonPressed {
        button: i32,
    },
    MouseButtonReleased {
        button: i32,
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

#[derive(Debug)]
pub struct Event {
    pub catogories: EventCategory,
    pub name: &'static str,
    pub handled: bool,
    pub data: EventType,
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

    pub fn key_pressed(key_code: KeyCode) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_KEYBOARD
                | EventCategory::EVENT_CATEGORY_INPUT,
            name: "KeyPressed",
            handled: false,
            data: EventType::KeyPressed { key_code },
        }
    }

    pub fn key_release(key_code: KeyCode) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_KEYBOARD
                | EventCategory::EVENT_CATEGORY_INPUT,
            name: "KeyReleased",
            handled: false,
            data: EventType::KeyReleased { key_code },
        }
    }

    pub fn key_typed(key_code: KeyCode) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_KEYBOARD
                | EventCategory::EVENT_CATEGORY_INPUT,
            name: "KeyTyped",
            handled: false,
            data: EventType::KeyTyped { key_code },
        }
    }

    pub fn close() -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_APPLICATION,
            name: "WindowClose",
            handled: false,
            data: EventType::WindowClose,
        }
    }

    pub fn mouse_move(x: f64, y: f64) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_MOUSE | EventCategory::EVENT_CATEGORY_INPUT,
            name: "MouseMove",
            handled: false,
            data: EventType::MouseMoved {
                x_mouse: x,
                y_mouse: y,
            },
        }
    }

    pub fn mouse_button_pressed(x: i32) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_MOUSE_BUTTON
                | EventCategory::EVENT_CATEGORY_INPUT,
            name: "MouseButtonPressed",
            handled: false,
            data: EventType::MouseButtonPressed { button: x },
        }
    }

    pub fn mouse_button_released(x: i32) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_MOUSE_BUTTON
                | EventCategory::EVENT_CATEGORY_INPUT,
            name: "MouseButtonReleased",
            handled: false,
            data: EventType::MouseButtonReleased { button: x },
        }
    }

    pub fn mouse_scrolled(x: f64, y: f64) -> Event {
        Event {
            catogories: EventCategory::EVENT_CATEGORY_MOUSE_BUTTON
                | EventCategory::EVENT_CATEGORY_INPUT,
            name: "MouseScroll",
            handled: false,
            data: EventType::MouseScrolled {
                x_offset: x,
                y_offset: y,
            },
        }
    }
}

pub struct EventDispatcher<'a> {
    event: &'a Event,
}

impl<'a> EventDispatcher<'a> {
    pub fn new(event: &'a Event) -> EventDispatcher<'a> {
        EventDispatcher { event }
    }

    pub fn dispatch<A: Application>(&self, app: &mut A) {
        app.notify(&self.event);
    }
}
