use super::switcher::Switcher;

pub struct Bank {
    pub name: String,
    pub lower_bound: u16,
    pub upper_bound: u16,
    pub current: u16,
    pub switcher: Option<Switcher>,
}

impl Bank {
    pub fn new(name: String, lower_bound: u16, upper_bound: u16, switcher: Option<Switcher>) -> Bank {
        Bank {
            name,
            lower_bound,
            upper_bound,
            current: 0,
            switcher,
        }
    }
}

pub fn create_banks() -> Vec<Bank> {
    Vec::new()
}

pub fn init_banks(banks: &mut Vec<Bank>) {
    let cartridge_switcher = Some(Switcher::new(0x4000, 0x8000));
    let video_switcher = Some(Switcher::new(0x8000, 0xA000));
    let external_ram_switcher = Some(Switcher::new(0xD000, 0xE000));
    let work_ram_switcher = Some(Switcher::new(0xD000, 0xE000));

    banks.push(Bank::new("cartridge-fixed".to_string(), 0x0000, 0x3FFF, None));
    banks.push(Bank::new("cartridge-switchable".to_string(), 0x4000, 0x7FFF, cartridge_switcher));
    banks.push(Bank::new("video".to_string(), 0x8000, 0x9FFF, video_switcher));
    banks.push(Bank::new("external-ram".to_string(), 0xA000, 0xBFFF, external_ram_switcher));
    banks.push(Bank::new("work-ram".to_string(), 0xC000, 0xCFFF, None));
    banks.push(Bank::new("work-ram-switchable".to_string(), 0xD000, 0xDFFF, work_ram_switcher));
    banks.push(Bank::new("echo-ram".to_string(), 0xE000, 0xFDFF, None));
    banks.push(Bank::new("object-attribute-memory".to_string(), 0xFE00, 0xFE9F, None));
    banks.push(Bank::new("unusable".to_string(), 0xFEA0, 0xFEFF, None));
    banks.push(Bank::new("hardware-io-registers".to_string(), 0xFF00, 0xFF7F, None));
    banks.push(Bank::new("high-ram".to_string(), 0xFF80, 0xFFFE, None));
    banks.push(Bank::new("interrupt-enable-register".to_string(), 0xFFFF, 0xFFFF, None));
}

mod test {

    #[test]
    fn test_init_banks() {
        use super::init_banks;
        use super::Bank;
        let mut banks: Vec<Bank> = Vec::new();
        init_banks(&mut banks);
        assert_eq!(banks.len(), 12);
    }
}
