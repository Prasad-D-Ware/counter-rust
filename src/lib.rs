use borsh::{ BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{ next_account_info, AccountInfo }, entrypoint::{entrypoint, ProgramResult}, msg, pubkey::Pubkey
};

entrypoint!(counter_initialize);

#[derive(BorshDeserialize,BorshSerialize)]
enum Instruction{
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize,BorshSerialize)]
struct Counter{
    count : u32
}

pub fn counter_initialize(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
)-> ProgramResult {

    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = Instruction::try_from_slice(instruction_data)?;

    let mut counter = Counter::try_from_slice(*acc.data.borrow())?;

    match instruction_type {
        Instruction::Increment(val)=>{
            counter.count += val;
        },
        Instruction::Decrement(val) => {
            counter.count -= val;
        }
    };

    counter.serialize(&mut *acc.data.borrow_mut());

    msg!("Counter Successfully Executed");

    Ok(())

}