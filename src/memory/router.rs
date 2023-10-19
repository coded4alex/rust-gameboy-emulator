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
    pub fn new(name: String, lower: u16, upper: u16, switchable: bool) -> Bank {
        Bank {
            name: name,
            lower_bound: lower,
            upper_bound: upper,
            current: 0,
            switchable: switchable,
            banks: HashMap::new(),
        }
    }
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
}
