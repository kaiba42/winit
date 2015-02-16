use events::VirtualKeyCode;
use winapi;

pub fn vkeycode_to_element(code: winapi::WPARAM) -> Option<VirtualKeyCode> {
    match code {
        //winapi::VK_LBUTTON => Some(VirtualKeyCode::Lbutton),
        //winapi::VK_RBUTTON => Some(VirtualKeyCode::Rbutton),
        //winapi::VK_CANCEL => Some(VirtualKeyCode::Cancel),
        //winapi::VK_MBUTTON => Some(VirtualKeyCode::Mbutton),
        //winapi::VK_XBUTTON1 => Some(VirtualKeyCode::Xbutton1),
        //winapi::VK_XBUTTON2 => Some(VirtualKeyCode::Xbutton2),
        winapi::VK_BACK => Some(VirtualKeyCode::Back),
        winapi::VK_TAB => Some(VirtualKeyCode::Tab),
        //winapi::VK_CLEAR => Some(VirtualKeyCode::Clear),
        winapi::VK_RETURN => Some(VirtualKeyCode::Return),
        //winapi::VK_SHIFT => Some(VirtualKeyCode::Shift),
        //winapi::VK_CONTROL => Some(VirtualKeyCode::Control),
        //winapi::VK_MENU => Some(VirtualKeyCode::Menu),
        winapi::VK_PAUSE => Some(VirtualKeyCode::Pause),
        winapi::VK_CAPITAL => Some(VirtualKeyCode::Capital),
        winapi::VK_KANA => Some(VirtualKeyCode::Kana),
        //winapi::VK_HANGUEL => Some(VirtualKeyCode::Hanguel),
        //winapi::VK_HANGUL => Some(VirtualKeyCode::Hangul),
        //winapi::VK_JUNJA => Some(VirtualKeyCode::Junja),
        //winapi::VK_FINAL => Some(VirtualKeyCode::Final),
        //winapi::VK_HANJA => Some(VirtualKeyCode::Hanja),
        winapi::VK_KANJI => Some(VirtualKeyCode::Kanji),
        winapi::VK_ESCAPE => Some(VirtualKeyCode::Escape),
        winapi::VK_CONVERT => Some(VirtualKeyCode::Convert),
        //winapi::VK_NONCONVERT => Some(VirtualKeyCode::Nonconvert),
        //winapi::VK_ACCEPT => Some(VirtualKeyCode::Accept),
        //winapi::VK_MODECHANGE => Some(VirtualKeyCode::Modechange),
        winapi::VK_SPACE => Some(VirtualKeyCode::Space),
        winapi::VK_PRIOR => Some(VirtualKeyCode::PageUp),
        winapi::VK_NEXT => Some(VirtualKeyCode::PageDown),
        winapi::VK_END => Some(VirtualKeyCode::End),
        winapi::VK_HOME => Some(VirtualKeyCode::Home),
        winapi::VK_LEFT => Some(VirtualKeyCode::Left),
        winapi::VK_UP => Some(VirtualKeyCode::Up),
        winapi::VK_RIGHT => Some(VirtualKeyCode::Right),
        winapi::VK_DOWN => Some(VirtualKeyCode::Down),
        //winapi::VK_SELECT => Some(VirtualKeyCode::Select),
        //winapi::VK_PRINT => Some(VirtualKeyCode::Print),
        //winapi::VK_EXECUTE => Some(VirtualKeyCode::Execute),
        winapi::VK_SNAPSHOT => Some(VirtualKeyCode::Snapshot),
        winapi::VK_INSERT => Some(VirtualKeyCode::Insert),
        winapi::VK_DELETE => Some(VirtualKeyCode::Delete),
        //winapi::VK_HELP => Some(VirtualKeyCode::Help),
        0x30 => Some(VirtualKeyCode::Key0),
        0x31 => Some(VirtualKeyCode::Key1),
        0x32 => Some(VirtualKeyCode::Key2),
        0x33 => Some(VirtualKeyCode::Key3),
        0x34 => Some(VirtualKeyCode::Key4),
        0x35 => Some(VirtualKeyCode::Key5),
        0x36 => Some(VirtualKeyCode::Key6),
        0x37 => Some(VirtualKeyCode::Key7),
        0x38 => Some(VirtualKeyCode::Key8),
        0x39 => Some(VirtualKeyCode::Key9),
        0x41 => Some(VirtualKeyCode::A),
        0x42 => Some(VirtualKeyCode::B),
        0x43 => Some(VirtualKeyCode::C),
        0x44 => Some(VirtualKeyCode::D),
        0x45 => Some(VirtualKeyCode::E),
        0x46 => Some(VirtualKeyCode::F),
        0x47 => Some(VirtualKeyCode::G),
        0x48 => Some(VirtualKeyCode::H),
        0x49 => Some(VirtualKeyCode::I),
        0x4A => Some(VirtualKeyCode::J),
        0x4B => Some(VirtualKeyCode::K),
        0x4C => Some(VirtualKeyCode::L),
        0x4D => Some(VirtualKeyCode::M),
        0x4E => Some(VirtualKeyCode::N),
        0x4F => Some(VirtualKeyCode::O),
        0x50 => Some(VirtualKeyCode::P),
        0x51 => Some(VirtualKeyCode::Q),
        0x52 => Some(VirtualKeyCode::R),
        0x53 => Some(VirtualKeyCode::S),
        0x54 => Some(VirtualKeyCode::T),
        0x55 => Some(VirtualKeyCode::U),
        0x56 => Some(VirtualKeyCode::V),
        0x57 => Some(VirtualKeyCode::W),
        0x58 => Some(VirtualKeyCode::X),
        0x59 => Some(VirtualKeyCode::Y),
        0x5A => Some(VirtualKeyCode::Z),
        //winapi::VK_LWIN => Some(VirtualKeyCode::Lwin),
        //winapi::VK_RWIN => Some(VirtualKeyCode::Rwin),
        winapi::VK_APPS => Some(VirtualKeyCode::Apps),
        winapi::VK_SLEEP => Some(VirtualKeyCode::Sleep),
        winapi::VK_NUMPAD0 => Some(VirtualKeyCode::Numpad0),
        winapi::VK_NUMPAD1 => Some(VirtualKeyCode::Numpad1),
        winapi::VK_NUMPAD2 => Some(VirtualKeyCode::Numpad2),
        winapi::VK_NUMPAD3 => Some(VirtualKeyCode::Numpad3),
        winapi::VK_NUMPAD4 => Some(VirtualKeyCode::Numpad4),
        winapi::VK_NUMPAD5 => Some(VirtualKeyCode::Numpad5),
        winapi::VK_NUMPAD6 => Some(VirtualKeyCode::Numpad6),
        winapi::VK_NUMPAD7 => Some(VirtualKeyCode::Numpad7),
        winapi::VK_NUMPAD8 => Some(VirtualKeyCode::Numpad8),
        winapi::VK_NUMPAD9 => Some(VirtualKeyCode::Numpad9),
        winapi::VK_MULTIPLY => Some(VirtualKeyCode::Multiply),
        winapi::VK_ADD => Some(VirtualKeyCode::Add),
        //winapi::VK_SEPARATOR => Some(VirtualKeyCode::Separator),
        winapi::VK_SUBTRACT => Some(VirtualKeyCode::Subtract),
        winapi::VK_DECIMAL => Some(VirtualKeyCode::Decimal),
        winapi::VK_DIVIDE => Some(VirtualKeyCode::Divide),
        winapi::VK_F1 => Some(VirtualKeyCode::F1),
        winapi::VK_F2 => Some(VirtualKeyCode::F2),
        winapi::VK_F3 => Some(VirtualKeyCode::F3),
        winapi::VK_F4 => Some(VirtualKeyCode::F4),
        winapi::VK_F5 => Some(VirtualKeyCode::F5),
        winapi::VK_F6 => Some(VirtualKeyCode::F6),
        winapi::VK_F7 => Some(VirtualKeyCode::F7),
        winapi::VK_F8 => Some(VirtualKeyCode::F8),
        winapi::VK_F9 => Some(VirtualKeyCode::F9),
        winapi::VK_F10 => Some(VirtualKeyCode::F10),
        winapi::VK_F11 => Some(VirtualKeyCode::F11),
        winapi::VK_F12 => Some(VirtualKeyCode::F12),
        winapi::VK_F13 => Some(VirtualKeyCode::F13),
        winapi::VK_F14 => Some(VirtualKeyCode::F14),
        winapi::VK_F15 => Some(VirtualKeyCode::F15),
        /*winapi::VK_F16 => Some(VirtualKeyCode::F16),
        winapi::VK_F17 => Some(VirtualKeyCode::F17),
        winapi::VK_F18 => Some(VirtualKeyCode::F18),
        winapi::VK_F19 => Some(VirtualKeyCode::F19),
        winapi::VK_F20 => Some(VirtualKeyCode::F20),
        winapi::VK_F21 => Some(VirtualKeyCode::F21),
        winapi::VK_F22 => Some(VirtualKeyCode::F22),
        winapi::VK_F23 => Some(VirtualKeyCode::F23),
        winapi::VK_F24 => Some(VirtualKeyCode::F24),*/
        winapi::VK_NUMLOCK => Some(VirtualKeyCode::Numlock),
        winapi::VK_SCROLL => Some(VirtualKeyCode::Scroll),
        /*winapi::VK_LSHIFT => Some(VirtualKeyCode::Lshift),
        winapi::VK_RSHIFT => Some(VirtualKeyCode::Rshift),
        winapi::VK_LCONTROL => Some(VirtualKeyCode::Lcontrol),
        winapi::VK_RCONTROL => Some(VirtualKeyCode::Rcontrol),
        winapi::VK_LMENU => Some(VirtualKeyCode::Lmenu),
        winapi::VK_RMENU => Some(VirtualKeyCode::Rmenu),
        winapi::VK_BROWSER_BACK => Some(VirtualKeyCode::Browser_back),
        winapi::VK_BROWSER_FORWARD => Some(VirtualKeyCode::Browser_forward),
        winapi::VK_BROWSER_REFRESH => Some(VirtualKeyCode::Browser_refresh),
        winapi::VK_BROWSER_STOP => Some(VirtualKeyCode::Browser_stop),
        winapi::VK_BROWSER_SEARCH => Some(VirtualKeyCode::Browser_search),
        winapi::VK_BROWSER_FAVORITES => Some(VirtualKeyCode::Browser_favorites),
        winapi::VK_BROWSER_HOME => Some(VirtualKeyCode::Browser_home),
        winapi::VK_VOLUME_MUTE => Some(VirtualKeyCode::Volume_mute),
        winapi::VK_VOLUME_DOWN => Some(VirtualKeyCode::Volume_down),
        winapi::VK_VOLUME_UP => Some(VirtualKeyCode::Volume_up),
        winapi::VK_MEDIA_NEXT_TRACK => Some(VirtualKeyCode::Media_next_track),
        winapi::VK_MEDIA_PREV_TRACK => Some(VirtualKeyCode::Media_prev_track),
        winapi::VK_MEDIA_STOP => Some(VirtualKeyCode::Media_stop),
        winapi::VK_MEDIA_PLAY_PAUSE => Some(VirtualKeyCode::Media_play_pause),
        winapi::VK_LAUNCH_MAIL => Some(VirtualKeyCode::Launch_mail),
        winapi::VK_LAUNCH_MEDIA_SELECT => Some(VirtualKeyCode::Launch_media_select),
        winapi::VK_LAUNCH_APP1 => Some(VirtualKeyCode::Launch_app1),
        winapi::VK_LAUNCH_APP2 => Some(VirtualKeyCode::Launch_app2),
        winapi::VK_OEM_1 => Some(VirtualKeyCode::Oem_1),
        winapi::VK_OEM_PLUS => Some(VirtualKeyCode::Oem_plus),
        winapi::VK_OEM_COMMA => Some(VirtualKeyCode::Oem_comma),
        winapi::VK_OEM_MINUS => Some(VirtualKeyCode::Oem_minus),
        winapi::VK_OEM_PERIOD => Some(VirtualKeyCode::Oem_period),
        winapi::VK_OEM_2 => Some(VirtualKeyCode::Oem_2),
        winapi::VK_OEM_3 => Some(VirtualKeyCode::Oem_3),
        winapi::VK_OEM_4 => Some(VirtualKeyCode::Oem_4),
        winapi::VK_OEM_5 => Some(VirtualKeyCode::Oem_5),
        winapi::VK_OEM_6 => Some(VirtualKeyCode::Oem_6),
        winapi::VK_OEM_7 => Some(VirtualKeyCode::Oem_7),
        winapi::VK_OEM_8 => Some(VirtualKeyCode::Oem_8),
        winapi::VK_OEM_102 => Some(VirtualKeyCode::Oem_102),
        winapi::VK_PROCESSKEY => Some(VirtualKeyCode::Processkey),
        winapi::VK_PACKET => Some(VirtualKeyCode::Packet),
        winapi::VK_ATTN => Some(VirtualKeyCode::Attn),
        winapi::VK_CRSEL => Some(VirtualKeyCode::Crsel),
        winapi::VK_EXSEL => Some(VirtualKeyCode::Exsel),
        winapi::VK_EREOF => Some(VirtualKeyCode::Ereof),
        winapi::VK_PLAY => Some(VirtualKeyCode::Play),
        winapi::VK_ZOOM => Some(VirtualKeyCode::Zoom),
        winapi::VK_NONAME => Some(VirtualKeyCode::Noname),
        winapi::VK_PA1 => Some(VirtualKeyCode::Pa1),
        winapi::VK_OEM_CLEAR => Some(VirtualKeyCode::Oem_clear),*/
        _ => None
    }
}
