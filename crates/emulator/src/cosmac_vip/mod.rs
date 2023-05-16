pub(crate) mod cpu;
pub(crate) mod display;
pub(crate) mod keypad;
pub(crate) mod memory;
pub(crate) mod opcode;
pub(crate) mod registers;
pub(crate) mod rom;
pub(crate) mod stack;

pub use rom::Rom;

use system::emulator::{Emulator, Keycode};
use system::{Result, System, Bitmap};

use self::cpu::Cpu;
use self::keypad::Keypad;
use self::memory::MEMORY_SIZE;

pub struct CosmacVip {
    cpu: Cpu,
}

impl CosmacVip {
    pub fn new(rom: Rom) -> Self {
        let mut cpu = Cpu::new();

        cpu.load(rom);

        Self { cpu }
    }
}

impl Emulator for CosmacVip {
    fn start(&mut self, system: &mut System) -> Result<()> {
        system.start();

        while let Some(keyboard_state) = system.next_keyboard_state() {
            if self.cpu.pc as usize >= MEMORY_SIZE {
                return Err(system::Error::EmulatorMemoryOutOfBounds(
                    MEMORY_SIZE,
                    self.cpu.pc as usize,
                ));
            }

            let mut keypad = Keypad::new();
            keyboard_state.pressed_scancodes().for_each(|code| {
                if let Some(keycode) = Keycode::from_scancode(code) {
                    match keycode {
                        Keycode::Num1 => keypad[0x1] = true,
                        Keycode::Num2 => keypad[0x2] = true,
                        Keycode::Num3 => keypad[0x3] = true,
                        Keycode::Num4 => keypad[0xC] = true,
                        Keycode::Q => keypad[0x4] = true,
                        Keycode::W => keypad[0x5] = true,
                        Keycode::E => keypad[0x6] = true,
                        Keycode::R => keypad[0xD] = true,
                        Keycode::A => keypad[0x7] = true,
                        Keycode::S => keypad[0x8] = true,
                        Keycode::D => keypad[0x9] = true,
                        Keycode::F => keypad[0xE] = true,
                        Keycode::Z => keypad[0xA] = true,
                        Keycode::X => keypad[0x0] = true,
                        Keycode::C => keypad[0xB] = true,
                        Keycode::V => keypad[0xF] = true,
                        _ => {}
                    }
                }
            });

            let cycle = self.cpu.cycle(keypad);

            if cycle.display_update {
                let bitmap = Bitmap::new(&cycle.display_buffer.0);

                system.render(&bitmap)?;
            }

            std::thread::sleep(std::time::Duration::from_millis(2));
        }

        Ok(())
    }
}
