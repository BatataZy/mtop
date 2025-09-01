use crate::{
    io::read,
    unit_types::{Magnitude, Size},
};

#[derive(Debug)]
pub struct Gpu {
    pub clock: Magnitude,
    pub util: Magnitude,
    pub memory: Size,
    pub temp: f32,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            clock: Magnitude::new("/sys/class/drm/renderD128/device/pp_dpm_sclk"),
            util: Magnitude::new("/sys/class/drm/renderD128/device/gpu_busy_percent"),
            memory: Size::new(
                "/sys/class/drm/renderD128/device/mem_info_vram_used",
                read("/sys/class/drm/renderD128/device/mem_info_vram_total", 0, 0)
                    .parse::<usize>()
                    .expect("fully numeric string")
                    / 1_048_576,
            ),
            temp: 0.0,
        }
    }

    pub fn update(&mut self) {
        //GPU CLOCK LOGIC
        self.clock.add(match self.clock.read(14, 17).as_str() {
            " 70" => 700,
            s => s.parse::<u16>().expect("fully numeric string"),
        });

        //GPU UTIL LOGIC
        self.util.add(
            self.util
                .read(0, 0)
                .parse::<u16>()
                .expect("fully numeric string"),
        );

        //GPU MEMORY LOGIC
        self.memory.used = self
            .memory
            .read(0, 0)
            .parse::<usize>()
            .expect("fully numeric string")
            / 1_048_576;
    }
}
