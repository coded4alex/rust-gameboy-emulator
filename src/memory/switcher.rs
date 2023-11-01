use super::access::Memory;

pub struct Switcher {
    banks: Vec<[u8; 0x4000]>,
    current: u8,
    upper: u16,
    lower: u16,
}

impl Switcher{

    pub fn new(lower: u16, upper: u16) -> Switcher {
        Switcher {
            banks: vec![[0; 0x4000]; 256],
            current: 0,
            upper: upper,
            lower: lower,
        }
    }

    fn get_current_bank(&self) -> u8 {
        self.current
    }

    fn set_current_bank(&mut self, bank: u8, mem: &mut Memory) {
        let size = self.upper - self.lower;
        
        for i in 0..size {            
            self.banks[self.current as usize][i as usize] = mem.read(self.lower + i);
            mem.write(self.lower + i, self.banks[bank as usize][i as usize]);
        }
        self.current = bank;
    }
}

mod test {
    use super::Switcher;
    use crate::memory::access::create_memory;

    #[test]
    fn test_switchable() {
        let mut mem = create_memory();
        let mut switchable = Switcher::new(0x0000, 0x4000);

        switchable.set_current_bank(0x00, &mut mem);
        mem.write(0x0000, 0x01);
        switchable.set_current_bank(0x01, &mut mem);

        mem.write(0x0000, 0x02);
        assert_eq!(mem.read(0x0000), 0x02);
        assert_eq!(switchable.get_current_bank(), 0x01);

        switchable.set_current_bank(0x00, &mut mem);
        assert_eq!(mem.read(0x0000), 0x01);
    }
}