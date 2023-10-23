use crate::memory::switcher::Switchable;
use crate::memory::access::Memory;

struct CatridgeSwitchable {
    banks: Vec<[u8; 0x4000]>,
    current: u8,
}

impl CatridgeSwitchable {
    pub fn new() -> CatridgeSwitchable {
        let mut rval = CatridgeSwitchable {
            banks: Vec::new(),
            current: 0,
        };
        for _ in 0..255 {
            rval.banks.push([0; 0x4000]);
        }
        rval
    }
}

impl Switchable for CatridgeSwitchable {
    fn get_valid_banks(&self) -> Vec<u8> {
        let mut rval: Vec<u8> = Vec::new();
        for i in 0..255 {
            rval[i] = i as u8;
        }
        rval
    }

    fn get_current_bank(&self) -> u8 {
        self.current
    }

    fn set_current_bank(&mut self, bank: u8, mem: &mut Memory) {
        self.current = bank;
        for i in 0..0x4000 {
            let temp = mem.read(0x4000 + i);
            mem.write(0x4000 + i, self.banks[bank as usize][i as usize]);
            self.banks[bank as usize][i as usize] = temp;
        }
    }
}


mod test {
    use crate::memory::switcher::Switchable;
    use super::CatridgeSwitchable;
    use crate::memory::access;

    #[test]
    fn test_switchable() {
        let mut mem = access::create_memory();
        let mut switchable = CatridgeSwitchable::new();
        switchable.set_current_bank(0, &mut mem);
        assert_eq!(switchable.get_current_bank(), 0);
    }
}