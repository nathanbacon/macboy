pub enum GpuEvent {
    None,
    LCD,
    VBlank,
}

pub struct LCDC {
    lcd_enabled: bool,            // bit 7
    window_tile_map_select: bool, // bit 6
    window_display_toggle: bool,  // bit 5
    bg_window_tile_select: bool,  // bit 4
    bg_tile_map_select: bool,     // bit 3
    sprite_size: bool,            // bit 2
    sprite_display: bool,         // bit 1
    bg_display: bool,             // bit 0
}

impl LCDC {
    pub fn set(&mut self, val: u8) {
        self.lcd_enabled = (val & 0x80) > 0;
        self.window_tile_map_select = (val & 0x40) > 0;
        self.window_display_toggle = (val & 0x20) > 0;
        self.bg_window_tile_select = (val & 0x10) > 0;
        self.bg_tile_map_select = (val & 0x08) > 0;
        self.sprite_size = (val & 0x04) > 0;
        self.sprite_display = (val & 0x02) > 0;
        self.bg_display = (val & 0x01) > 0;
    }

    pub fn get(&self) -> u8 {
        0u8
    }

    pub fn new() -> LCDC {
        LCDC {
            lcd_enabled: false,
            window_tile_map_select: false,
            window_display_toggle: false,
            bg_window_tile_select: false,
            bg_tile_map_select: false,
            sprite_size: false,
            sprite_display: false,
            bg_display: false,
        }
    }
}

pub struct STAT {
    lyc_ly_coincidence: bool,
}

pub struct VRAM {
    memory: Box<[u8; 0x2000]>,
    oam: Box<[u8; 0xA0]>,
    lcdc: LCDC,
    ticks: u64,
}

impl VRAM {
    pub fn new() -> VRAM {
        VRAM {
            memory: Box::new([0u8; 0x2000]),
            oam: Box::new([0u8; 0xA0]),
            ticks: 0,
            lcdc: LCDC::new(),
        }
    }

    pub fn go(&mut self, ticks: u64) -> GpuEvent {
        self.ticks = self.ticks + ticks;
        GpuEvent::None
    }

    pub fn read(&self, address: usize) -> u8 {
        let address = address & 0x1FFF;
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        let address = address & 0x1FFF;
        self.memory[address] = value;
    }

    pub fn subscript(&mut self, index: usize) -> &mut u8 {
        &mut self.memory[index]
    }
}
