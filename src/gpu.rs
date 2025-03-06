use crate::{read, unit_types::{Magnitude, Percent, Size}};

#[derive(Debug)]
pub struct Gpu{
    pub card: u8,
    pub clock: Magnitude,
    pub util: Percent,
    pub memory: Size,
    pub temp: f32,
}

impl Gpu {
    pub fn new(buf: &mut String) -> Gpu {
        Gpu {
            card: 1,
            clock: Magnitude::new(),
            util: Percent::new(),
            memory: Size::new(read("/sys/class/drm/card1/device/mem_info_vram_total", buf, 0, 0).parse::<usize>().unwrap()/1048576),
            temp: 0.0,
        }
    }


    pub async fn update(&mut self, buf: &mut String) {

        //GPU CLOCK LOGIC

        let current_clock = match read(&("/sys/class/drm/card".to_owned() + &self.card.to_string() + "/device/pp_dpm_sclk"), buf, 14, 17).as_str() {
            " 70" => 700,
            x => x.parse::<u16>().unwrap(),
        };
        self.clock.add(current_clock);

        //GPU UTIL LOGIC
        let current_util = read(&("/sys/class/drm/card".to_owned() + &self.card.to_string() + "/device/gpu_busy_percent"), buf,0,0).parse::<u8>().unwrap();

        self.util.add(current_util);

        //GPU MEMORY LOGIC

        self.memory.used = read(&("/sys/class/drm/card".to_owned() + &self.card.to_string() + "/device/mem_info_vram_used"), buf, 0, 0).parse::<usize>().unwrap()/1048576;
    }
}