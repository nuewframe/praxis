pub trait CheckoutPort {
    fn process_order(&self, order: OrderPayload) -> OrderResult;
    fn cancel_order(&self, order_id: String) -> bool;
}
