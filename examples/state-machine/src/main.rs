enum VendingMachineState {
    Idle,
    SelectingItem,
    DispensingItem,
    RefundingCoins,
    EndState,
}

fn update_state(current_state: VendingMachineState) -> VendingMachineState {
    match current_state {
        VendingMachineState::Idle => VendingMachineState::SelectingItem,
        VendingMachineState::SelectingItem => VendingMachineState::DispensingItem,
        VendingMachineState::DispensingItem => VendingMachineState::RefundingCoins,
        VendingMachineState::RefundingCoins => VendingMachineState::EndState,
        VendingMachineState::EndState => VendingMachineState::EndState,
    }
}

fn main() {
    let mut current_state = VendingMachineState::Idle;
    loop {
        match current_state {
            VendingMachineState::Idle => {
                println!("Vending machine is idle.");
            }
            VendingMachineState::SelectingItem => {
                println!("Selecting item...");
            }
            VendingMachineState::DispensingItem => {
                println!("Dispensing item...");
            }
            VendingMachineState::RefundingCoins => {
                println!("Refunding coins...");
            }
            VendingMachineState::EndState => {
                println!("Vending machine is done.");
                break;
            }
        }
        current_state = update_state(current_state);
    }
}
