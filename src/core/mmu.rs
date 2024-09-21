
pub const ROM_SIZE: usize= 0x8000;
pub const VIDEO_RAM_SIZE: usize= 0x8000;
pub const EXTERNAL_RAM_SIZE: usize= 0x2000;
pub const WORK_RAM_SIZE: usize= 0x2000;
pub const OAM_SIZE: usize= 0x00A0;
pub const HIGH_RAM_SIZE: usize= 0x007F;

pub struct MMU {

    rom: [u8; ROM_SIZE],
    video_ram: [u8; VIDEO_RAM_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    work_ram: [u8; WORK_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    high_ram: [u8; HIGH_RAM_SIZE],

    // Interrupt enable flag
    ie: u8
    
}

impl MMU {

    pub fn new() -> Self {

        MMU {

            rom: [0; ROM_SIZE],
            video_ram: [0; VIDEO_RAM_SIZE],
            external_ram: [0; EXTERNAL_RAM_SIZE],
            work_ram: [0; WORK_RAM_SIZE],
            oam: [0; OAM_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            ie: 0x0

        }

    }

    pub fn reset(&mut self) {

        self.rom= [0; ROM_SIZE];
        self.video_ram= [0; VIDEO_RAM_SIZE];
        self.external_ram= [0; EXTERNAL_RAM_SIZE];
        self.work_ram= [0; WORK_RAM_SIZE];
        self.oam= [0; OAM_SIZE];
        self.high_ram= [0; HIGH_RAM_SIZE];

    }

}