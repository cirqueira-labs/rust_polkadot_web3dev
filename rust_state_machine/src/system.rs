use std::collections::BTreeMap;

/// Este é o Pallet do Sistema.
/// Ele lida com o estado de baixo nível necessário para seu blockchain.
pub struct Pallet {
    /// O número do bloco atual.
    /* TODO: Crie um campo `block_number` que armazena um `u32`. */
    /// Um mapa de uma conta para seu nonce.
    /* TODO: Crie um campo `nonce` que seja um `BTreeMap` de `String` para `u32`. */

    block_number: u32,
    nonce: BTreeMap<String,u32>
}

impl Pallet {
    /// Cria uma nova instância do System Pallet.
    pub fn new() -> Self {
        /* TODO: Retorne uma nova instância da struct `Pallet`. */
    
        Self { 
            block_number : 0,
            nonce : BTreeMap::new()
        }

    }

    /// Obtém o número atual do bloco.
    pub fn block_number(&self) -> u32 {
        /* TODO: Retorne o número do bloco atual. */
        self.block_number
    }

    /// Esta função pode ser usada para incrementar o número do bloco.
    /// Aumenta o número do bloco em um.
    pub fn inc_block_number(&mut self) {
        /* TODO: Aumenta o número do bloco atual em um. */
        self.block_number = self.block_number.checked_add(1).unwrap_or(0);
    }

    /// Incrementa o nonce de uma conta. Isso nos ajuda a acompanhar quantas transações cada conta fez.
    pub fn inc_nonce(&mut self, who: &str) {
        /* TODO: Obtenha o nonce atual de `who` e aumente-o em um. */
        self.nonce.insert(who.to_string(), self.nonce.get(who).unwrap_or(&0).checked_add(1).unwrap_or(1) );
        
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    #[test]
    fn init_system() {
        /* TODO: Crie um teste que verifique o seguinte:
            - Aumenta o número do bloco atual.
            - Aumente o nonce de `alice`.
            - Verifique se o número do bloco é o que esperamos.
            - Verifique se o nonce de `alice` é o que esperamos.
        */
        
        let mut pallet = Pallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number,1);
        pallet.inc_nonce("alice");
        assert_eq!(pallet.nonce.get("alice").ok_or(0),Ok(&1));

    }
}
