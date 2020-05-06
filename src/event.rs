// Pushrod Events
// Event Definitions
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// These are the events that are generated by the Pushrod Event translation system for SDL2.
/// These events are passed into the `handle_event` function, which contains the currently
/// translated events.
#[derive(Debug, Clone)]
pub enum PushrodEvent {
    /// Mouse movement detected, coordinates are based on the X/Y position within the widget, not
    /// relative to the window.
    MouseMoved {
        /// The `Widget` ID.
        widget_id: u32,

        /// X coordinate of the mouse pointer.
        x: u32,

        /// Y coordinate of the mouse pointer.
        y: i32,
    },

    /// Mouse wheel scrolled.
    MouseScrolled {
        /// The `Widget` ID.
        widget_id: u32,

        /// Horizontal scrolling direction, and amount.  Positive numbers indicate movement to
        /// the right.  Negative indicates to the left.
        hor: i32,

        /// Vertical scrolling direction, and amount.  Positive numbers indicate downward movement.
        /// Negative numbers indicate upward movement.
        ver: i32,
    },

    /// Mouse button click/release.
    MouseButton {
        /// The `Widget` ID.
        widget_id: u32,

        /// The button ID that was clicked.
        button: u32,

        /// Pressed/released state: `true` indicates mouse button press, `false` is released.
        state: bool,
    },

    /// Widget was clicked.
    WidgetClicked {
        /// The `Widget` ID.
        widget_id: u32,

        /// The button ID that was clicked.
        button: u32,

        /// Number of clicks registered.
        clicks: u8,
    },

    /// Widget was selected.
    WidgetSelected {
        /// The `Widget` ID.
        widget_id: u32,

        /// Selection state: `true` indicates selected, `false` indicates unselected.
        state: bool,
    },

    /// Widget state was toggled.
    WidgetToggled {
        /// The `Widget` ID.
        widget_id: u32,

        /// Toggle state: `true` indicates toggled, `false` indicates untoggled.
        state: bool,
    },

    /// Widget radio button state was selected.
    WidgetRadioSelected {
        /// The `Widget` ID.
        widget_id: u32,

        /// The group ID.
        group_id: u32,
    },

    /// Widget radio button state was unselected.
    WidgetRadioUnselected {
        /// The `Widget` ID.
        widget_id: u32,

        /// The group ID.
        group_id: u32,
    },

    /// Mouse entered the scope of a widget.
    WidgetMouseEntered {
        /// The `Widget` ID.
        widget_id: u32,
    },

    /// Mouse exited the scope of a widget.
    WidgetMouseExited {
        /// The `Widget` ID.
        widget_id: u32,
    },

    /// Widget gained focus through a `[TAB]` key, or other means.
    WidgetFocusGained {
        /// The `Widget` ID.
        widget_id: u32,
    },

    /// Widget lost focus.
    WidgetFocusLost {
        /// The `Widget` ID.
        widget_id: u32,
    },

    /// Tab was selected inside a `Tab Widget` object.
    WidgetTabSelected {
        /// The `Widget` ID.
        widget_id: u32,

        /// The ID of the tab that was selected.
        tab_id: u8,
    },

    /// Indicates that a value contained within a `Widget` was altered.
    WidgetValueChanged {
        /// The `Widget` ID.
        widget_id: u32,

        /// The value before the change.
        old_value: u32,

        /// The new value.
        new_value: u32,
    },

    /// Indicates a `Widget`'s position was altered within the bounds of the `Window`.
    WidgetMoved {
        /// The `Widget` ID.
        widget_id: u32,

        /// The `Widget`'s new X coordinates in relation to the `Window`.
        new_x: u32,

        /// The `Widget`'s new Y coordinates in relation to the `Window`.
        new_y: u32,
    },

    /// Indicates that the visibility of a `Widget` on the screen has physically changed.
    /// This is an indication of the visibility based on its alpha value.  An alpha value
    /// of 100 indicates that the object is completely visible, whereas an alpha of 0
    /// indicates that it is hidden.
    WidgetVisibilityChanged {
        /// The `Widget` ID.
        widget_id: u32,

        /// The `Widget`'s visible alpha value: 0 to 100.
        alpha: u32,

        /// Indicates whether or not the `Widget` has disappeared from the `Window`.  This can
        /// be set to true or false, even if the alpha value is greater than 0.  If the value is
        /// set `true`, it means the object is not visible.  `false` otherwise.  If the alpha is
        /// 0, and hidden is `false`, it will still indicate a visibility of 0, which will mean
        /// the object cannot be interacted with.
        hidden: bool,
    }

}

pub enum Event {
    Pushrod(PushrodEvent),
    SDL2(sdl2::event::Event),
}
