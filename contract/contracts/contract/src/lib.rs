#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct TravelBookingContract;

#[contractimpl]
impl TravelBookingContract {
    // Book a trip
    pub fn book_trip(env: Env, user: Address, destination: Symbol, date: Symbol) {
        env.storage().persistent().set(&user, &(destination, date));
    }

    // Retrieve a booking
    pub fn get_booking(env: Env, user: Address) -> Option<(Symbol, Symbol)> {
        env.storage().persistent().get(&user)
    }

    // Cancel a booking
    pub fn cancel_booking(env: Env, user: Address) {
        env.storage().persistent().remove(&user);
    }
}
