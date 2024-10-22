use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    fn set_balance(&mut self, who: &str, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    fn balance(&self, who: &str) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: &str,
        to: &str,
        amount: u128,
    ) -> Result<(), &'static str> {
        
        /* TODO:
            - Obter o saldo da conta `caller`.
            - Obter o saldo da conta `to`.
            - Usar matemática segura para calcular um `new_caller_balance`.
            - Usar matemática segura para calcular um `new_to_balance`.
            - Inserir o novo saldo de `caller`.
            - Inserir o novo saldo de `to`.
        */

        let balance_caller = self.balance(&caller);
        let balance_to = self.balance(&to);

        let new_caller_balance = balance_caller.checked_sub(amount).ok_or("Error")? ;
        let new_to_balance = balance_to.checked_add(amount).ok_or("Error")? ;

        self.set_balance( caller , new_caller_balance );
        self.set_balance( to , new_to_balance );


        Ok(())
    }
}

#[test]
fn init_balances() {
    let mut balances = Pallet::new();

    assert_eq!(balances.balance("alice"), 0);
    balances.set_balance("alice", 100);
    assert_eq!(balances.balance("alice"), 100);
    assert_eq!(balances.balance("bob"), 0);
}

#[test]
fn transfer_balance() { 
    /* TODO: Crie um teste que verifique o seguinte:
    - Que `alice` não pode transferir fundos que ela não possui.
    - Que `alice` pode transferir fundos para `bob` com sucesso.
    - Que o saldo de `alice` e `bob` esteja atualizado corretamente.
    */

    let mut balances = Pallet::new();

    assert!(balances.transfer("alice", "bob", 10).is_err());
    balances.set_balance("alice", 100);
    assert!(balances.transfer("alice", "bob", 10).is_ok());

    assert_eq!(balances.balance("alice"), 90);
    assert_eq!(balances.balance("bob"), 10);
    
}
