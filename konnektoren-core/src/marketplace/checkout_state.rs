use super::{Cart, Payment};

#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutError {
    EmptyCart,
    PaymentFailed,
    InsufficientFunds,
    IllegalState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutState {
    Cart(Cart),
    Billing(Cart),
    Payment(Cart, Payment),
    Complete(Cart),
}

impl CheckoutState {
    pub fn new(cart: Cart) -> Self {
        CheckoutState::Cart(cart)
    }

    pub fn cancel(&self) -> Result<Self, CheckoutError> {
        match self {
            CheckoutState::Billing(cart) => Ok(CheckoutState::Cart(cart.clone())),
            CheckoutState::Payment(cart, _) => Ok(CheckoutState::Cart(cart.clone())),
            _ => Err(CheckoutError::IllegalState),
        }
    }

    pub fn show_billing(&self) -> Result<Self, CheckoutError> {
        match self {
            CheckoutState::Cart(cart) => Ok(CheckoutState::Billing(cart.clone())),
            _ => Err(CheckoutError::IllegalState),
        }
    }

    pub fn show_payment(&self, payment: Payment) -> Result<Self, CheckoutError> {
        match self {
            CheckoutState::Billing(cart) => Ok(CheckoutState::Payment(cart.clone(), payment)),
            _ => Err(CheckoutError::IllegalState),
        }
    }

    pub fn complete(&self) -> Result<Self, CheckoutError> {
        match self {
            CheckoutState::Payment(cart, _) => Ok(CheckoutState::Complete(cart.clone())),
            _ => Err(CheckoutError::IllegalState),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_checkout_state() {
        let cart = Cart::new();
        let checkout_state = CheckoutState::new(cart.clone());
        assert_eq!(checkout_state, CheckoutState::Cart(cart));
    }

    #[test]
    fn test_cancel() {
        let cart = Cart::new();
        let mut checkout_state = CheckoutState::new(cart.clone());
        checkout_state = checkout_state.show_billing().unwrap();
        checkout_state = checkout_state.cancel().unwrap();
        assert_eq!(checkout_state, CheckoutState::Cart(cart));
    }

    #[test]
    fn test_show_billing() {
        let cart = Cart::new();
        let mut checkout_state = CheckoutState::new(cart.clone());
        checkout_state = checkout_state.show_billing().unwrap();
        assert_eq!(checkout_state, CheckoutState::Billing(cart));
    }

    #[test]
    fn test_show_payment() {
        let cart = Cart::new();
        let payment = Payment {
            method: "cash".to_string(),
            amount: 0.0,
        };
        let mut checkout_state = CheckoutState::new(cart.clone());
        checkout_state = checkout_state.show_billing().unwrap();
        checkout_state = checkout_state.show_payment(payment.clone()).unwrap();
        assert_eq!(checkout_state, CheckoutState::Payment(cart, payment));
    }

    #[test]
    fn test_complete() {
        let cart = Cart::new();
        let payment = Payment {
            method: "cash".to_string(),
            amount: 0.0,
        };
        let mut checkout_state = CheckoutState::new(cart.clone());
        checkout_state = checkout_state.show_billing().unwrap();
        checkout_state = checkout_state.show_payment(payment.clone()).unwrap();
        checkout_state = checkout_state.complete().unwrap();
        assert_eq!(checkout_state, CheckoutState::Complete(cart));
    }

    #[test]
    fn test_cancel_illegal_state() {
        let cart = Cart::new();
        let mut checkout_state = CheckoutState::new(cart.clone());
        checkout_state = checkout_state.show_billing().unwrap();
        checkout_state = checkout_state.show_payment(Payment::default()).unwrap();
        checkout_state = checkout_state.complete().unwrap();
        let cancel_result = checkout_state.cancel();
        assert!(cancel_result.is_err());
        assert_eq!(cancel_result.unwrap_err(), CheckoutError::IllegalState);
    }
}
