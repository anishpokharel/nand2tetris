use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_variable: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        // Predefined symbols
        for (i, symbol) in [
            "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9",
            "R10", "R11", "R12", "R13", "R14", "R15",
            "SCREEN", "KBD", "SP", "LCL", "ARG", "THIS", "THAT", "LOOP", "STOP", "i", "sum"
        ].iter().enumerate() {
            table.insert(symbol.to_string(), i as u16);
        }
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);
        table.insert("LOOP".to_string(), 4);
        table.insert("STOP".to_string(), 18);
        table.insert("i".to_string(), 16);
        table.insert("sum".to_string(), 17);

        SymbolTable {
            table: table,
            next_variable: 16, // Starting point for user-defined variables
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<u16> {
        self.table.get(symbol).cloned()
    }

    pub fn add_variable(&mut self, symbol: String) -> u16 {
        let address = self.next_variable;
        self.table.insert(symbol, address);
        self.next_variable += 1;
        address
    }
}