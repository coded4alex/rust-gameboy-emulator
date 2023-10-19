use crate::memory::router::Bank;


pub fn init_banks() -> Vec<Bank> {
    let mut banks: Vec<Bank> = Vec::new();

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

    banks
}


mod test {
    #[test]
    fn test_init_banks() {
        use super::init_banks;
        let banks = init_banks();
        assert_eq!(banks.len(), 12);
        assert_eq!(banks[0].name, "cartridge-fixed");
        assert_eq!(banks[1].name, "cartridge-switchable");
        assert_eq!(banks[2].name, "video");
        assert_eq!(banks[3].name, "external-ram");
        assert_eq!(banks[4].name, "external-ram");
        assert_eq!(banks[5].name, "external-ram-switchable");
        assert_eq!(banks[6].name, "echo-ram");
        assert_eq!(banks[7].name, "object-attribute-memory");
        assert_eq!(banks[8].name, "unusable");
        assert_eq!(banks[9].name, "hardware-io-registers");
        assert_eq!(banks[10].name, "high-ram");
        assert_eq!(banks[11].name, "interrupt-enable-register");
    }
}
