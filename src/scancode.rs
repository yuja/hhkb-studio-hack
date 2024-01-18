/// Translates HHKB Studio scancode to short string label.
pub fn scancode_to_label(code: u16) -> Option<&'static str> {
    match code {
        // 0x0000..0x00e8: USB HID Keyboard (with some HHKB specific mappings)
        0x0000 => None, // Reserved
        0x0001 => None, // Error roll over
        0x0002 => None, // POST fail
        0x0003 => None, // Error undefined
        0x0004 => Some("A"),
        0x0005 => Some("B"),
        0x0006 => Some("C"),
        0x0007 => Some("D"),
        0x0008 => Some("E"),
        0x0009 => Some("F"),
        0x000a => Some("G"),
        0x000b => Some("H"),
        0x000c => Some("I"),
        0x000d => Some("J"),
        0x000e => Some("K"),
        0x000f => Some("L"),
        0x0010 => Some("M"),
        0x0011 => Some("N"),
        0x0012 => Some("O"),
        0x0013 => Some("P"),
        0x0014 => Some("Q"),
        0x0015 => Some("R"),
        0x0016 => Some("S"),
        0x0017 => Some("T"),
        0x0018 => Some("U"),
        0x0019 => Some("V"),
        0x001a => Some("W"),
        0x001b => Some("X"),
        0x001c => Some("Y"),
        0x001d => Some("Z"),
        0x001e => Some("1 !"),
        0x001f => Some("2 @"),
        0x0020 => Some("3 #"),
        0x0021 => Some("4 $"),
        0x0022 => Some("5 %"),
        0x0023 => Some("6 ^"),
        0x0024 => Some("7 &"),
        0x0025 => Some("8 *"),
        0x0026 => Some("9 ("),
        0x0027 => Some("0 )"),
        0x0028 => Some("Return"),
        0x0029 => Some("Esc"),
        0x002a => Some("Backspace"),
        0x002b => Some("Tab"),
        0x002c => Some("Space"),
        0x002d => Some("- _"),
        0x002e => Some("= +"),
        0x002f => Some("[ {"),
        0x0030 => Some("] }"),
        0x0031 => Some("\\ |"),
        0x0032 => Some("# ~"), // Int
        0x0033 => Some("; :"),
        0x0034 => Some("' \""),
        0x0035 => Some("` ~"),
        0x0036 => Some(", <"),
        0x0037 => Some(". >"),
        0x0038 => Some("/ ?"),
        0x0039 => Some("Caps"),
        0x003a => Some("F1"),
        0x003b => Some("F2"),
        0x003c => Some("F3"),
        0x003d => Some("F4"),
        0x003e => Some("F5"),
        0x003f => Some("F6"),
        0x0040 => Some("F7"),
        0x0041 => Some("F8"),
        0x0042 => Some("F9"),
        0x0043 => Some("F10"),
        0x0044 => Some("F11"),
        0x0045 => Some("F12"),
        0x0046 => Some("PrtSc"),
        0x0047 => Some("ScrLock"),
        0x0048 => Some("Pause"),
        0x0049 => Some("Insert"),
        0x004a => Some("Home"),
        0x004b => Some("PgUp"),
        0x004c => Some("Delete"),
        0x004d => Some("End"),
        0x004e => Some("PgDn"),
        0x004f => Some("Right"),
        0x0050 => Some("Left"),
        0x0051 => Some("Down"),
        0x0052 => Some("Up"),
        0x0053 => Some("KP NumLock"),
        0x0054 => Some("KP /"),
        0x0055 => Some("KP *"),
        0x0056 => Some("KP -"),
        0x0057 => Some("KP +"),
        0x0058 => Some("KP Enter"),
        0x0059 => Some("KP 1"),
        0x005a => Some("KP 2"),
        0x005b => Some("KP 3"),
        0x005c => Some("KP 4"),
        0x005d => Some("KP 5"),
        0x005e => Some("KP 6"),
        0x005f => Some("KP 7"),
        0x0060 => Some("KP 8"),
        0x0061 => Some("KP 9"),
        0x0062 => Some("KP 0"),
        0x0063 => Some("KP ."),
        0x0064 => Some("\\ |"), // Int
        0x0065 => Some("Application"),
        0x0066 => None, // Sun Power
        0x0067 => Some("KP ="),
        0x0068 => Some("F13"),
        0x0069 => Some("F14"),
        0x006a => Some("F15"),
        0x006b => Some("F16"),
        0x006c => Some("F17"),
        0x006d => Some("F18"),
        0x006e => Some("F19"),
        0x006f => Some("F20"),
        0x0070 => Some("F21"),
        0x0071 => Some("F22"),
        0x0072 => Some("F23"),
        0x0073 => Some("F24"),
        0x0074 => Some("Execute"),
        0x0075 => Some("Help"),
        0x0076 => Some("Menu"),
        0x0077 => Some("Select"),
        0x0078 => Some("Stop"),
        0x0079 => Some("Again"),
        0x007a => Some("Undo"),
        0x007b => Some("Cut"),
        0x007c => Some("Copy"),
        0x007d => Some("Paste"),
        0x007e => Some("Find"),
        0x007f => None, // Sun Mute
        0x0080 => None, // Sun VolumeUp
        0x0081 => None, // Sun VolumeDown
        0x0082 => Some("Locking CapsLock"),
        0x0083 => Some("Locking NumLock"),
        0x0084 => Some("Locking ScrollLock"),
        0x0085 => Some("KP ,"),
        0x0086 => Some("KP EqualsSign"),
        0x0087 => Some("\\ Ro"), // Int1
        0x0088 => Some("Kana"),  // Int2
        0x0089 => Some("Yen"),   // Int3
        0x008a => Some("Xfer"),  // Int4
        0x008b => Some("Nfer"),  // Int5
        0x008c => Some("Int6"),
        0x008d => Some("Int7"),
        0x008e => Some("Int8"),
        0x008f => Some("Int9"),
        0x0090 => Some("Lang1"),
        0x0091 => Some("Lang2"),
        0x0092 => Some("Lang3"),
        0x0093 => Some("Lang4"),
        0x0094 => Some("Lang5"),
        0x0095 => Some("Lang6"),
        0x0096 => Some("Lang7"),
        0x0097 => Some("Lang8"),
        0x0098 => Some("Lang9"),
        0x0099 => Some("Alternative Erase"),
        0x009a => Some("SysReq"),
        0x009b => Some("Cancel"),
        0x009c => Some("Clear"),
        0x009d => Some("Prior"),
        0x009e => Some("Return"),
        0x009f => Some("Separator"),
        0x00a0 => Some("Out"),
        0x00a1 => Some("Oper"),
        0x00a2 => Some("Clear"),
        0x00a3 => Some("ClSel"),
        0x00a4 => Some("ExSel"),
        0x00a5 => Some("Power"), // HHKB
        0x00a6 => None,
        0x00a7 => None,
        0x00a8 => Some("Mute"),  // HHKB
        0x00a9 => Some("VolUP"), // HHKB
        0x00aa => Some("VolDn"), // HHKB
        0x00ab => None,
        0x00ac => None,
        0x00ad => None,
        0x00ae => None,
        0x00af => None,
        0x00b0 => Some("KP 00"),
        0x00b1 => Some("KP 000"),
        0x00b2 => Some("Thousands Separator"),
        0x00b3 => Some("Decimal Separator"),
        0x00b4 => Some("Currency Unit"),
        0x00b5 => Some("Currency Sub-unit"),
        0x00b6 => Some("KP ("),
        0x00b7 => Some("KP )"),
        0x00b8 => Some("KP {"),
        0x00b9 => Some("KP }"),
        0x00ba => Some("KP Tab"),
        0x00bb => Some("KP Backspace"),
        0x00bc => Some("KP A"),
        0x00bd => Some("KP B"),
        0x00be => Some("KP C"),
        0x00bf => Some("KP D"),
        0x00c0 => Some("KP E"),
        0x00c1 => Some("KP F"),
        0x00c2 => Some("KP XOR"),
        0x00c3 => Some("KP ^"),
        0x00c4 => Some("KP %"),
        0x00c5 => Some("KP <"),
        0x00c6 => Some("KP >"),
        0x00c7 => Some("KP &"),
        0x00c8 => Some("KP &&"),
        0x00c9 => Some("KP |"),
        0x00ca => Some("KP ||"),
        0x00cb => Some("KP :"),
        0x00cc => Some("KP #"),
        0x00cd => Some("KP Space"),
        0x00ce => Some("KP @"),
        0x00cf => Some("KP !"),
        0x00d0 => Some("KP Memory Store"),
        0x00d1 => Some("KP Memory Recall"),
        0x00d2 => Some("KP Memory Clear"),
        0x00d3 => Some("KP Memory Add"),
        0x00d4 => Some("KP Memory Subtract"),
        0x00d5 => Some("KP Memory Multiply"),
        0x00d6 => Some("KP Memory Divide"),
        0x00d7 => Some("KP +/-"),
        0x00d8 => Some("KP Clear"),
        0x00d9 => Some("KP Clear Entry"),
        0x00da => Some("KP Binary"),
        0x00db => Some("KP Octal"),
        0x00dc => Some("KP Decimal"),
        0x00dd => Some("KP Hexadecimal"),
        0x00de => None,
        0x00df => None,
        0x00e0 => Some("LControl"),
        0x00e1 => Some("LShift"),
        0x00e2 => Some("LAlt"),
        0x00e3 => Some("LMeta"), // or Super
        0x00e4 => Some("RControl"),
        0x00e5 => Some("RShift"),
        0x00e6 => Some("RAlt"),
        0x00e7 => Some("RMeta"), // or Super
        0x00e8 => None,
        0x00e9 => None,
        0x00ea => None,
        0x00eb => None,
        0x00ec => None,
        0x00ed => None,
        0x00ee => None,
        0x00ef => None,
        0x00f0 => None,
        0x00f1 => None,
        0x00f2 => None,
        0x00f3 => None,
        0x00f4 => Some("LB"), // HHKB left mouse button
        0x00f5 => Some("MB"), // HHKB middle mouse button
        0x00f6 => Some("RB"), // HHKB right mouse button
        0x00f7 => None,
        0x00f8 => None,
        0x00f9 => None,
        0x00fa => None,
        0x00fb => None,
        0x00fc => None,
        0x00fd => None,
        0x00fe => None,
        0x00ff => None,
        0x5101 => Some("Fn1"),
        0x5102 => Some("Fn2"),
        0x5103 => Some("Fn3"),
        _ => None,
    }
}
