use std::collections::HashMap;


pub struct Bank {
    pub name: String,
    pub lower_bound: u16,
    pub upper_bound: u16,
    pub switchable: bool,
    pub current: u16,
    pub banks: HashMap<u16, Vec<u8>>,
}


impl Bank {
    pub fn new(name: String, lower_bound: u16, upper_bound: u16, switchable: bool) -> Bank {
        Bank {
            name,
            lower_bound,
            upper_bound,
            current: 0,
            switchable,
            banks: HashMap::new(),
        }
    }
}


pub fn create_banks() -> Vec<Bank> {
    Vec::new()
}


pub fn init_banks(banks: &mut Vec<Bank>) {

    banks.push(Bank::new("cartridge-fixed".to_string(), 0x0000, 0x3FFF, false));
    banks.push(Bank::new("cartridge-switchable".to_string(), 0x4000, 0x7FFF, true));
    banks.push(Bank::new("video".to_string(), 0x8000, 0x9FFF, false));
    banks.push(Bank::new("external-ram".to_string(), 0xA000, 0xBFFF, true));
    banks.push(Bank::new("external-ram".to_string(), 0xC000, 0xCFFF, false));
    banks.push(Bank::new("external-ram-switchable".to_string(), 0xD000, 0xDFFF, true));
    banks.push(Bank::new("echo-ram".to_string(), 0xE000, 0xFDFF, false));
    banks.push(Bank::new("object-attribute-memory".to_string(), 0xFE00, 0xFE9F, false));
    banks.push(Bank::new("unusable".to_string(), 0xFEA0, 0xFEFF, false));
    banks.push(Bank::new("hardware-io-registers".to_string(), 0xFF00, 0xFF7F, false));
    banks.push(Bank::new("high-ram".to_string(), 0xFF80, 0xFFFE, false));
    banks.push(Bank::new("interrupt-enable-register".to_string(), 0xFFFF, 0xFFFF, false));
}


mod test {
    #[test]
    fn test_bank() {
        use super::Bank;
        let bank = Bank::new("test".to_string(), 0x0000, 0x3FFF, false);
        assert_eq!(bank.name, "test");
        assert_eq!(bank.lower_bound, 0x0000);
        assert_eq!(bank.upper_bound, 0x3FFF);
        assert_eq!(bank.switchable, false);
        assert_eq!(bank.current, 0);
    }

    #[test]
    fn test_init_banks() {
        use super::Bank;
        use super::init_banks;
        let mut banks: Vec<Bank> = Vec::new();
        init_banks(&mut banks);
        assert_eq!(banks.len(), 12);}

}
